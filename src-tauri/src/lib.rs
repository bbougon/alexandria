use crate::collections::tauri_commands as collection_commands;
use crate::infra::repositories::file_repositories::init_prod;

mod clock;
mod collections;
mod event_bus;
mod infra;
mod repositories;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_prod();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
        .invoke_handler(tauri::generate_handler![
            collection_commands::create_collection,
            collection_commands::update_video,
            collection_commands::get_collections
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
