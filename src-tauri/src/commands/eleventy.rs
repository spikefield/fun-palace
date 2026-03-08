use crate::models::project::ProjectRegistry;
use crate::process::ProcessManager;
use crate::TwelvetyError;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn eleventy_serve(
    app: AppHandle,
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<u32, TwelvetyError> {
    let registry_path = app.path().app_data_dir().unwrap().join("projects.json");
    let registry = ProjectRegistry::load(&registry_path)?;
    let project = registry.find(&project_id).ok_or_else(|| TwelvetyError {
        code: "PROJECT_NOT_FOUND".to_string(),
        message: format!("Project '{}' not found", project_id),
    })?;

    let project_dir = project.path.clone();
    let node_path = "node";
    let eleventy_path = "npx";

    process_manager.serve(&project_id, &project_dir, node_path, eleventy_path)
}

#[tauri::command]
pub async fn eleventy_build(
    app: AppHandle,
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<String, TwelvetyError> {
    let registry_path = app.path().app_data_dir().unwrap().join("projects.json");
    let registry = ProjectRegistry::load(&registry_path)?;
    let project = registry.find(&project_id).ok_or_else(|| TwelvetyError {
        code: "PROJECT_NOT_FOUND".to_string(),
        message: format!("Project '{}' not found", project_id),
    })?;

    let project_dir = project.path.clone();
    let node_path = "node";
    let eleventy_path = "npx";

    process_manager.build(&project_dir, node_path, eleventy_path)
}

#[tauri::command]
pub async fn eleventy_stop(
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<(), TwelvetyError> {
    process_manager.stop(&project_id)
}

#[tauri::command]
pub async fn eleventy_status(
    process_manager: State<'_, ProcessManager>,
    project_id: String,
) -> Result<bool, TwelvetyError> {
    Ok(process_manager.is_running(&project_id))
}
