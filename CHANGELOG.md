# Changelog

All notable changes to this project are documented in this file. Format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- `--global` (`install.sh`) / `-Global` (`install.ps1`) flag for a
  system-wide install (`/usr/local/bin`, or `%ProgramFiles%\Terminion` on
  Windows), requiring sudo / an elevated PowerShell. Per-user remains the
  default.

### Fixed

- `install.ps1` now prompts before adding the install directory to PATH,
  defaulting to yes when run non-interactively.
- `install.ps1` no longer silently leaves a stale binary in place when the
  existing `terminion.exe` is locked by a running process (e.g. an open
  `terminion shell`); it now fails with a clear message instead.

## [0.1.0-alpha] - 2026-07-19

First published release. Alpha: interfaces and flags may still change
between releases without notice.

### Added

- Initial set of commands: `ls`, `cp`, `mv`, `rm`, `cat`, `pwd`, `cd`,
  `mkdir`, `touch`, `echo`, `find`, `grep`, `head`, `tail`, `wc`, `which`,
  `env`, `clear`, `whoami`, `hostname`, `date`.
- Interactive shell mode (`terminion shell`, or bare `terminion` with no
  subcommand): a REPL with command history and quoted-argument parsing so
  you don't have to type `terminion` before every command.
- Cross-platform CI (Linux, macOS, Windows) and release packaging.
- Install scripts for Unix (`install.sh`) and Windows (`install.ps1`).
- Integration test suite (`tests/cli.rs`) covering all commands and the
  shell.
