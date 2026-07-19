# Changelog

All notable changes to this project are documented in this file. Format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Interactive shell mode (`terminion shell`, or bare `terminion` with no
  subcommand): a REPL with command history and quoted-argument parsing so
  you don't have to type `terminion` before every command.
- `cd` command, primarily useful inside the interactive shell where it
  persists across commands for the rest of the session.
- Integration test suite (`tests/cli.rs`) covering all commands and the
  shell.

## [0.1.0]

### Added

- Initial set of commands: `ls`, `cp`, `mv`, `rm`, `cat`, `pwd`, `mkdir`,
  `touch`, `echo`, `find`, `grep`, `head`, `tail`, `wc`, `which`, `env`,
  `clear`, `whoami`, `hostname`, `date`.
- Cross-platform CI (Linux, macOS, Windows) and release packaging.
- Install scripts for Unix (`install.sh`) and Windows (`install.ps1`).
