use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEntry {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub created_at: String,
    pub template_lang: String,
    pub css_approach: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectRegistry {
    pub projects: Vec<ProjectEntry>,
}

impl ProjectRegistry {
    pub fn load(path: &std::path::Path) -> Result<Self, crate::TwelvetyError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(path)?;
        let registry: ProjectRegistry = serde_json::from_str(&data)?;
        Ok(registry)
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), crate::TwelvetyError> {
        let data = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn add(&mut self, entry: ProjectEntry) {
        self.projects.push(entry);
    }

    pub fn remove(&mut self, id: &str) {
        self.projects.retain(|p| p.id != id);
    }

    pub fn find(&self, id: &str) -> Option<&ProjectEntry> {
        self.projects.iter().find(|p| p.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_registry_load_empty() {
        let path = Path::new("/tmp/twelvety_test_nonexistent.json");
        let registry = ProjectRegistry::load(path).unwrap();
        assert!(registry.projects.is_empty());
    }

    #[test]
    fn test_registry_add_and_find() {
        let mut registry = ProjectRegistry::default();
        let entry = ProjectEntry {
            id: "test-1".to_string(),
            name: "Test Project".to_string(),
            path: PathBuf::from("/tmp/test"),
            created_at: "2026-03-08".to_string(),
            template_lang: "nunjucks".to_string(),
            css_approach: "vanilla".to_string(),
        };
        registry.add(entry);
        assert_eq!(registry.projects.len(), 1);
        assert!(registry.find("test-1").is_some());
        assert!(registry.find("nonexistent").is_none());
    }

    #[test]
    fn test_registry_remove() {
        let mut registry = ProjectRegistry::default();
        registry.add(ProjectEntry {
            id: "rm-1".to_string(),
            name: "Remove Me".to_string(),
            path: PathBuf::from("/tmp/rm"),
            created_at: "2026-03-08".to_string(),
            template_lang: "nunjucks".to_string(),
            css_approach: "vanilla".to_string(),
        });
        registry.remove("rm-1");
        assert!(registry.projects.is_empty());
    }

    #[test]
    fn test_registry_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("projects.json");

        let mut registry = ProjectRegistry::default();
        registry.add(ProjectEntry {
            id: "save-1".to_string(),
            name: "Saved".to_string(),
            path: PathBuf::from("/tmp/saved"),
            created_at: "2026-03-08".to_string(),
            template_lang: "nunjucks".to_string(),
            css_approach: "vanilla".to_string(),
        });
        registry.save(&path).unwrap();

        let loaded = ProjectRegistry::load(&path).unwrap();
        assert_eq!(loaded.projects.len(), 1);
        assert_eq!(loaded.projects[0].name, "Saved");
    }
}
