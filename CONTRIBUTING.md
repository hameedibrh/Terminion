# Contributing to Terminion

Thanks for considering a contribution. Terminion is small on purpose — please
read this before opening a PR.

## Setup

Requires a stable Rust toolchain (install via [rustup](https://rustup.rs)).

```sh
git clone https://github.com/hameedibrh/Terminion.git
cd Terminion
cargo build
```

## Workflow

1. Open an issue first for anything beyond a small fix (new command, flag
   changes, behavior changes) so the approach can be agreed on before you
   write code.
2. Create a branch off `main`.
3. Make your change. See [ARCHITECTURE.md](ARCHITECTURE.md) for how the
   codebase is organized and the steps for adding a new command.
4. Before opening a PR, run:

   ```sh
   cargo fmt
   cargo clippy -- -D warnings
   cargo build
   cargo test
   ```

5. Manually smoke-test whatever command you touched or added — run it with
   normal input, missing arguments, and a nonexistent path, and confirm the
   output/exit code make sense. CI covers Linux, macOS, and Windows; if your
   change is platform-sensitive (path handling, line endings, `PATH`
   lookup), think through all three even if you can only test one locally.
6. Open a PR describing what changed and why. Reference the issue if there
   is one.

## Coding conventions

- Match existing command style (see `src/commands/*.rs`): a `ClapArgs`
  struct plus a `run()` function, errors as `anyhow::Result`, no `unwrap()`
  on anything that depends on user input, filesystem state, or the
  environment.
- No OS-specific flags or behavior visible to the user — differences are
  handled internally (`#[cfg(windows)]` etc.), the CLI surface stays
  identical across platforms.
- No new dependency without a reason; prefer `std` first.
- Keep flags consistent with the conventions already in use: `-r`
  (recursive), `-f` (force), `-n` (count / line numbers, context-dependent),
  `-i` (case-insensitive), `-p` (create parents).
- `cargo fmt` and `cargo clippy -- -D warnings` must be clean; CI enforces
  this on all three platforms.

## Using AI coding agents

You're welcome to use AI coding agents (Claude Code, Copilot, Cursor, or
anything else) to help write your contribution — the maintainer does too;
see the Acknowledgements section in [README.md](README.md). What isn't
welcome is a PR you haven't personally reviewed and understood. Before
opening one:

- Read every line of your own diff.
- Be able to explain why each change is needed.
- Run the full check list from the Workflow section above yourself, and
  confirm it actually passes on your machine.
- Keep the change scoped to what you set out to do — no unrelated
  reformatting, renames, or "while I was in there" additions.

PRs that read as generated-and-unreviewed (inconsistent style, unexplained
changes, failing checks, no manual testing) will be sent back for cleanup
rather than merged as-is.

## Reporting bugs

Open a GitHub issue with: the exact command you ran, the OS/shell, expected
output, and actual output. If it's a cross-platform inconsistency, note the
behavior on each OS you tested.

## License

By contributing, you agree that your contributions will be licensed under
the project's [MIT License](LICENSE).
