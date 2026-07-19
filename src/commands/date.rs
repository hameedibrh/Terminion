use anyhow::Result;
use chrono::{Local, Utc};
use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {
    /// Print UTC instead of local time
    #[arg(long)]
    utc: bool,
    /// strftime-style format string
    #[arg(short = 'f', long, default_value = "%Y-%m-%d %H:%M:%S")]
    format: String,
}

pub fn run(args: Args) -> Result<()> {
    if args.utc {
        println!("{}", Utc::now().format(&args.format));
    } else {
        println!("{}", Local::now().format(&args.format));
    }
    Ok(())
}
