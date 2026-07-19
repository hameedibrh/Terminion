use anyhow::Result;
use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {}

pub fn run(_args: Args) -> Result<()> {
    println!(
        "{}",
        whoami::hostname().unwrap_or_else(|_| "unknown".to_string())
    );
    Ok(())
}
