use anyhow::Result;


#[tauri::command]
fn check_config_exists() -> bool {
    printspots_core::config::check_config_exists()
}

#[tauri::command]
fn check_pallettes_exists() -> Result<i32, String> {
    printspots_core::config::check_pallettes_exists().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            check_config_exists,
            check_pallettes_exists
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
