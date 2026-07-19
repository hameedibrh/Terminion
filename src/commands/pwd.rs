use anyhow::Result;
use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {}

pub fn run(_args: Args) -> Result<()> {
    println!("{}", std::env::current_dir()?.display());
    Ok(())
}
