use anyhow::Result;
use clap::Args as ClapArgs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(ClapArgs)]
pub struct Args {
    /// Directory to search
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Substring to match against file/directory names
    #[arg(short = 'n', long)]
    name: Option<String>,
    /// Only match files, not directories
    #[arg(short = 't', long)]
    type_file: bool,
}

pub fn run(args: Args) -> Result<()> {
    for entry in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()) {
        if args.type_file && !entry.file_type().is_file() {
            continue;
        }
        if let Some(name) = &args.name {
            let file_name = entry.file_name().to_string_lossy();
            if !file_name.contains(name.as_str()) {
                continue;
            }
        }
        println!("{}", entry.path().display());
    }
    Ok(())
}
