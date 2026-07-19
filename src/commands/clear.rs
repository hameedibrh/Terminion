use anyhow::Result;
use clap::Args as ClapArgs;
use std::io::Write;

#[derive(ClapArgs)]
pub struct Args {}

pub fn run(_args: Args) -> Result<()> {
    // ANSI clear + move cursor home; supported by Windows Terminal, cmd (VT mode), and Unix shells.
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush()?;
    Ok(())
}
