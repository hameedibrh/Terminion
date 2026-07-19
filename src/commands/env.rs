use anyhow::Result;
use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {
    /// Print only this variable's value
    key: Option<String>,
}

pub fn run(args: Args) -> Result<()> {
    match args.key {
        Some(key) => {
            if let Ok(value) = std::env::var(&key) {
                println!("{value}");
            }
        }
        None => {
            let mut vars: Vec<(String, String)> = std::env::vars().collect();
            vars.sort();
            for (k, v) in vars {
                println!("{k}={v}");
            }
        }
    }
    Ok(())
}
