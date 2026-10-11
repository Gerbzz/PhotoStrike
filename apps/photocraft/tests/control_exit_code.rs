//! `--control` setup failures must exit non-zero (#1816): a launcher that sees
//! status 0 believes the control server is running when it is not.
//!
//! These tests run the real binary as a subprocess and assert the exit code,
//! covering both failure paths in `main.rs`: control authentication and the
//! automation workspace.

/// A syntactically valid 64-hex-char token, so the workspace test reaches the
/// workspace check instead of failing on token validation first.
const VALID_TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

/// Run `photostrike` with the given arguments in an isolated config dir and
/// return the exit status and stderr. Each call gets its own config dir (tests run in
/// parallel), and no display variables, so a regression fails fast instead of opening a window.
fn run_photocraft(args: &[&str]) -> (std::process::ExitStatus, String) {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("photocraft-control-exit-{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_photostrike"))
        .args(args)
        .env("PHOTOSTRIKE_CONFIG_DIR", &dir)
        // Isolate from the host environment so the test is reproducible.
        .env_remove("PHOTOSTRIKE_CONTROL_PORT")
        .env_remove("PHOTOSTRIKE_CONTROL_TOKEN")
        .env_remove("PHOTOSTRIKE_CONTROL_TOKEN_FILE")
        .env_remove("PHOTOSTRIKE_AUTOMATION_READ_ROOT")
        .env_remove("PHOTOSTRIKE_AUTOMATION_WRITE_ROOT")
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .output()
        .expect("failed to run photocraft binary");
    std::fs::remove_dir_all(&dir).ok();
    (output.status, String::from_utf8_lossy(&output.stderr).into_owned())
}

#[test]
fn control_without_explicit_credentials_exits_with_status_2() {
    let (status, stderr) = run_photocraft(&["--control", "0"]);
    assert_eq!(status.code(), Some(2), "control without credentials must exit 2, stderr: {stderr}");
    assert!(stderr.contains("requires --control-token or --control-token-file"), "stderr: {stderr}");
}

#[test]
fn invalid_control_token_exits_with_status_2() {
    // `validate_token` rejects a token that is not exactly 64 hex characters.
    let (status, stderr) = run_photocraft(&["--control", "0", "--control-token", "invalid"]);
    assert_eq!(status.code(), Some(2), "invalid control token must exit 2, stderr: {stderr}");
    assert!(stderr.contains("cannot configure control authentication"), "stderr: {stderr}");
}

#[test]
fn both_token_and_token_file_exits_with_status_2() {
    // `server_token` rejects the combination of a supplied token and a token file.
    let dir = std::env::temp_dir().join(format!("photocraft-token-file-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let token_file = dir.join("token.txt");
    std::fs::write(&token_file, VALID_TOKEN).unwrap();
    let (status, stderr) = run_photocraft(&["--control", "0", "--control-token", VALID_TOKEN, "--control-token-file", token_file.to_str().unwrap()]);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(status.code(), Some(2), "token + token file must exit 2, stderr: {stderr}");
    assert!(stderr.contains("cannot configure control authentication"), "stderr: {stderr}");
}

#[test]
fn control_without_either_explicit_root_exits_with_status_2() {
    let (status, stderr) = run_photocraft(&["--control", "0", "--control-token", VALID_TOKEN]);
    assert_eq!(status.code(), Some(2), "control without roots must exit 2, stderr: {stderr}");
    assert!(stderr.contains("requires explicit --automation-read-root and --automation-write-root"), "stderr: {stderr}");
    assert!(!stderr.contains(VALID_TOKEN), "secret leaked: {stderr}");
}

#[test]
fn control_with_only_one_root_exits_with_status_2() {
    let root = std::env::temp_dir();
    let root = root.to_str().unwrap();
    for flag in ["--automation-read-root", "--automation-write-root"] {
        let (status, stderr) = run_photocraft(&["--control", "0", "--control-token", VALID_TOKEN, flag, root]);
        assert_eq!(status.code(), Some(2), "control with only {flag} must exit 2, stderr: {stderr}");
        assert!(stderr.contains("requires explicit --automation-read-root and --automation-write-root"), "stderr: {stderr}");
        assert!(!stderr.contains(VALID_TOKEN), "secret leaked: {stderr}");
    }
}

#[test]
fn missing_caller_supplied_token_file_is_not_generated() {
    let path = std::env::temp_dir().join(format!("photostrike-missing-token-{}-{}", std::process::id(), std::thread::current().name().unwrap_or("test")));
    let _ = std::fs::remove_file(&path);
    let (status, stderr) = run_photocraft(&["--control", "0", "--control-token-file", path.to_str().unwrap()]);
    assert_eq!(status.code(), Some(2), "missing token file must exit 2, stderr: {stderr}");
    assert!(!path.exists(), "a control secret was generated without its caller");
}

#[test]
fn missing_automation_read_root_exits_with_status_2() {
    // `AuthorizedWorkspace::new` fails when the read root does not exist.
    let (status, stderr) = run_photocraft(&["--control", "0", "--control-token", VALID_TOKEN, "--automation-read-root", "/nonexistent/photocraft-test-path"]);
    assert_eq!(status.code(), Some(2), "missing read root must exit 2, stderr: {stderr}");
    assert!(stderr.contains("cannot configure automation workspace"), "stderr: {stderr}");
}

#[test]
fn missing_automation_write_root_exits_with_status_2() {
    // `AuthorizedWorkspace::new` fails when the write root does not exist.
    let (status, stderr) = run_photocraft(&["--control", "0", "--control-token", VALID_TOKEN, "--automation-write-root", "/nonexistent/photocraft-test-path"]);
    assert_eq!(status.code(), Some(2), "missing write root must exit 2, stderr: {stderr}");
    assert!(stderr.contains("cannot configure automation workspace"), "stderr: {stderr}");
}
