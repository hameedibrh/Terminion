# Architecture

## Overview

Terminion is a single Rust binary. It parses a subcommand with
[`clap`](https://docs.rs/clap) and dispatches to a per-command module. There
is no shared runtime state, no plugin system, and no config file — every
command is a self-contained function that reads its arguments and does one
thing.

## Layout

```
src/
  main.rs              CLI entry point: defines the subcommand enum and a
                        dispatch() function that calls commands::<name>::run()
  shell.rs              interactive REPL (`terminion` with no args, or
                        `terminion shell`); reuses dispatch() for every line
  commands/
    mod.rs              declares every command module
    ls.rs, cp.rs, ...    one file per command
```

Each command module follows the same shape:

```rust
use anyhow::Result;
use clap::Args as ClapArgs;

#[derive(ClapArgs)]
pub struct Args {
    // clap-derived fields: positional args, #[arg(short, long)] flags
}

pub fn run(args: Args) -> Result<()> {
    // do the thing, return Err(...) on failure
}
```

`main.rs` wires each module's `Args` struct into the top-level `Command` enum
and its `dispatch()` function calls `run()` on whichever variant was
selected. That's the entire dispatch mechanism — there's no dynamic registry
to keep in sync. `dispatch()` is called from two places: once in `main()` for
normal one-shot invocations (`terminion ls`), and once per line in
`shell.rs`'s REPL loop, which re-parses each typed line as if it were a
fresh `terminion <line>` invocation and calls the same `dispatch()`. This is
also why `cd` works the way it does: it calls `std::env::set_current_dir`
in-process, so inside the shell's single long-lived process it persists
across subsequent commands, whereas `terminion cd <path>` run standalone
only affects that one short-lived process.

## Design principles

- **One behavior everywhere.** A command must behave the same on Windows,
  Linux, and macOS given the same flags. Platform differences are handled
  inside the command (see `which.rs` for `PATHEXT` handling, `mv.rs` for the
  cross-filesystem rename fallback), never exposed to the user as
  OS-specific flags.
- **No shell dependency.** Commands do not shell out to `cmd`, `powershell`,
  or `/bin/sh`. Everything is implemented against `std::fs`/`std::io` or a
  small, explicit set of crates (`walkdir`, `regex`, `chrono`, `filetime`,
  `whoami`, `rustyline`, `shell-words`). This is what makes the binary
  portable and the behavior predictable. `terminion shell` is Terminion's
  own REPL, not a wrapper around the host OS's shell.
- **Errors via `anyhow`.** Commands return `anyhow::Result<()>`. `main.rs`
  prints `Err` as `terminion: <message>` and exits with status 1. Use
  `.with_context(...)` when the raw error (e.g. an `io::Error`) wouldn't
  tell the user which path or operation failed.
- **No global config, no interactive prompts.** Every command is
  non-interactive and driven entirely by CLI arguments, so it's safe to use
  in scripts on any platform.

## Adding a new command

1. Create `src/commands/<name>.rs` with an `Args` struct (`#[derive(ClapArgs)]`)
   and a `pub fn run(args: Args) -> anyhow::Result<()>`.
2. Add `pub mod <name>;` to `src/commands/mod.rs` (keep the list
   alphabetical).
3. Add a variant to the `Command` enum in `src/main.rs` and a matching arm in
   the `dispatch()` function. No change to `shell.rs` is needed — it calls
   `dispatch()` too, so every command is automatically available inside
   `terminion shell` as soon as it's wired into `main.rs`.
4. Add a row to the command table in `README.md`.
5. Run `cargo fmt && cargo clippy -- -D warnings && cargo build` and smoke
   test the new command manually (see [CONTRIBUTING.md](CONTRIBUTING.md)).

Keep new commands consistent with existing ones: short flags matching Unix
conventions where they exist (`-r` recursive, `-f` force, `-n` count/lines,
`-i` case-insensitive), long flags for everything, no OS-specific behavior
visible in the CLI surface.

## CI / Release

- `.github/workflows/ci.yml` — runs `cargo build`, `cargo test`,
  `cargo fmt --check`, and `cargo clippy -- -D warnings` on Linux, macOS, and
  Windows for every push/PR to `main`.
- `.github/workflows/release.yml` — on a `v*.*.*` tag push, cross-compiles
  release binaries for `x86_64-unknown-linux-gnu`,
  `x86_64-apple-darwin`/`aarch64-apple-darwin`, and
  `x86_64-pc-windows-msvc`, then attaches them to a GitHub Release.
- `install.sh` / `install.ps1` download the latest release archive for the
  caller's platform and place the binary on `PATH`.
