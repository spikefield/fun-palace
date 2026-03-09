use crate::FunPalaceError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

pub struct ProcessManager {
    processes: Mutex<HashMap<String, Child>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Mutex::new(HashMap::new()),
        }
    }

    pub fn serve(
        &self,
        project_id: &str,
        project_dir: &PathBuf,
        eleventy_path: &str,
    ) -> Result<u32, FunPalaceError> {
        let mut procs = self.processes.lock().unwrap();

        if let Some(mut child) = procs.remove(project_id) {
            let _ = child.kill();
        }

        let child = Command::new(eleventy_path)
            .args(["@11ty/eleventy", "--serve"])
            .current_dir(project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| FunPalaceError {
                code: "PROCESS_SPAWN_ERROR".to_string(),
                message: format!("Failed to start Eleventy: {}", e),
            })?;

        let pid = child.id();
        procs.insert(project_id.to_string(), child);
        Ok(pid)
    }

    pub fn build(
        &self,
        project_dir: &PathBuf,
        eleventy_path: &str,
    ) -> Result<String, FunPalaceError> {
        let output = Command::new(eleventy_path)
            .args(["@11ty/eleventy"])
            .current_dir(project_dir)
            .output()
            .map_err(|e| FunPalaceError {
                code: "BUILD_ERROR".to_string(),
                message: format!("Failed to run Eleventy build: {}", e),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FunPalaceError {
                code: "BUILD_FAILED".to_string(),
                message: format!("Eleventy build failed:\n{}", stderr),
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }

    pub fn stop(&self, project_id: &str) -> Result<(), FunPalaceError> {
        let mut procs = self.processes.lock().unwrap();
        if let Some(mut child) = procs.remove(project_id) {
            child.kill().map_err(|e| FunPalaceError {
                code: "PROCESS_KILL_ERROR".to_string(),
                message: format!("Failed to stop process: {}", e),
            })?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn stop_all(&self) {
        let mut procs = self.processes.lock().unwrap();
        for (_, mut child) in procs.drain() {
            let _ = child.kill();
        }
    }

    pub fn is_running(&self, project_id: &str) -> bool {
        let procs = self.processes.lock().unwrap();
        procs.contains_key(project_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_is_running_false_initially() {
        let pm = ProcessManager::new();
        assert!(!pm.is_running("test-project"));
    }

    #[test]
    fn test_process_manager_stop_nonexistent_is_ok() {
        let pm = ProcessManager::new();
        assert!(pm.stop("nonexistent").is_ok());
    }

    #[test]
    fn test_process_manager_stop_all_empty() {
        let pm = ProcessManager::new();
        pm.stop_all();
    }
}
