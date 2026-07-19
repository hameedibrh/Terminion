use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(ClapArgs)]
pub struct Args {
    /// Source path
    source: PathBuf,
    /// Destination path
    dest: PathBuf,
    /// Copy directories recursively
    #[arg(short = 'r', long)]
    recursive: bool,
}

pub fn run(args: Args) -> Result<()> {
    if args.source.is_dir() {
        if !args.recursive {
            anyhow::bail!(
                "{} is a directory (use -r to copy recursively)",
                args.source.display()
            );
        }
        copy_dir(&args.source, &args.dest)?;
    } else {
        if let Some(parent) = args.dest.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).ok();
        }
        fs::copy(&args.source, &args.dest).with_context(|| {
            format!(
                "failed to copy {} to {}",
                args.source.display(),
                args.dest.display()
            )
        })?;
    }
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)
                .with_context(|| format!("failed to copy {}", src_path.display()))?;
        }
    }
    Ok(())
}
