mod commands;
mod shell;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "terminion",
    version,
    about = "One command syntax for Windows, Linux and macOS shells"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
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
    /// Change the current directory (persists only inside `terminion shell`)
    Cd(commands::cd::Args),
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
    /// Start an interactive shell so you don't have to type `terminion` before every command
    Shell,
}

fn dispatch(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Ls(args) => commands::ls::run(args),
        Command::Cp(args) => commands::cp::run(args),
        Command::Mv(args) => commands::mv::run(args),
        Command::Rm(args) => commands::rm::run(args),
        Command::Cat(args) => commands::cat::run(args),
        Command::Pwd(args) => commands::pwd::run(args),
        Command::Cd(args) => commands::cd::run(args),
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
        Command::Shell => shell::run(),
    }
}

/// Print `terminion: <error>`, plus an OS-appropriate escalation hint if the
/// underlying cause was a permission error (e.g. writing to a system path
/// without `sudo` on Linux/macOS, or without an elevated shell on Windows).
fn print_error(e: &anyhow::Error) {
    eprintln!("terminion: {e}");
    if is_permission_denied(e) {
        let hint = if cfg!(windows) {
            "hint: try running from an elevated PowerShell or Command Prompt (Run as Administrator)."
        } else {
            "hint: try running with sudo."
        };
        eprintln!("{hint}");
    }
}

fn is_permission_denied(e: &anyhow::Error) -> bool {
    e.chain()
        .filter_map(|cause| cause.downcast_ref::<std::io::Error>())
        .any(|io_err| io_err.kind() == std::io::ErrorKind::PermissionDenied)
}

fn main() {
    let cli = Cli::parse();

    // Running `terminion` with no subcommand drops into the same
    // interactive shell as `terminion shell`.
    let result = match cli.command {
        Some(command) => dispatch(command),
        None => shell::run(),
    };

    if let Err(e) = result {
        print_error(&e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::is_permission_denied;

    #[test]
    fn detects_permission_denied_in_error_chain() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = anyhow::Error::new(io_err).context("failed to remove /etc/foo");
        assert!(is_permission_denied(&err));
    }

    #[test]
    fn ignores_unrelated_errors() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let err = anyhow::Error::new(io_err).context("failed to read /etc/foo");
        assert!(!is_permission_denied(&err));
    }
}
