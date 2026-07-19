mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "terminion",
    version,
    about = "One command syntax for Windows, Linux and macOS shells"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List directory contents
    Ls(commands::ls::Args),
    /// Copy files or directories
    Cp(commands::cp::Args),
    /// Move / rename files or directories
    Mv(commands::mv::Args),
    /// Remove files or directories
    Rm(commands::rm::Args),
    /// Print file contents
    Cat(commands::cat::Args),
    /// Print working directory
    Pwd(commands::pwd::Args),
    /// Create directories
    Mkdir(commands::mkdir::Args),
    /// Create an empty file / update its timestamp
    Touch(commands::touch::Args),
    /// Print text
    Echo(commands::echo::Args),
    /// Find files by name
    Find(commands::find::Args),
    /// Search file contents with a pattern
    Grep(commands::grep::Args),
    /// Print the first lines of a file
    Head(commands::head::Args),
    /// Print the last lines of a file
    Tail(commands::tail::Args),
    /// Count lines, words and bytes
    Wc(commands::wc::Args),
    /// Locate a command on PATH
    Which(commands::which::Args),
    /// Print or set environment variables
    Env(commands::env::Args),
    /// Clear the terminal screen
    Clear(commands::clear::Args),
    /// Print the current user name
    Whoami(commands::whoami::Args),
    /// Print the machine host name
    Hostname(commands::hostname::Args),
    /// Print the current date and time
    Date(commands::date::Args),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Ls(args) => commands::ls::run(args),
        Command::Cp(args) => commands::cp::run(args),
        Command::Mv(args) => commands::mv::run(args),
        Command::Rm(args) => commands::rm::run(args),
        Command::Cat(args) => commands::cat::run(args),
        Command::Pwd(args) => commands::pwd::run(args),
        Command::Mkdir(args) => commands::mkdir::run(args),
        Command::Touch(args) => commands::touch::run(args),
        Command::Echo(args) => commands::echo::run(args),
        Command::Find(args) => commands::find::run(args),
        Command::Grep(args) => commands::grep::run(args),
        Command::Head(args) => commands::head::run(args),
        Command::Tail(args) => commands::tail::run(args),
        Command::Wc(args) => commands::wc::run(args),
        Command::Which(args) => commands::which::run(args),
        Command::Env(args) => commands::env::run(args),
        Command::Clear(args) => commands::clear::run(args),
        Command::Whoami(args) => commands::whoami::run(args),
        Command::Hostname(args) => commands::hostname::run(args),
        Command::Date(args) => commands::date::run(args),
    };

    if let Err(e) = result {
        eprintln!("terminion: {e}");
        std::process::exit(1);
    }
}
