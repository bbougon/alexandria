use crate::collections::tauri_commands as collection_commands;
use crate::infra::repositories::file_repositories::init_prod;
use tauri::Manager;

mod clock;
mod collections;
mod event_bus;
mod infra;
mod repositories;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory")
                .join("collections");

            if !app_data_dir.exists() {
                std::fs::create_dir_all(&app_data_dir)
                    .expect("Failed to create app data directory");
            }

            init_prod(app_data_dir);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            collection_commands::create_collection,
            collection_commands::update_video,
            collection_commands::get_collections
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
