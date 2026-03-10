use crate::models::project::{ProjectEntry, ProjectRegistry};
use crate::FunPalaceError;
use tauri::AppHandle;
use tauri::Manager;

fn registry_path(app: &AppHandle) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("failed to get app data dir")
        .join("projects.json")
}

#[tauri::command]
pub async fn list_projects(app: AppHandle) -> Result<Vec<ProjectEntry>, FunPalaceError> {
    let path = registry_path(&app);
    let registry = ProjectRegistry::load(&path)?;
    Ok(registry.projects)
}

#[tauri::command]
pub async fn add_project(app: AppHandle, entry: ProjectEntry) -> Result<(), FunPalaceError> {
    let path = registry_path(&app);
    let mut registry = ProjectRegistry::load(&path)?;
    registry.add(entry);
    registry.save(&path)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_project(app: AppHandle, id: String) -> Result<(), FunPalaceError> {
    let path = registry_path(&app);
    let mut registry = ProjectRegistry::load(&path)?;
    registry.remove(&id);
    registry.save(&path)?;
    Ok(())
}
