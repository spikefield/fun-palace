mod commands;
mod config;
mod error;
mod models;
mod process;

pub use error::TwelvetyError;

use process::ProcessManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(ProcessManager::new())
        .invoke_handler(tauri::generate_handler![
            commands::projects::list_projects,
            commands::projects::add_project,
            commands::projects::remove_project,
            commands::eleventy::eleventy_serve,
            commands::eleventy::eleventy_build,
            commands::eleventy::eleventy_stop,
            commands::eleventy::eleventy_status,
            commands::scaffold::scaffold_project,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
