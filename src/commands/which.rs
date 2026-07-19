use anyhow::Result;
use clap::Args as ClapArgs;
use std::env;
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Command name to locate
    name: String,
}

pub fn run(args: Args) -> Result<()> {
    match find_on_path(&args.name) {
        Some(path) => {
            println!("{}", path.display());
            Ok(())
        }
        None => {
            eprintln!("terminion: {}: not found", args.name);
            std::process::exit(1);
        }
    }
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;

    #[cfg(windows)]
    let extensions: Vec<String> = env::var("PATHEXT")
        .unwrap_or_else(|_| ".EXE;.CMD;.BAT;.COM".to_string())
        .split(';')
        .map(|s| s.to_lowercase())
        .collect();

    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(name);

        #[cfg(windows)]
        {
            if candidate.extension().is_some() && candidate.is_file() {
                return Some(candidate);
            }
            for ext in &extensions {
                let with_ext = dir.join(format!("{name}{ext}"));
                if with_ext.is_file() {
                    return Some(with_ext);
                }
            }
        }

        #[cfg(not(windows))]
        {
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}
