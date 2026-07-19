use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::fs;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Files to count
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

pub fn run(args: Args) -> Result<()> {
    let mut total = (0usize, 0usize, 0usize);
    let multi = args.files.len() > 1;

    for file in &args.files {
        let bytes = fs::read(file).with_context(|| format!("failed to read {}", file.display()))?;
        let content = String::from_utf8_lossy(&bytes);
        let lines = content.lines().count();
        let words = content.split_whitespace().count();
        let byte_count = bytes.len();

        total.0 += lines;
        total.1 += words;
        total.2 += byte_count;

        println!("{lines:>8} {words:>8} {byte_count:>8} {}", file.display());
    }

    if multi {
        println!("{:>8} {:>8} {:>8} total", total.0, total.1, total.2);
    }
    Ok(())
}
