//! Black-box integration tests: run the built binary and check its output.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_terminion"))
}

fn run_in(dir: &Path, args: &[&str]) -> Output {
    bin()
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to run terminion")
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// Feed `script` (one command per line) to `terminion shell` over stdin and
/// return what it printed to stdout.
fn run_shell_in(dir: &Path, script: &str) -> Output {
    let mut child = bin()
        .arg("shell")
        .current_dir(dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn terminion shell");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    child.wait_with_output().expect("shell did not exit")
}

#[test]
fn echo_prints_joined_args() {
    let out = bin().args(["echo", "hello", "world"]).output().unwrap();
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim_end(), "hello world");
}

#[test]
fn echo_no_newline_flag_omits_trailing_newline() {
    let out = bin().args(["echo", "-n", "hi"]).output().unwrap();
    assert!(out.status.success());
    assert_eq!(stdout(&out), "hi");
}

#[test]
fn pwd_prints_current_directory() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_in(dir.path(), &["pwd"]);
    assert!(out.status.success());
    let printed = stdout(&out).trim().to_string();
    assert_eq!(
        std::fs::canonicalize(&printed).unwrap(),
        std::fs::canonicalize(dir.path()).unwrap()
    );
}

#[test]
fn mkdir_touch_ls_round_trip() {
    let dir = tempfile::tempdir().unwrap();

    assert!(
        run_in(dir.path(), &["mkdir", "-p", "sub/nested"])
            .status
            .success()
    );
    assert!(dir.path().join("sub/nested").is_dir());

    assert!(
        run_in(dir.path(), &["touch", "sub/file.txt"])
            .status
            .success()
    );
    assert!(dir.path().join("sub/file.txt").is_file());

    let out = run_in(dir.path(), &["ls", "sub"]);
    assert!(out.status.success());
    let listing = stdout(&out);
    assert!(listing.contains("file.txt"));
    assert!(listing.contains("nested"));
}

#[test]
fn cp_copies_file_contents() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), b"payload").unwrap();

    assert!(
        run_in(dir.path(), &["cp", "a.txt", "b.txt"])
            .status
            .success()
    );
    assert_eq!(std::fs::read(dir.path().join("b.txt")).unwrap(), b"payload");
}

#[test]
fn mv_renames_and_removes_source() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), b"payload").unwrap();

    assert!(
        run_in(dir.path(), &["mv", "a.txt", "b.txt"])
            .status
            .success()
    );
    assert!(!dir.path().join("a.txt").exists());
    assert_eq!(std::fs::read(dir.path().join("b.txt")).unwrap(), b"payload");
}

#[test]
fn rm_removes_file_and_recursive_directory() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), b"x").unwrap();
    std::fs::create_dir(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("sub/b.txt"), b"y").unwrap();

    assert!(run_in(dir.path(), &["rm", "a.txt"]).status.success());
    assert!(!dir.path().join("a.txt").exists());

    assert!(run_in(dir.path(), &["rm", "-r", "sub"]).status.success());
    assert!(!dir.path().join("sub").exists());
}

#[test]
fn rm_without_force_fails_on_missing_path() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_in(dir.path(), &["rm", "missing.txt"]);
    assert!(!out.status.success());
}

#[test]
fn rm_force_ignores_missing_path() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_in(dir.path(), &["rm", "-f", "missing.txt"]);
    assert!(out.status.success());
}

#[test]
fn cat_prints_file_contents() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), b"one\ntwo\n").unwrap();

    let out = run_in(dir.path(), &["cat", "a.txt"]);
    assert!(out.status.success());
    assert_eq!(stdout(&out), "one\ntwo\n");
}

#[test]
fn head_and_tail_limit_lines() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), "1\n2\n3\n4\n5\n").unwrap();

    let head = stdout(&run_in(dir.path(), &["head", "a.txt", "-n", "2"]));
    assert_eq!(head, "1\n2\n");

    let tail = stdout(&run_in(dir.path(), &["tail", "a.txt", "-n", "2"]));
    assert_eq!(tail, "4\n5\n");
}

#[test]
fn wc_counts_lines_words_bytes() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), b"foo bar\nbaz\n").unwrap();

    let out = stdout(&run_in(dir.path(), &["wc", "a.txt"]));
    let numbers: Vec<usize> = out
        .split_whitespace()
        .take(3)
        .map(|n| n.parse().unwrap())
        .collect();
    assert_eq!(numbers, vec![2, 3, 12]);
}

#[test]
fn grep_matches_pattern_and_respects_case_flag() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), "Hello\nworld\nHELLO again\n").unwrap();

    let out = stdout(&run_in(dir.path(), &["grep", "hello", "a.txt", "-i"]));
    assert_eq!(out, "Hello\nHELLO again\n");

    let out = stdout(&run_in(dir.path(), &["grep", "hello", "a.txt"]));
    assert_eq!(out, "");
}

#[test]
fn find_matches_by_name_substring() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("sub/target.txt"), b"").unwrap();
    std::fs::write(dir.path().join("other.txt"), b"").unwrap();

    let out = stdout(&run_in(dir.path(), &["find", ".", "-n", "target"]));
    assert!(out.contains("target.txt"));
    assert!(!out.contains("other.txt"));
}

#[test]
fn which_finds_a_known_system_binary() {
    #[cfg(windows)]
    let name = "cmd";
    #[cfg(not(windows))]
    let name = "sh";

    let out = bin().args(["which", name]).output().unwrap();
    assert!(out.status.success());
    assert!(!stdout(&out).trim().is_empty());
}

#[test]
fn which_fails_for_unknown_binary() {
    let out = bin()
        .args(["which", "definitely-not-a-real-command-xyz"])
        .output()
        .unwrap();
    assert!(!out.status.success());
}

#[test]
fn env_prints_a_known_variable() {
    let out = bin()
        .args(["env", "TERMINION_TEST_VAR"])
        .env("TERMINION_TEST_VAR", "test-value")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(stdout(&out).trim(), "test-value");
}

#[test]
fn whoami_and_hostname_print_something() {
    let whoami_out = bin().arg("whoami").output().unwrap();
    assert!(whoami_out.status.success());
    assert!(!stdout(&whoami_out).trim().is_empty());

    let hostname_out = bin().arg("hostname").output().unwrap();
    assert!(hostname_out.status.success());
    assert!(!stdout(&hostname_out).trim().is_empty());
}

#[test]
fn date_prints_a_formatted_timestamp() {
    let out = bin().arg("date").output().unwrap();
    assert!(out.status.success());
    let printed = stdout(&out);
    // default format is "%Y-%m-%d %H:%M:%S"
    assert_eq!(printed.trim().len(), "2026-07-19 10:30:05".len());
}

#[test]
fn cd_changes_directory_for_the_current_process() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("sub")).unwrap();

    assert!(run_in(dir.path(), &["cd", "sub"]).status.success());
    assert!(!run_in(dir.path(), &["cd", "missing"]).status.success());
}

#[test]
fn shell_runs_commands_from_stdin_and_exits_cleanly() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_shell_in(dir.path(), "echo hello-from-shell\nexit\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("hello-from-shell"));
}

#[test]
fn shell_cd_persists_across_commands() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("sub/marker.txt"), b"x").unwrap();

    let out = run_shell_in(dir.path(), "cd sub\nls\nexit\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("marker.txt"));
}

#[test]
fn shell_reports_unknown_command_but_keeps_running() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_shell_in(dir.path(), "not-a-real-command\necho still-alive\nexit\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("still-alive"));
}

#[test]
fn shell_exits_cleanly_on_stdin_eof_without_exit_command() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_shell_in(dir.path(), "echo no-explicit-exit\n");
    assert!(out.status.success());
    assert!(stdout(&out).contains("no-explicit-exit"));
}
