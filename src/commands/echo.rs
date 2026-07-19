use anyhow::Result;
use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {
    /// Text to print
    text: Vec<String>,
    /// Do not print the trailing newline
    #[arg(short = 'n')]
    no_newline: bool,
}

pub fn run(args: Args) -> Result<()> {
    let line = args.text.join(" ");
    if args.no_newline {
        print!("{line}");
    } else {
        println!("{line}");
    }
    Ok(())
}
