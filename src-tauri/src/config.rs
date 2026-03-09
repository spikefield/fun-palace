#![allow(dead_code)]

use crate::FunPalaceError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunPalaceConfig {
    pub name: Option<String>,
    #[serde(rename = "eleventyVersion")]
    pub eleventy_version: Option<String>,
    #[serde(rename = "templateLang")]
    pub template_lang: Option<String>,
    pub css: Option<String>,
    pub deploy: Option<DeployConfig>,
    pub indieweb: Option<IndiewebConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeployConfig {
    pub target: Option<String>,
    #[serde(rename = "siteId")]
    pub site_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IndiewebConfig {
    pub domain: Option<String>,
    pub author: Option<AuthorConfig>,
    pub micropub: Option<bool>,
    pub webmention: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorConfig {
    pub name: Option<String>,
    pub url: Option<String>,
    pub photo: Option<String>,
}

/// Find the config file path in a project directory.
/// Resolution order: funpalace.config.js -> .funpalace.js
pub fn find_config_file(project_dir: &Path) -> Option<PathBuf> {
    let candidates = ["funpalace.config.js", ".funpalace.js"];
    for name in &candidates {
        let path = project_dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Read the config by evaluating the JS file with Node.
pub fn read_config(project_dir: &Path, node_path: &str) -> Result<FunPalaceConfig, FunPalaceError> {
    let config_path = find_config_file(project_dir).ok_or_else(|| FunPalaceError {
        code: "CONFIG_NOT_FOUND".to_string(),
        message: format!(
            "No funpalace.config.js or .funpalace.js found in {}",
            project_dir.display()
        ),
    })?;

    let script = format!(
        r#"import('{url}').then(m => console.log(JSON.stringify(m.default || m)))"#,
        url = config_path.display().to_string().replace('\\', "/")
    );

    let output = std::process::Command::new(node_path)
        .arg("--input-type=module")
        .arg("-e")
        .arg(&script)
        .current_dir(project_dir)
        .output()
        .map_err(|e| FunPalaceError {
            code: "NODE_EXEC_ERROR".to_string(),
            message: format!("Failed to run Node: {}", e),
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(FunPalaceError {
            code: "CONFIG_EVAL_ERROR".to_string(),
            message: format!("Failed to evaluate config: {}", stderr),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let config: FunPalaceConfig = serde_json::from_str(stdout.trim())?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_config_prefers_funpalace_config_js() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("funpalace.config.js"), "").unwrap();
        fs::write(dir.path().join(".funpalace.js"), "").unwrap();

        let found = find_config_file(dir.path()).unwrap();
        assert!(found.ends_with("funpalace.config.js"));
    }

    #[test]
    fn test_find_config_falls_back_to_dotfile() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(".funpalace.js"), "").unwrap();

        let found = find_config_file(dir.path()).unwrap();
        assert!(found.ends_with(".funpalace.js"));
    }

    #[test]
    fn test_find_config_returns_none_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(find_config_file(dir.path()).is_none());
    }

    #[test]
    fn test_read_config_parses_js_export() {
        if std::process::Command::new("node")
            .arg("--version")
            .output()
            .is_err()
        {
            eprintln!("Skipping: Node not available");
            return;
        }

        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("funpalace.config.js"),
            r#"export default { name: "Test Site", templateLang: "nunjucks" };"#,
        )
        .unwrap();

        let config = read_config(dir.path(), "node").unwrap();
        assert_eq!(config.name.unwrap(), "Test Site");
        assert_eq!(config.template_lang.unwrap(), "nunjucks");
    }
}
