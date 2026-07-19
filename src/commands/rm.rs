use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::fs;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Paths to remove
    #[arg(required = true)]
    paths: Vec<PathBuf>,
    /// Remove directories and their contents recursively
    #[arg(short = 'r', long)]
    recursive: bool,
    /// Ignore nonexistent paths and errors
    #[arg(short = 'f', long)]
    force: bool,
}

pub fn run(args: Args) -> Result<()> {
    for path in &args.paths {
        let result = if path.is_dir() {
            if args.recursive {
                fs::remove_dir_all(path)
            } else {
                fs::remove_dir(path)
            }
        } else {
            fs::remove_file(path)
        };

        if let Err(e) = result {
            if args.force && e.kind() == std::io::ErrorKind::NotFound {
                continue;
            }
            if args.force {
                continue;
            }
            return Err(e).with_context(|| format!("failed to remove {}", path.display()));
        }
    }
    Ok(())
}
