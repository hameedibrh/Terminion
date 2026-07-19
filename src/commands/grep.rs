use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use regex::RegexBuilder;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(ClapArgs)]
pub struct Args {
    /// Pattern to search for (regular expression)
    pattern: String,
    /// Files or directories to search
    #[arg(required = true)]
    paths: Vec<PathBuf>,
    /// Case-insensitive match
    #[arg(short = 'i', long)]
    ignore_case: bool,
    /// Show line numbers
    #[arg(short = 'n', long)]
    line_number: bool,
    /// Search directories recursively
    #[arg(short = 'r', long)]
    recursive: bool,
}

pub fn run(args: Args) -> Result<()> {
    let re = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .with_context(|| format!("invalid pattern: {}", args.pattern))?;

    for path in &args.paths {
        if path.is_dir() {
            if !args.recursive {
                anyhow::bail!(
                    "{} is a directory (use -r to search recursively)",
                    path.display()
                );
            }
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    search_file(entry.path(), &re, &args)?;
                }
            }
        } else {
            search_file(path, &re, &args)?;
        }
    }
    Ok(())
}

fn search_file(path: &std::path::Path, re: &regex::Regex, args: &Args) -> Result<()> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(()), // skip unreadable/binary files
    };
    for (i, line) in content.lines().enumerate() {
        if re.is_match(line) {
            if args.paths.len() > 1 || args.recursive {
                if args.line_number {
                    println!("{}:{}:{}", path.display(), i + 1, line);
                } else {
                    println!("{}:{}", path.display(), line);
                }
            } else if args.line_number {
                println!("{}:{}", i + 1, line);
            } else {
                println!("{line}");
            }
        }
    }
    Ok(())
}
