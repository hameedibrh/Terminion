use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(ClapArgs)]
pub struct Args {
    /// Files to print; use '-' or omit for stdin
    files: Vec<PathBuf>,
}

pub fn run(args: Args) -> Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    if args.files.is_empty() {
        io::copy(&mut io::stdin(), &mut out)?;
        return Ok(());
    }

    for file in &args.files {
        if file.as_os_str() == "-" {
            io::copy(&mut io::stdin(), &mut out)?;
            continue;
        }
        let mut content = Vec::new();
        std::fs::File::open(file)
            .with_context(|| format!("failed to open {}", file.display()))?
            .read_to_end(&mut content)?;
        out.write_all(&content)?;
    }
    Ok(())
}
