use crate::models::project::ProjectRegistry;
use crate::process::ProcessManager;
use crate::FunPalaceError;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn eleventy_serve(
    app: AppHandle,
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<u32, FunPalaceError> {
    let registry_path = app.path().app_data_dir().unwrap().join("projects.json");
    let registry = ProjectRegistry::load(&registry_path)?;
    let project = registry.find(&project_id).ok_or_else(|| FunPalaceError {
        code: "PROJECT_NOT_FOUND".to_string(),
        message: format!("Project '{}' not found", project_id),
    })?;

    let project_dir = project.path.clone();

    process_manager.serve(&project_id, &project_dir, "npx")
}

#[tauri::command]
pub async fn eleventy_build(
    app: AppHandle,
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<String, FunPalaceError> {
    let registry_path = app.path().app_data_dir().unwrap().join("projects.json");
    let registry = ProjectRegistry::load(&registry_path)?;
    let project = registry.find(&project_id).ok_or_else(|| FunPalaceError {
        code: "PROJECT_NOT_FOUND".to_string(),
        message: format!("Project '{}' not found", project_id),
    })?;

    let project_dir = project.path.clone();

    process_manager.build(&project_dir, "npx")
}

#[tauri::command]
pub async fn eleventy_stop(
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<(), FunPalaceError> {
    process_manager.stop(&project_id)
}

#[tauri::command]
pub async fn eleventy_status(
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<bool, FunPalaceError> {
    Ok(process_manager.is_running(&project_id))
}
