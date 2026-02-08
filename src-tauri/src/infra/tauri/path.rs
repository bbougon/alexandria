use tauri::{AppHandle, Manager, Runtime};

pub fn allow_path<R: Runtime>(app: &AppHandle<R>, path: &str) -> Result<(), String> {
    app.asset_protocol_scope()
        .allow_file(path)
        .map_err(|e: tauri::Error| e.to_string())
}
