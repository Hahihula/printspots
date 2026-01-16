use std::path::PathBuf;
use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};
use printspots_core::{
    config::{PrintConfig,PrintingConstraints},
    grayscale::{calibration::generate_calibration_objects, export_to_3mf, generate::generate_image},
};
use printspots_core::mesh::add_build_plate_padding;
use image::Rgb;
use printspots_core::grayscale::{ColorPalette, image_processing::dither_to_palette, enforce_min_feature_size};
use image::{DynamicImage, ImageReader, Luma, GrayImage};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilamentInfo {
    pub brand: String,
    pub color: String,
    pub material: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationSettings {
    pub filaments: Vec<FilamentInfo>,
    // Advanced Settings
    pub base_thickness: f32,
    pub layer_thickness: f32,
    pub square_size: f32,
    pub max_layers: f32,
}

#[tauri::command]
pub fn generate_calibration(profile: PrinterProfile, settings: CalibrationSettings) -> Result<String, String> {
    // Determine output path: inside config_dir/calibrations
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("calibrations");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let filament_str = settings.filaments.iter()
        .map(|f| format!("{}_{}", f.brand.replace(" ", ""), f.color.replace(" ", "")))
        .collect::<Vec<_>>()
        .join("_");

    let filename = format!("{}_calibration_{}.3mf", 
        profile.name.replace(" ", "_"),
        filament_str
    );
    let output_path = config_dir.join(filename);
    let output_path_str = output_path.to_string_lossy().to_string();

    // Create PrintConfig from profile
    let config = PrintConfig {
        base_thickness: settings.base_thickness,
        layer_thickness: settings.layer_thickness,
        image_size_mm: 100.0, // Not used for calibration box generation directly but good to have
        max_layers: settings.max_layers, 
    };

    let objects = generate_calibration_objects(&config, settings.square_size, false);

    export_to_3mf(&objects, &output_path_str)
        .map_err(|e| e.to_string())?;

    Ok(output_path_str)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterProfile {
    pub id: String, // uuid
    pub name: String,
    pub description: String,
    pub bed_width: f32,
    pub bed_depth: f32,
    pub nozzle_diameter: f32,
    pub min_layer_height: f32,
    #[serde(default)]
    pub has_automatic_filament_change: bool,
}

#[tauri::command]
pub fn save_printer_profile(profile: PrinterProfile) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("profiles");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let file_path = config_dir.join(format!("{}.json", profile.id));
    let json = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;
    
    fs::write(file_path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteData {
    pub id: Option<String>,
    pub colors: Vec<[u8; 3]>,
    pub layer_counts: Vec<u32>,
}

#[tauri::command]
pub fn save_palette(data: PaletteData, name: String) -> Result<String, String> {
    

    let colors: Vec<Rgb<u8>> = data.colors.into_iter().map(|c| Rgb(c)).collect();
    
    let palette = ColorPalette {
        colors,
        layer_counts: data.layer_counts,
    };
    
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("palettes"); // core utils uses 'pallettes' or 'palettes'?

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }
    
    // Normalize name
    let filename = format!("{}.ron", name.replace(" ", "_"));
    let path = config_dir.join(filename);
    
    palette.save_to_file(&path).map_err(|e| e.to_string())?;
    
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn get_printer_profiles() -> Result<Vec<PrinterProfile>, String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("profiles");

    if !config_dir.exists() {
        return Ok(Vec::new());
    }

    let mut profiles = Vec::new();
    for entry in fs::read_dir(config_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let profile: PrinterProfile = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            profiles.push(profile);
        }
    }
    
    Ok(profiles)
}

#[tauri::command]
pub fn get_palettes() -> Result<Vec<PaletteData>, String> {

    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("palettes");

    if !config_dir.exists() {
        return Ok(Vec::new());
    }

    let mut palettes = Vec::new();
    let entries = fs::read_dir(config_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("ron") {
            // Load using Core struct which handles RON
            match ColorPalette::load_from_file(&path) {
                Ok(core_palette) => {
                    // Convert back to Transport Struct
                     let colors: Vec<[u8; 3]> = core_palette.colors.iter()
                        .map(|c| [c.0[0], c.0[1], c.0[2]])
                        .collect();
                    
                     let palette = PaletteData {
                         colors,
                         layer_counts: core_palette.layer_counts,
                         id: Some(path.file_stem().unwrap().to_string_lossy().to_string())
                     };
                     palettes.push(palette);
                },
                Err(e) => {
                     println!("Failed to parse palette {:?}: {}", path, e);
                }
            }
        }
    }
    
    Ok(palettes)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardProgress {
    pub step: String,
    pub printer_data: Option<PrinterProfile>,
    pub calibration_data: Option<CalibrationSettings>,
    pub generated_path: Option<String>,
}

#[tauri::command]
pub fn save_wizard_progress(progress: WizardProgress) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let file_path = config_dir.join("wizard_progress.json");
    let json = serde_json::to_string_pretty(&progress).map_err(|e| e.to_string())?;
    
    fs::write(file_path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn load_wizard_progress() -> Result<Option<WizardProgress>, String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots");
    
    let file_path = config_dir.join("wizard_progress.json");
    
    if !file_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let progress: WizardProgress = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(Some(progress))
}

#[tauri::command]
pub fn clear_wizard_progress() -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots");
    
    let file_path = config_dir.join("wizard_progress.json");
    
    if file_path.exists() {
        fs::remove_file(file_path).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

/// Shows a file or folder in the system file explorer
#[tauri::command]
pub fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        match Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
        {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to open folder with explorer: {}", e);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let path = if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            match std::fs::metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            }
        } else {
            path
        };

        // Try using xdg-open first
        if Command::new("xdg-open").arg(&path).spawn().is_err() {
            // Fallback to dbus-send if xdg-open fails
            let uri = format!("file://{}", path);
            match Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:\"{}\"", uri).as_str(),
                    "string:\"\"",
                ])
                .spawn()
            {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to open file with dbus-send: {}", e);
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        match Command::new("open").args(["-R", &path]).spawn() {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to open file with open: {}", e);
            }
        }
    }
  }

#[tauri::command]
pub fn import_image(project_id: String, source_path: String) -> Result<String, String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("projects")
        .join(&project_id);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let source_path_buf = PathBuf::from(&source_path);
    let extension = source_path_buf
        .extension()
        .and_then(|e| e.to_str())
        .ok_or("File has no extension")?;
    
    let dest_filename = format!("source_image.{}", extension);
    let dest_path = config_dir.join(dest_filename);

    // Copy the file to the project directory
    fs::copy(&source_path, &dest_path).map_err(|e| e.to_string())?;

    // Read the file and convert to base64
    let image_bytes = fs::read(&dest_path).map_err(|e| e.to_string())?;
    let base64_data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &image_bytes);
    
    // Determine MIME type from extension
    let mime_type = match extension.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        _ => "application/octet-stream",
    };

    // Return data URL
    Ok(format!("data:{};base64,{}", mime_type, base64_data))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub base_thickness: f32,
    pub layer_thickness: f32,
    pub image_size_mm: f32,
    #[serde(default)]
    pub add_pads: bool,
    #[serde(default)]
    pub flat_top: bool,
}

#[tauri::command]
pub async fn generate_prediction(
    project_id: String,
    image_data: String,
    project_config: ProjectConfig,
    printer_profile_id: String,
    palette_id: String,
) -> Result<String, String> {
    

    // Decode base64 image data
    let base64_start = image_data.find("base64,").ok_or("Invalid data URL format")?;
    let base64_data = &image_data[base64_start + 7..];
    let image_bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, base64_data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // Decode image
    let img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess image format: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    
    // Check for alpha channel and extract mask if present
    let (rgb_img, mask) = if img.color().has_alpha() {
        let rgba_img = img.to_rgba8();
        let rgb_img = DynamicImage::ImageRgba8(rgba_img.clone()).to_rgb8();
        
        // Extract alpha channel as grayscale mask
        let mask = GrayImage::from_fn(rgba_img.width(), rgba_img.height(), |x, y| {
            Luma([rgba_img.get_pixel(x, y)[3]])
        });
        
        (rgb_img, Some(mask))
    } else {
        (img.to_rgb8(), None)
    };

    // Load palette
    let palette_path = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("palettes")
        .join(format!("{}.ron", palette_id));
    
    let palette = ColorPalette::load_from_file(&palette_path)
        .map_err(|e| format!("Failed to load palette: {}", e))?;

    // Derive max_layers from palette
    let max_layers = palette.layer_counts.len() as f32;

    // Load printer profile
    let profile_path = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("profiles")
        .join(format!("{}.json", printer_profile_id));
    
    let profile_content = fs::read_to_string(&profile_path)
        .map_err(|e| format!("Failed to read printer profile: {}", e))?;
    let printer_profile: PrinterProfile = serde_json::from_str(&profile_content)
        .map_err(|e| format!("Failed to parse printer profile: {}", e))?;

    // Build PrintConfig
    let config = PrintConfig {
        base_thickness: project_config.base_thickness,
        layer_thickness: project_config.layer_thickness,
        image_size_mm: project_config.image_size_mm,
        max_layers,
    };

    // Build PrintingConstraints
    let constraints = PrintingConstraints {
        min_feature_size_mm: 2.0 * printer_profile.nozzle_diameter,
        merge_small_features: true, // TODO: expose in advanced section
        erosion_dilation_passes: 1, // TODO: expose in advanced section
    };

    // Apply dithering
    let dithered = dither_to_palette(&rgb_img, &palette);

    // Apply feature size enforcement
    let printable = enforce_min_feature_size(&dithered, &palette, &constraints, &config);

    // Save to project directory
    let project_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("projects")
        .join(&project_id);
    
    if !project_dir.exists() {
        fs::create_dir_all(&project_dir).map_err(|e| e.to_string())?;
    }

    let prediction_path = project_dir.join("prediction.png");
    printable.save(&prediction_path)
        .map_err(|e| format!("Failed to save prediction: {}", e))?;
    
    // Save mask if present
    if let Some(mask_img) = mask {
        let mask_path = project_dir.join("mask.png");
        mask_img.save(&mask_path)
            .map_err(|e| format!("Failed to save mask: {}", e))?;
    };

    // Encode as base64 and return
    let prediction_bytes = fs::read(&prediction_path).map_err(|e| e.to_string())?;
    let base64_prediction = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &prediction_bytes);
    
    Ok(format!("data:image/png;base64,{}", base64_prediction))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStats {
    pub output_path: String,
    pub black_vertices: usize,
    pub black_triangles: usize,
    pub white_vertices: usize,
    pub white_triangles: usize,
}

#[tauri::command]
pub async fn generate_3mf(
    project_id: String,
    project_config: ProjectConfig,
    _printer_profile_id: String,
    palette_id: String,
) -> Result<MeshStats, String> {
    

    // Load prediction image
    let project_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("projects")
        .join(&project_id);
    
    let prediction_path = project_dir.join("prediction.png");
    if !prediction_path.exists() {
        return Err("Prediction image not found. Please generate prediction first.".to_string());
    }

    let img = ImageReader::open(&prediction_path)
        .map_err(|e| format!("Failed to open prediction: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode prediction: {}", e))?;
    
    let prediction = img.to_rgb8();
    
    // Load mask if it exists
    let mask_path = project_dir.join("mask.png");
    let mask = if mask_path.exists() {
        let mask_img = ImageReader::open(&mask_path)
            .map_err(|e| format!("Failed to open mask: {}", e))?
            .decode()
            .map_err(|e| format!("Failed to decode mask: {}", e))?
            .to_luma8();
        Some(mask_img)
    } else {
        None
    };

    // Load palette
    let palette_path = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("palettes")
        .join(format!("{}.ron", palette_id));
    
    let palette = ColorPalette::load_from_file(&palette_path)
        .map_err(|e| format!("Failed to load palette: {}", e))?;

    // Derive max_layers from palette
    let max_layers = palette.layer_counts.len() as f32;

    // Build PrintConfig
    let config = PrintConfig {
        base_thickness: project_config.base_thickness,
        layer_thickness: project_config.layer_thickness,
        image_size_mm: project_config.image_size_mm,
        max_layers,
    };

    // Generate 3D meshes
    let mut image_objects = generate_image(&prediction, &palette, &config, project_config.flat_top, mask.as_ref());

    // Add pads if requested
    if project_config.add_pads {
        add_build_plate_padding(&mut image_objects.black_mesh, 10.0);
        add_build_plate_padding(&mut image_objects.white_mesh, 10.0);
    }

    // Export to 3MF
    let output_path = project_dir.join("output.3mf");
    export_to_3mf(&image_objects, output_path.to_str().unwrap())
        .map_err(|e| format!("Failed to export 3MF: {}", e))?;

    // Collect stats
    let stats = MeshStats {
        output_path: output_path.to_string_lossy().to_string(),
        black_vertices: image_objects.black_mesh.vertices.vertex.len(),
        black_triangles: image_objects.black_mesh.triangles.triangle.len(),
        white_vertices: image_objects.white_mesh.vertices.vertex.len(),
        white_triangles: image_objects.white_mesh.triangles.triangle.len(),
    };

    Ok(stats)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub id: String,
    pub name: String,
    pub image_size_mm: f32,
    pub base_thickness: f32,
    pub layer_thickness: f32,
    #[serde(default)]
    pub add_pads: bool,
    #[serde(default)]
    pub flat_top: bool,
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

#[tauri::command]
pub fn read_project_image(
    project_id: String,
    image_name: String,
) -> Result<String, String> {
    // Get project directory
    let project_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("printspots")
        .join("projects")
        .join(&project_id);
    
    let image_path = project_dir.join(&image_name);
    
    if !image_path.exists() {
        return Err(format!("Image not found: {}", image_name));
    }

    // Read image file
    let image_data = fs::read(&image_path)
        .map_err(|e| format!("Failed to read image: {}", e))?;
    
    // Encode as base64 data URL
    let base64_data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &image_data);
    let data_url = format!("data:image/png;base64,{}", base64_data);
    
    Ok(data_url)
}
