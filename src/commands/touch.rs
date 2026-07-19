use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use filetime::FileTime;
use std::fs::OpenOptions;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Files to create or update
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

pub fn run(args: Args) -> Result<()> {
    for file in &args.files {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(file)
            .with_context(|| format!("failed to create {}", file.display()))?;
        let now = FileTime::now();
        filetime::set_file_times(file, now, now)
            .with_context(|| format!("failed to update timestamp for {}", file.display()))?;
    }
    Ok(())
}
