use anyhow::Result;
use clap::Args as ClapArgs;
use std::fs;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Directory to list
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Show hidden entries (names starting with '.')
    #[arg(short = 'a', long)]
    all: bool,
    /// Long format: type, size, modified time, name
    #[arg(short = 'l', long)]
    long: bool,
}

pub fn run(args: Args) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(&args.path)?
        .filter_map(|e| e.ok())
        .filter(|e| args.all || !e.file_name().to_string_lossy().starts_with('.'))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !args.long {
            println!("{name}");
            continue;
        }
        let meta = entry.metadata()?;
        let kind = if meta.is_dir() { "d" } else { "-" };
        let size = meta.len();
        let modified = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        println!("{kind} {size:>10} {modified:>12} {name}");
    }
    Ok(())
}
