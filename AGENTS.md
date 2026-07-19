# AGENTS.md

Instructions for AI coding agents (any tool) working in this repository.
Read this before making changes. It is the machine-readable counterpart to
[ARCHITECTURE.md](ARCHITECTURE.md) and [CONTRIBUTING.md](CONTRIBUTING.md) —
read those too for full context.

## What this project is

Terminion is a single Rust CLI binary that provides one common set of
commands (`ls`, `cp`, `mv`, `rm`, `cat`, `grep`, ...) with identical syntax
and behavior on Windows, Linux, and macOS. The entire point of the project
is cross-platform consistency — that constraint overrides normal
per-OS-idiomatic choices.

## Non-negotiable rules

1. **No behavior differs by OS at the CLI surface.** A flag either exists on
   all platforms and does the same thing, or it doesn't exist. Internal
   implementation may branch on `#[cfg(windows)]` / `#[cfg(unix)]` (see
   `src/commands/which.rs` for `PATHEXT` handling), but the user-visible
   command, flags, and output must be identical.
2. **No shelling out to `cmd`, `powershell`, `/bin/sh`, or any other shell.**
   Everything is implemented directly against `std::fs` / `std::io` or the
   existing dependencies (`clap`, `anyhow`, `walkdir`, `regex`, `chrono`,
   `filetime`, `whoami`). Adding `std::process::Command` to shell out
   defeats the purpose of the project — don't do it.
3. **Every command module has the same shape**: a `#[derive(ClapArgs)]
   struct Args` and `pub fn run(args: Args) -> anyhow::Result<()>`. Follow
   the pattern in an existing file (`src/commands/rm.rs` is a good short
   example) rather than inventing a new structure.
4. **Errors are `anyhow::Result`.** Use `.with_context(|| ...)` when a raw
   `io::Error` wouldn't tell the user which path/operation failed. Don't
   `unwrap()`/`expect()` on filesystem calls, environment lookups, or
   anything else that depends on external state.
5. **No new files or planning documents beyond what's asked.** Don't create
   summary docs, design docs, or extra `.md` files unless the user
   explicitly asks for one.

## Before committing a change

Run, in order:

```sh
cargo fmt
cargo clippy -- -D warnings
cargo build
cargo test
```

All four must be clean. Then manually exercise whatever command you touched
(normal input, a missing/invalid path, and — if relevant — an empty
directory or empty file) and confirm output and exit code look right. See
[CONTRIBUTING.md](CONTRIBUTING.md) for the full checklist.

If you are operating on behalf of a human contributor, make sure they read
and understand the diff before it becomes a PR — don't let a change go out
that no one but you has reviewed. See "Using AI coding agents" in
[CONTRIBUTING.md](CONTRIBUTING.md).

## Adding a command

Follow the numbered steps in the "Adding a new command" section of
[ARCHITECTURE.md](ARCHITECTURE.md). In short: new file in `src/commands/`,
register it in `src/commands/mod.rs`, wire it into the `Command` enum and
match arm in `src/main.rs`, add a row to the table in `README.md`.

## Where things live

| Need to...                          | Look at / edit                          |
| ------------------------------------ | ---------------------------------------- |
| Understand overall structure         | `ARCHITECTURE.md`                        |
| Add or change a command              | `src/commands/<name>.rs`, `src/main.rs`  |
| Change CLI dispatch / global flags   | `src/main.rs`                            |
| Change CI checks                     | `.github/workflows/ci.yml`               |
| Change release/packaging             | `.github/workflows/release.yml`, `install.sh`, `install.ps1` |
| Update install instructions          | `README.md`, `install.sh`, `install.ps1` |
| Understand contribution workflow     | `CONTRIBUTING.md`                        |
