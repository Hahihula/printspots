mod cli;
// mod commands;
// mod config;
use image::{ImageReader, RgbImage, Rgb};

use clap::Parser;
use cli::{Cli, Commands};
use printspots_core::grayscale::generate::generate_image;
use printspots_core::mesh::add_build_plate_padding;
use printspots_core::{config::load_config, grayscale::calibration::generate_calibration_objects, grayscale::image_processing::dither_to_palette};
use printspots_core::config::{save_config, PrintConfig, PrintingConstraints};
use dialoguer::{theme::ColorfulTheme, Input, Confirm};

use printspots_core::grayscale::{enforce_min_feature_size, export_to_3mf, export_to_stl, ColorPalette};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("PrintSpots - Convert images to 3D printable grayscale models");
    println!("");
    println!("==============================================");
    println!("Before any other command, please run `printspots configure` to set up your default print settings.");
    println!("==============================================");

    match cli.command {

        Some(Commands::Calibration { size, flat_top, filename }) => {
            let config = load_config();
            let calibration_objects = generate_calibration_objects(&config, size as f32, flat_top);
            let filename = filename.unwrap_or_else(|| "calibration.3mf".into());
            match export_to_3mf(&calibration_objects, filename.to_str().unwrap()) {
                Ok(_) => {
                    println!("Calibration pattern saved to {}", filename.to_str().unwrap());
                },
                Err(e) => {
                    eprintln!("Error exporting calibration pattern: {}", e);
                }
            }
        }

        Some(Commands::Generate { input, size, flat_top, stl, add_pads, palette, output }) => {
            if flat_top {
                println!("⚠ Warning: Flat top option is not yet implemented and will be ignored.");
            }
            println!("Processing image: {}", input.to_str().unwrap());
            let config = load_config();
            let img = ImageReader::open(input).unwrap().decode().unwrap();
            let rgb_img: RgbImage = img.to_rgb8();
            let palette = ColorPalette::load_from_file(palette).unwrap_or_else(|_| {
                eprintln!("⚠ Could not load palette file, using fake grayscale palette.");
                println!("⚠ Please generate a proper palette using the calibration command and provide its path.");
                println!("⚠ Fake palette will map colors linearly to layer counts, which WILL NOT yield REAL results.");
                ColorPalette::fake(config.max_layers as u32)
            });
            let dithered = dither_to_palette(&rgb_img, &palette);

            let constrains = PrintingConstraints::default(); // for now default constraints

            let printable = enforce_min_feature_size(&dithered, &palette, &constrains, &config);
            match printable.save("prediction.png") {
                Ok(_) => println!("✓ Saved prediction image to prediction.png"),
                Err(e) => eprintln!("⚠ Could not save prediction image: {}", e),
            }
            println!("Generating 3D printable objects...");

            let mut image_objects = generate_image(&printable, &palette, &config, flat_top);

            if add_pads {
                println!("Adding build plate padding to objects...");
                add_build_plate_padding(&mut image_objects.black_mesh, 3.0); // this is unnecesary
                add_build_plate_padding(&mut image_objects.white_mesh, 5.0);
            }

            // Export results
            match export_to_3mf(&image_objects, output.to_str().unwrap()) {
                Ok(_) => {
                    println!("✓ 3MF file saved successfully to {}", output.to_str().unwrap());
                },
                Err(e) => {
                    eprintln!("✗ Error exporting 3MF file: {}", e);
                }
            }
            // Optionally export also to STL files
            if stl {
                println!("Exporting individual STL files for black and white meshes...");
                export_to_stl(&image_objects, "black_mesh.stl", "white_mesh.stl")?;
            }

            println!("✓ Complete! Stats:");
            println!("Black mesh: {} vertices, {} triangles", 
                image_objects.black_mesh.vertices.vertex.len(), 
                image_objects.black_mesh.triangles.triangle.len());
            println!("White mesh: {} vertices, {} triangles", 
                image_objects.white_mesh.vertices.vertex.len(), 
                image_objects.white_mesh.triangles.triangle.len());
        }

        Some(Commands::Configure) => {
            interactive_configure()?;
        }
        
        None => {
            // This case is handled by clap's arg_required_else_help
            unreachable!()
        }
    }
    
    Ok(())
}

/// Interactive configuration wizard using dialoguer
pub fn interactive_configure() -> Result<PrintConfig, Box<dyn std::error::Error>> {
    let current_config = load_config();
    let theme = ColorfulTheme::default();
    
    println!("\n╔═══════════════════════════════════════╗");
    println!("║   PrintSpots Configuration Wizard    ║");
    println!("╚═══════════════════════════════════════╝\n");
    
    println!("  ℹ The thickness of the base layer that all patterns are built upon\n");

    let base_thickness: f32 = Input::with_theme(&theme)
        .with_prompt("Base thickness (mm)")
        .with_initial_text(current_config.base_thickness.to_string())
        .default(current_config.base_thickness)
        .interact_text()?;
    
    println!("  ℹ The height of each individual layer in the print.\nShould be set to minimal printable height for your printer for best results.\n");
    
    let layer_thickness: f32 = Input::with_theme(&theme)
        .with_prompt("Layer thickness (mm)")
        .with_initial_text(current_config.layer_thickness.to_string())
        .default(current_config.layer_thickness)
        .interact_text()?;
    
    println!("  ℹ The default size for the larger dimension of generated images\n");
    
    let image_size_mm: f32 = Input::with_theme(&theme)
        .with_prompt("Image size (mm)")
        .with_initial_text(current_config.image_size_mm.to_string())
        .default(current_config.image_size_mm)
        .interact_text()?;
    
    println!("  ℹ The maximum number of layers that can be stacked. default: 19\n");
    
    let max_layers: f32 = Input::with_theme(&theme)
        .with_prompt("Maximum layers")
        .with_initial_text(current_config.max_layers.to_string())
        .default(current_config.max_layers)
        .interact_text()?;
    
    
    let new_config = PrintConfig {
        base_thickness,
        layer_thickness,
        image_size_mm,
        max_layers,
    };
    
    println!("\n┌─── New Configuration ───┐");
    println!("│ Base thickness:  {:.2} mm", new_config.base_thickness);
    println!("│ Layer thickness: {:.2} mm", new_config.layer_thickness);
    println!("│ Image size:      {:.2} mm", new_config.image_size_mm);
    println!("│ Max layers:      {:.0}", new_config.max_layers);
    println!("└─────────────────────────┘\n");
    
    if Confirm::with_theme(&theme)
        .with_prompt("Save this configuration?")
        .default(true)
        .interact()?
    {
        save_config(&new_config)?;
        Ok(new_config)
    } else {
        println!("✗ Configuration not saved.");
        Ok(current_config)
    }
}