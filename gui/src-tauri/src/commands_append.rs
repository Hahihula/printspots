use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;
use base64::{Engine as _, engine::general_purpose};
use image::{ImageFormat, RgbImage};
use printspots_core::{
    config::{ColorPalette, PrintConfig, PrinterProfile, PrintingConstraints},
    grayscale::{dither_to_palette, enforce_min_feature_size},
    mesh::{generate_image, add_build_plate_padding, export_to_3mf},
};

// ... existing code ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub id: String,
    pub name: String,
    pub image_size_mm: f32,
    pub base_thickness: f32,
    pub layer_thickness: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image: Option<String>,
    #[serde(default)]
    pub settings: serde_json::Value,
    pub last_modified: i64,
}

#[tauri::command]
pub fn save_project_config(
    project_data: ProjectData,
) -> Result<(), String> {
    // Create project directory
    let project_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("projects")
        .join(&project_data.id);
    
    if !project_dir.exists() {
        fs::create_dir_all(&project_dir).map_err(|e| e.to_string())?;
    }

    // Save full project data to JSON
    let config_path = project_dir.join("project.json");
    let config_json = serde_json::to_string_pretty(&project_data)
        .map_err(|e| format!("Failed to serialize project: {}", e))?;
    
    fs::write(&config_path, config_json)
        .map_err(|e| format!("Failed to write project: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn load_project_from_file(
    path: String,
) -> Result<ProjectData, String> {
    // Read project.json file
    let project_json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read project file: {}", e))?;
    
    // Deserialize project data
    let project_data: ProjectData = serde_json::from_str(&project_json)
        .map_err(|e| format!("Failed to parse project file: {}", e))?;
    
    Ok(project_data)
}
