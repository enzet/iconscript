use anyhow::{bail, Result};
use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod generator;
mod listener;
mod parser;
mod types;

use generator::{generate_icons, OptimizationOptions};

/// Parse version string into (major, minor, patch) tuple.
/// Minor and patch default to 0 if not provided.
fn parse_version(version_str: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = version_str.split('.').collect();
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }
    let major = parts[0].parse().ok()?;
    let minor = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    Some((major, minor, patch))
}

/// Check if the file version is compatible with the tool version.
fn check_version_compatibility(content: &str) -> Result<()> {
    let first_line = content.lines().next().unwrap_or("");

    // Check if first line matches `# iconscript <version>` pattern.
    let prefix = "# iconscript ";
    if !first_line.starts_with(prefix) {
        return Ok(());
    }

    let file_version_str = first_line[prefix.len()..].trim();
    if file_version_str.is_empty() {
        return Ok(());
    }

    let file_version = match parse_version(file_version_str) {
        Some(v) => v,
        None => return Ok(()), // Invalid version format, proceed normally.
    };

    let tool_version = parse_version(VERSION).expect("Invalid tool version");

    // Check if file version is greater than tool version.
    if file_version > tool_version {
        bail!(
            "File requires iconscript version {}, but this is version {}.",
            file_version_str,
            VERSION
        );
    }

    Ok(())
}

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
    #[arg(value_name = "DIR")]
    output: Option<PathBuf>,

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
    let output_dir = args.output.unwrap_or_else(|| PathBuf::from("output"));
    let sketch_mode = args.sketch;

    // Build optimization options from CLI flags.
    let optimization_options = OptimizationOptions {
        enable_rounding: !args.no_rounding,
        enable_deduplication: !args.no_deduplication,
        enable_collinear_simplification: !args.no_collinear,
    };

    // Read input file.
    let content = fs::read_to_string(&input_file)?;

    // Check version compatibility.
    check_version_compatibility(&content)?;

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
