use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::fs;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Source path
    source: PathBuf,
    /// Destination path
    dest: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    // fs::rename fails across filesystems/drives; fall back to copy + remove.
    if fs::rename(&args.source, &args.dest).is_ok() {
        return Ok(());
    }

    if args.source.is_dir() {
        copy_dir_all(&args.source, &args.dest)?;
        fs::remove_dir_all(&args.source)
            .with_context(|| format!("failed to remove {}", args.source.display()))?;
    } else {
        fs::copy(&args.source, &args.dest).with_context(|| {
            format!(
                "failed to copy {} to {}",
                args.source.display(),
                args.dest.display()
            )
        })?;
        fs::remove_file(&args.source)
            .with_context(|| format!("failed to remove {}", args.source.display()))?;
    }
    Ok(())
}

fn copy_dir_all(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
