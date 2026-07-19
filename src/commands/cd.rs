use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Directory to change into
    #[arg(default_value = ".")]
    path: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    std::env::set_current_dir(&args.path)
        .with_context(|| format!("failed to change directory to {}", args.path.display()))
}
