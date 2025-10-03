mod cli;
// mod commands;
// mod config;

use clap::Parser;
use cli::{Cli, Commands};
use printspots_core::grayscale::calibration::generate_calibration_objects;
use printspots_core::config::PrintConfig;
use printspots_core::grayscale::export_to_3mf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {

        Some(Commands::Calibration { size, flat_top, filename }) => {
            let config = PrintConfig::default(); // Load or create default config
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
        
        None => {
            // This case is handled by clap's arg_required_else_help
            unreachable!()
        }
    }
    
    Ok(())
}