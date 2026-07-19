# Terminion

[![CI](https://github.com/hameedibrh/Terminion/actions/workflows/ci.yml/badge.svg)](https://github.com/hameedibrh/Terminion/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

One command syntax for Windows, Linux, and macOS shells.

Terminion is a single static binary that gives you the same commands with the
same flags on cmd.exe, PowerShell, bash, and zsh — no more remembering
`Remove-Item -Recurse -Force` on Windows vs `rm -rf` on Unix.

## Project status

Terminion is an open source hobby project, maintained outside of a full-time
job. It is **not intended for production environments**. For anything
running in production, use your platform's native, well-established
commands — that remains the safer and recommended choice. Terminion is best
suited to personal workflows, scripts, and environments where a single
consistent syntax across operating systems is more valuable than relying on
each platform's built-in tooling.

## Install

**Linux / macOS**

```sh
curl -fsSL https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.sh | bash
```

**Windows (PowerShell)**

```powershell
irm https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.ps1 | iex
```

**From source**

```sh
cargo install --path .
```

## Usage

```sh
terminion <command> [args]
```

| Command                                | Description                             |
| --------------------------------------- | ---------------------------------------- |
| `ls [path] [-a -l]`                    | List directory contents                 |
| `cp <src> <dst> [-r]`                  | Copy files or directories               |
| `mv <src> <dst>`                       | Move / rename files or directories      |
| `rm <path>... [-r -f]`                 | Remove files or directories             |
| `cat [file]...`                        | Print file contents (stdin if omitted)  |
| `pwd`                                   | Print working directory                 |
| `cd [path]`                            | Change directory (see note below)       |
| `mkdir <path>... [-p]`                 | Create directories                      |
| `touch <file>...`                      | Create empty file / update timestamp    |
| `echo <text>... [-n]`                  | Print text                              |
| `find [path] [-n name] [-t]`           | Search for files                        |
| `grep <pattern> <path>... [-i -n -r]`  | Search file contents                    |
| `head <file> [-n N]`                   | Print first N lines (default 10)        |
| `tail <file> [-n N]`                   | Print last N lines (default 10)         |
| `wc <file>...`                         | Count lines, words, bytes               |
| `which <name>`                         | Locate a command on PATH                |
| `env [key]`                            | Print environment variables             |
| `clear`                                 | Clear the terminal screen               |
| `whoami`                                | Print the current user name             |
| `hostname`                              | Print the machine host name             |
| `date [--utc] [-f FORMAT]`             | Print current date/time                 |
| `shell`                                 | Start an interactive shell (see below)  |

Run `terminion <command> --help` for full flag details on any command.

### Interactive shell

Typing `terminion` before every single command gets old fast. Run
`terminion` with no arguments (or `terminion shell` explicitly) to drop into
an interactive prompt where you type command names directly:

```
$ terminion
Terminion interactive shell. Type a command name, 'help', or 'exit' to quit.
terminion C:\Users\you\project> ls -l
terminion C:\Users\you\project> cd src
terminion C:\Users\you\project\src> pwd
C:\Users\you\project\src
terminion C:\Users\you\project\src> exit
```

The shell supports command history (arrow keys), quoted arguments
(`mkdir "my folder"`), and `cd`, which only makes sense here: running
`terminion cd <path>` on its own from an external shell changes directory
for that one process and then exits immediately, so it has no lasting
effect — inside `terminion shell` it persists for the rest of the session,
the way `cd` normally behaves. Type `exit`, `quit`, or press Ctrl+D to leave.

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) — how the project is structured, and how
  to add a new command.
- [CONTRIBUTING.md](CONTRIBUTING.md) — dev setup, workflow, coding
  conventions, PR process.
- [AGENTS.md](AGENTS.md) — instructions for AI coding agents working in this
  repository.

## Development

```sh
cargo build
cargo test
cargo fmt
cargo clippy -- -D warnings
```

Releases are built automatically by `.github/workflows/release.yml` when a
`v*.*.*` tag is pushed, producing binaries for Linux (x86_64), macOS
(x86_64 + arm64), and Windows (x86_64).

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for the
workflow and coding conventions.

## Acknowledgements

Thanks to everyone who installs Terminion, reports a platform inconsistency,
or opens a pull request — that's what keeps a project like this moving.

A note from the maintainer: Terminion started as an after-hours, personal
project born out of genuine frustration with juggling different command
syntax across shells. I maintain it alongside a full-time job, not as my
primary work, and I'll be upfront that some of the logic in this codebase
was written with the help of AI coding agents, which I've also used to help
security-review changes before they land. I'd rather be transparent about
that than have anyone assume otherwise.

In that same spirit, contributors are welcome to use AI agents when working
on Terminion — plenty of good contributions will come that way. What matters
is the result: understand the change you're submitting, keep it scoped and
consistent with the rest of the codebase (see
[CONTRIBUTING.md](CONTRIBUTING.md) and [ARCHITECTURE.md](ARCHITECTURE.md)),
and don't open a pull request you haven't personally reviewed and tested.
Contributions that read as unreviewed AI output — inconsistent style,
unexplained changes, unrun checks — will be sent back for cleanup rather
than merged as-is.

## License

Terminion is open source under the [MIT License](LICENSE).
