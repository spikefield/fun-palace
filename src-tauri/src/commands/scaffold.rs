use crate::models::project::{ProjectEntry, ProjectRegistry};
use crate::FunPalaceError;
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

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), FunPalaceError> {
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

fn customize_site_data(project_dir: &Path, opts: &ScaffoldOptions) -> Result<(), FunPalaceError> {
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

fn customize_eleventy_config(
    project_dir: &Path,
    opts: &ScaffoldOptions,
) -> Result<(), FunPalaceError> {
    let config_path = project_dir.join("eleventy.config.js");
    if !config_path.exists() {
        return Ok(());
    }

    let (formats, engine) = match opts.template_lang.as_str() {
        "liquid" => (r#"["md", "liquid", "html"]"#, r#""liquid""#),
        "webc" => (r#"["md", "webc", "html"]"#, r#""webc""#),
        "jsx" => (r#"["md", "11ty.jsx", "html"]"#, r#""11ty.jsx""#),
        "mdx" => (r#"["md", "mdx", "html"]"#, r#""mdx""#),
        "typescript" => (r#"["md", "11ty.ts", "html"]"#, r#""11ty.ts""#),
        "handlebars" => (r#"["md", "hbs", "html"]"#, r#""hbs""#),
        "pug" => (r#"["md", "pug", "html"]"#, r#""pug""#),
        "mustache" => (r#"["md", "mustache", "html"]"#, r#""mustache""#),
        "ejs" => (r#"["md", "ejs", "html"]"#, r#""ejs""#),
        "haml" => (r#"["md", "haml", "html"]"#, r#""haml""#),
        _ => (r#"["md", "njk", "html"]"#, r#""njk""#),
    };

    let content = std::fs::read_to_string(&config_path)?;
    let updated = content
        .replace(r#"["md", "njk", "html"]"#, formats)
        .replace(r#"markdownTemplateEngine: "njk""#, &format!("markdownTemplateEngine: {engine}"))
        .replace(r#"htmlTemplateEngine: "njk""#, &format!("htmlTemplateEngine: {engine}"))
        .replace(r#"title: "My Site""#, &format!(r#"title: "{}""#, opts.name))
        .replace(r#"url: "https://example.com""#, &format!(r#"url: "{}""#, opts.site_url))
        .replace(r#"author: "Your Name""#, &format!(r#"author: "{}""#, opts.author_name));

    std::fs::write(&config_path, updated)?;
    Ok(())
}

fn rename_template_extensions(
    project_dir: &Path,
    opts: &ScaffoldOptions,
) -> Result<(), FunPalaceError> {
    let ext = match opts.template_lang.as_str() {
        "nunjucks" => return Ok(()),
        "liquid" => "liquid",
        "webc" => "webc",
        "jsx" => "11ty.jsx",
        "mdx" => "mdx",
        "typescript" => "11ty.ts",
        "handlebars" => "hbs",
        "pug" => "pug",
        "mustache" => "mustache",
        "ejs" => "ejs",
        "haml" => "haml",
        _ => return Ok(()),
    };

    rename_njk_files_recursive(&project_dir.join("src"), ext)
}

fn rename_njk_files_recursive(dir: &Path, new_ext: &str) -> Result<(), FunPalaceError> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            rename_njk_files_recursive(&path, new_ext)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("njk") {
            let new_path = path.with_extension(new_ext);
            std::fs::rename(&path, &new_path)?;
        }
    }
    Ok(())
}

fn write_funpalace_config(project_dir: &Path, opts: &ScaffoldOptions) -> Result<(), FunPalaceError> {
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
    std::fs::write(project_dir.join("funpalace.config.js"), config)?;
    Ok(())
}

#[tauri::command]
pub async fn scaffold_project(
    app: AppHandle,
    options: ScaffoldOptions,
) -> Result<ProjectEntry, FunPalaceError> {
    let resource_path = app
        .path()
        .resource_dir()
        .unwrap()
        .join("templates")
        .join(&options.starter);

    // In dev mode, resource_dir doesn't contain templates — fall back to source tree
    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("templates")
        .join(&options.starter);

    let template_dir = if resource_path.exists() {
        resource_path
    } else if dev_path.exists() {
        dev_path
    } else {
        return Err(FunPalaceError {
            code: "TEMPLATE_NOT_FOUND".to_string(),
            message: format!("Starter template '{}' not found", options.starter),
        });
    };

    copy_dir_recursive(&template_dir, &options.directory)?;
    customize_site_data(&options.directory, &options)?;
    customize_eleventy_config(&options.directory, &options)?;
    rename_template_extensions(&options.directory, &options)?;
    write_funpalace_config(&options.directory, &options)?;

    // Install dependencies
    let install = std::process::Command::new("npm")
        .arg("install")
        .current_dir(&options.directory)
        .output()
        .map_err(|e| FunPalaceError {
            code: "NPM_INSTALL_ERROR".to_string(),
            message: format!("Failed to run npm install: {}", e),
        })?;
    if !install.status.success() {
        let stderr = String::from_utf8_lossy(&install.stderr);
        return Err(FunPalaceError {
            code: "NPM_INSTALL_FAILED".to_string(),
            message: format!("npm install failed:\n{}", stderr),
        });
    }

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

    #[test]
    fn test_customize_eleventy_config_webc() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("eleventy.config.js"),
            r#"  const siteData = {
    title: "My Site",
    url: "https://example.com",
    author: "Your Name",
  };
  return {
    templateFormats: ["md", "njk", "html"],
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
  };"#,
        )
        .unwrap();

        let opts = ScaffoldOptions {
            name: "Alice's Garden".to_string(),
            directory: dir.path().to_path_buf(),
            starter: "blog".to_string(),
            template_lang: "webc".to_string(),
            css: "vanilla".to_string(),
            author_name: "Alice".to_string(),
            author_url: "https://alice.example".to_string(),
            site_url: "https://alice.example".to_string(),
        };

        customize_eleventy_config(dir.path(), &opts).unwrap();

        let content = std::fs::read_to_string(dir.path().join("eleventy.config.js")).unwrap();
        assert!(content.contains(r#"["md", "webc", "html"]"#));
        assert!(content.contains(r#"markdownTemplateEngine: "webc""#));
        assert!(content.contains(r#"htmlTemplateEngine: "webc""#));
        assert!(content.contains(r#"title: "Alice's Garden""#));
        assert!(content.contains(r#"url: "https://alice.example""#));
        assert!(content.contains(r#"author: "Alice""#));
    }

    #[test]
    fn test_rename_template_extensions_webc() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        let includes = src.join("_includes");
        std::fs::create_dir_all(&includes).unwrap();
        std::fs::write(src.join("index.njk"), "index").unwrap();
        std::fs::write(src.join("about.njk"), "about").unwrap();
        std::fs::write(includes.join("base.njk"), "base").unwrap();
        std::fs::write(src.join("keep.md"), "markdown").unwrap();

        let opts = ScaffoldOptions {
            name: "Test".to_string(),
            directory: dir.path().to_path_buf(),
            starter: "blog".to_string(),
            template_lang: "webc".to_string(),
            css: "vanilla".to_string(),
            author_name: "".to_string(),
            author_url: "".to_string(),
            site_url: "".to_string(),
        };

        rename_template_extensions(dir.path(), &opts).unwrap();

        assert!(src.join("index.webc").exists());
        assert!(src.join("about.webc").exists());
        assert!(includes.join("base.webc").exists());
        assert!(src.join("keep.md").exists());
        assert!(!src.join("index.njk").exists());
    }

    #[test]
    fn test_rename_template_extensions_nunjucks_noop() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("index.njk"), "index").unwrap();

        let opts = ScaffoldOptions {
            name: "Test".to_string(),
            directory: dir.path().to_path_buf(),
            starter: "blog".to_string(),
            template_lang: "nunjucks".to_string(),
            css: "vanilla".to_string(),
            author_name: "".to_string(),
            author_url: "".to_string(),
            site_url: "".to_string(),
        };

        rename_template_extensions(dir.path(), &opts).unwrap();

        assert!(src.join("index.njk").exists());
    }

    #[test]
    fn test_customize_eleventy_config_nunjucks_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        let original = r#"    templateFormats: ["md", "njk", "html"],
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk","#;
        std::fs::write(dir.path().join("eleventy.config.js"), original).unwrap();

        let opts = ScaffoldOptions {
            name: "Test".to_string(),
            directory: dir.path().to_path_buf(),
            starter: "blog".to_string(),
            template_lang: "nunjucks".to_string(),
            css: "vanilla".to_string(),
            author_name: "".to_string(),
            author_url: "".to_string(),
            site_url: "".to_string(),
        };

        customize_eleventy_config(dir.path(), &opts).unwrap();

        let content = std::fs::read_to_string(dir.path().join("eleventy.config.js")).unwrap();
        assert_eq!(content, original);
    }
}
