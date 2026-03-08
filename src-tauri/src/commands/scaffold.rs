use crate::models::project::{ProjectEntry, ProjectRegistry};
use crate::TwelvetyError;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Deserialize)]
pub struct ScaffoldOptions {
    pub name: String,
    pub directory: PathBuf,
    pub starter: String,
    pub template_lang: String,
    pub css: String,
    pub author_name: String,
    pub author_url: String,
    pub site_url: String,
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), TwelvetyError> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn customize_site_data(project_dir: &Path, opts: &ScaffoldOptions) -> Result<(), TwelvetyError> {
    let site_json_path = project_dir.join("src/_data/site.json");
    if site_json_path.exists() {
        let mut data: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&site_json_path)?)?;

        if let Some(obj) = data.as_object_mut() {
            obj.insert("title".into(), serde_json::json!(opts.name));
            obj.insert("url".into(), serde_json::json!(opts.site_url));
            if let Some(author) = obj.get_mut("author").and_then(|a| a.as_object_mut()) {
                author.insert("name".into(), serde_json::json!(opts.author_name));
                author.insert("url".into(), serde_json::json!(opts.author_url));
            }
        }

        std::fs::write(&site_json_path, serde_json::to_string_pretty(&data)?)?;
    }
    Ok(())
}

fn write_twelvety_config(project_dir: &Path, opts: &ScaffoldOptions) -> Result<(), TwelvetyError> {
    let config = format!(
        r#"export default {{
  name: "{}",
  eleventyVersion: "3.0",
  templateLang: "{}",
  css: "{}",
  deploy: {{
    target: "github-pages",
  }},
  indieweb: {{
    domain: "{}",
    author: {{
      name: "{}",
      url: "{}",
    }},
    micropub: false,
    webmention: false,
  }},
}};
"#,
        opts.name,
        opts.template_lang,
        opts.css,
        opts.site_url.replace("https://", "").replace("http://", ""),
        opts.author_name,
        opts.author_url,
    );
    std::fs::write(project_dir.join("twelvety.config.js"), config)?;
    Ok(())
}

#[tauri::command]
pub async fn scaffold_project(
    app: AppHandle,
    options: ScaffoldOptions,
) -> Result<ProjectEntry, TwelvetyError> {
    let template_dir = app
        .path()
        .resource_dir()
        .unwrap()
        .join("templates")
        .join(&options.starter);

    if !template_dir.exists() {
        return Err(TwelvetyError {
            code: "TEMPLATE_NOT_FOUND".to_string(),
            message: format!("Starter template '{}' not found", options.starter),
        });
    }

    copy_dir_recursive(&template_dir, &options.directory)?;
    customize_site_data(&options.directory, &options)?;
    write_twelvety_config(&options.directory, &options)?;

    let entry = ProjectEntry {
        id: uuid::Uuid::new_v4().to_string(),
        name: options.name.clone(),
        path: options.directory.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
        template_lang: options.template_lang.clone(),
        css_approach: options.css.clone(),
    };

    let registry_path = app.path().app_data_dir().unwrap().join("projects.json");
    let mut registry = ProjectRegistry::load(&registry_path)?;
    registry.add(entry.clone());
    registry.save(&registry_path)?;

    Ok(entry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_dir_recursive() {
        let src = tempfile::tempdir().unwrap();
        let dst = tempfile::tempdir().unwrap();
        let dst_path = dst.path().join("output");

        std::fs::write(src.path().join("file.txt"), "hello").unwrap();
        std::fs::create_dir(src.path().join("sub")).unwrap();
        std::fs::write(src.path().join("sub/nested.txt"), "world").unwrap();

        copy_dir_recursive(src.path(), &dst_path).unwrap();

        assert!(dst_path.join("file.txt").exists());
        assert!(dst_path.join("sub/nested.txt").exists());
        assert_eq!(
            std::fs::read_to_string(dst_path.join("file.txt")).unwrap(),
            "hello"
        );
    }

    #[test]
    fn test_customize_site_data() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src/_data")).unwrap();
        std::fs::write(
            dir.path().join("src/_data/site.json"),
            r#"{"title":"placeholder","url":"https://example.com","author":{"name":"placeholder","url":"https://example.com"}}"#,
        )
        .unwrap();

        let opts = ScaffoldOptions {
            name: "Cool Blog".to_string(),
            directory: dir.path().to_path_buf(),
            starter: "blog".to_string(),
            template_lang: "nunjucks".to_string(),
            css: "vanilla".to_string(),
            author_name: "Alice".to_string(),
            author_url: "https://alice.example".to_string(),
            site_url: "https://alice.example".to_string(),
        };

        customize_site_data(dir.path(), &opts).unwrap();

        let data: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(dir.path().join("src/_data/site.json")).unwrap(),
        )
        .unwrap();

        assert_eq!(data["title"], "Cool Blog");
        assert_eq!(data["author"]["name"], "Alice");
    }
}
