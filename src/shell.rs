use crate::{Cli, dispatch, print_error};
use anyhow::Result;
use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

/// Run an interactive REPL: read a line, parse it as if it were
/// `terminion <line>`, and dispatch it. Runs until `exit`/`quit` or EOF.
pub fn run() -> Result<()> {
    println!("Terminion interactive shell. Type a command name, 'help', or 'exit' to quit.");

    let mut editor = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            anyhow::bail!("could not start an interactive shell in this terminal: {e}");
        }
    };

    loop {
        let cwd = std::env::current_dir().unwrap_or_default();
        let prompt = format!("terminion {}> ", cwd.display());

        let line = match editor.readline(&prompt) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("terminion: {e}");
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let _ = editor.add_history_entry(trimmed);

        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        if trimmed == "help" {
            // `--help` always yields Err(..) from try_parse_from; it never
            // succeeds, so this can't silently do nothing.
            if let Err(e) = Cli::try_parse_from(["terminion", "--help"]) {
                let _ = e.print();
            }
            continue;
        }

        let words = match shell_words::split(trimmed) {
            Ok(words) => words,
            Err(e) => {
                eprintln!("terminion: could not parse input: {e}");
                continue;
            }
        };

        let mut argv = vec!["terminion".to_string()];
        argv.extend(words);

        match Cli::try_parse_from(argv) {
            Ok(cli) => {
                if let Some(command) = cli.command
                    && let Err(e) = dispatch(command)
                {
                    print_error(&e);
                }
            }
            Err(e) => {
                // clap's Error already contains formatted usage/help text.
                let _ = e.print();
            }
        }
    }

    Ok(())
}
