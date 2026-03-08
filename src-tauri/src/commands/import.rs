use crate::TwelvetyError;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct DetectionResult {
    pub is_eleventy: bool,
    pub config_file: Option<String>,
    pub template_lang: Option<String>,
    pub post_count: usize,
    pub has_indieweb_markup: bool,
    pub has_twelvety_config: bool,
    pub has_feeds: bool,
}

fn detect_template_lang(dir: &Path) -> Option<String> {
    let extensions = [("njk", "nunjucks"), ("liquid", "liquid"), ("webc", "webc")];
    for (ext, lang) in &extensions {
        if has_files_with_ext(dir, ext) {
            return Some(lang.to_string());
        }
    }
    None
}

fn has_files_with_ext(dir: &Path, ext: &str) -> bool {
    fn walk(dir: &Path, ext: &str) -> bool {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if walk(&path, ext) {
                        return true;
                    }
                } else if path.extension().is_some_and(|e| e == ext) {
                    return true;
                }
            }
        }
        false
    }
    walk(dir, ext)
}

fn count_markdown_files(dir: &Path) -> usize {
    fn walk(dir: &Path, count: &mut usize) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, count);
                } else if path.extension().is_some_and(|e| e == "md") {
                    *count += 1;
                }
            }
        }
    }
    let mut count = 0;
    walk(dir, &mut count);
    count
}

fn check_indieweb_markup(dir: &Path) -> bool {
    fn walk(dir: &Path) -> bool {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if walk(&path) {
                        return true;
                    }
                } else if let Ok(content) = std::fs::read_to_string(&path) {
                    if content.contains("h-card")
                        || content.contains("h-entry")
                        || content.contains("h-feed")
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
    walk(dir)
}

pub fn detect_project(dir: &Path) -> DetectionResult {
    let config_candidates = [
        "eleventy.config.js",
        "eleventy.config.mjs",
        "eleventy.config.cjs",
        ".eleventy.js",
    ];
    let config_file = config_candidates
        .iter()
        .find(|f| dir.join(f).exists())
        .map(|f| f.to_string());

    let is_eleventy = config_file.is_some()
        || dir.join("package.json").exists()
            && std::fs::read_to_string(dir.join("package.json"))
                .unwrap_or_default()
                .contains("@11ty/eleventy");

    let has_twelvety = crate::config::find_config_file(dir).is_some();
    let has_feeds = dir.join("_site/feed.xml").exists()
        || dir.join("_site/feed.json").exists()
        || has_files_with_ext(dir, "xml");

    DetectionResult {
        is_eleventy,
        config_file,
        template_lang: detect_template_lang(dir),
        post_count: count_markdown_files(dir),
        has_indieweb_markup: check_indieweb_markup(dir),
        has_twelvety_config: has_twelvety,
        has_feeds,
    }
}

#[tauri::command]
pub async fn detect_eleventy_project(directory: String) -> Result<DetectionResult, TwelvetyError> {
    let path = PathBuf::from(&directory);
    if !path.exists() {
        return Err(TwelvetyError {
            code: "DIR_NOT_FOUND".to_string(),
            message: format!("Directory not found: {}", directory),
        });
    }
    Ok(detect_project(&path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = detect_project(dir.path());
        assert!(!result.is_eleventy);
        assert_eq!(result.post_count, 0);
    }

    #[test]
    fn test_detect_eleventy_by_config() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("eleventy.config.js"), "module.exports = {}").unwrap();
        let result = detect_project(dir.path());
        assert!(result.is_eleventy);
        assert_eq!(result.config_file, Some("eleventy.config.js".to_string()));
    }

    #[test]
    fn test_detect_template_lang_and_posts() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("eleventy.config.js"), "").unwrap();
        std::fs::create_dir(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("src/index.njk"), "").unwrap();
        std::fs::write(dir.path().join("src/post.md"), "# Hello").unwrap();
        let result = detect_project(dir.path());
        assert_eq!(result.template_lang, Some("nunjucks".to_string()));
        assert_eq!(result.post_count, 1);
    }

    #[test]
    fn test_detect_indieweb_markup() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("layout.njk"), "<div class=\"h-card\">").unwrap();
        let result = detect_project(dir.path());
        assert!(result.has_indieweb_markup);
    }
}
