use crate::infra::repositories::file_repositories::init_prod;

mod collections;
mod infra;
mod repositories;
mod video;

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
        .invoke_handler(tauri::generate_handler![video::process_video])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
