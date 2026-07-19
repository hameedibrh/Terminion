use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::fs;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// File to read
    file: PathBuf,
    /// Number of lines to print
    #[arg(short = 'n', long, default_value_t = 10)]
    lines: usize,
}

pub fn run(args: Args) -> Result<()> {
    let content = fs::read_to_string(&args.file)
        .with_context(|| format!("failed to read {}", args.file.display()))?;
    let all_lines: Vec<&str> = content.lines().collect();
    let start = all_lines.len().saturating_sub(args.lines);
    for line in &all_lines[start..] {
        println!("{line}");
    }
    Ok(())
}
