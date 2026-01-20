use anyhow::Result;
use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;

mod generator;
mod listener;
mod parser;
mod types;

use generator::{generate_icons, OptimizationOptions};

#[derive(ClapParser, Debug)]
#[command(
    name = "iconscript",
    about = "iconscript parser and SVG generator",
    version
)]
struct Args {
    /// Input iconscript file.
    #[arg(value_name = "INPUT")]
    input: Option<PathBuf>,

    /// Output directory for SVG files.
    #[arg(short, long, value_name = "DIR", default_value = "output")]
    output: PathBuf,

    /// Enable sketch mode (output raw paths without combining).
    #[arg(short, long)]
    sketch: bool,

    /// Disable coordinate rounding.
    #[arg(long)]
    no_rounding: bool,

    /// Disable duplicate point removal.
    #[arg(long)]
    no_deduplication: bool,

    /// Disable collinear point simplification.
    #[arg(long)]
    no_collinear: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_file = args
        .input
        .unwrap_or_else(|| PathBuf::from("main.iconscript"));
    let output_dir = args.output;
    let sketch_mode = args.sketch;

    // Build optimization options from CLI flags.
    let optimization_options = OptimizationOptions {
        enable_rounding: !args.no_rounding,
        enable_deduplication: !args.no_deduplication,
        enable_collinear_simplification: !args.no_collinear,
    };

    // Read input file.
    let content = fs::read_to_string(&input_file)?;

    // Parse and generate icons.
    let icon_count = generate_icons(
        &content,
        &output_dir,
        sketch_mode,
        &optimization_options,
    )?;

    println!(
        "\nGenerated {} SVG files in the `{}` directory.",
        icon_count,
        output_dir.display()
    );

    Ok(())
}
