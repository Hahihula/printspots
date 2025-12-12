use anyhow::Result;

mod commands;

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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_config_exists,
            check_pallettes_exists,
            commands::save_printer_profile,
            commands::get_printer_profiles,
            commands::generate_calibration,
            commands::save_palette,
            commands::get_palettes,
            commands::save_wizard_progress,
            commands::load_wizard_progress,
            commands::clear_wizard_progress,
            commands::show_in_folder,
            commands::import_image,
            commands::generate_prediction,
            commands::generate_3mf,
            commands::save_project_config,
            commands::load_project_from_file,
            commands::read_project_image
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
