use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

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
        #[arg(long, default_value_t = false)]
        flat_top: bool,

        /// Optional output filename, defaults to calibration.3mf
        #[arg(short, long)]
        filename: Option<PathBuf>,
    },

    /// Generate 3D model from input image
    Generate {
        /// Input image
        #[arg(short, long)]
        input: PathBuf,

        /// Size of the image in mm (the larger dimension will be scaled to this size)
        #[arg(short = 's', long, default_value = "100")]
        size: u32,

        /// Should the output mesh have flat top
        #[arg(long)]
        flat_top: bool,

        /// Path to the palette file
        #[arg(short, long)]
        palette: PathBuf,

        /// Output 3MF filename
        #[arg(short, long, default_value = "out.3mf")]
        output: PathBuf,
    },

    /// Configure default print settings - run this before any other command
    Configure,
}