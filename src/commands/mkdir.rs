use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Directories to create
    #[arg(required = true)]
    paths: Vec<PathBuf>,
    /// Create parent directories as needed
    #[arg(short = 'p', long)]
    parents: bool,
}

pub fn run(args: Args) -> Result<()> {
    for path in &args.paths {
        let result = if args.parents {
            std::fs::create_dir_all(path)
        } else {
            std::fs::create_dir(path)
        };
        result.with_context(|| format!("failed to create directory {}", path.display()))?;
    }
    Ok(())
}
