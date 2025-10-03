use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "printspots")]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate calibration patters
    Calibration {
        /// size of single square of the pattern in mm
        #[arg(short, long, default_value = "10")]
        size: u32,
        
        /// Should the calibration pattern have flat top
        #[arg(long)]
        flat_top: bool,

        /// Optional output filename, defaults to calibration.3mf
        #[arg(short, long)]
        filename: Option<PathBuf>,
    }
}