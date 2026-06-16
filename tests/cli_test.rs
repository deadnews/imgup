use std::process::Command;

fn imgup() -> Command {
    Command::new(env!("CARGO_BIN_EXE_imgup"))
}

#[test]
fn test_help() {
    let output = imgup().arg("--help").output().expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Upload images via APIs"));
    assert!(stdout.contains("--hosting"));
    assert!(stdout.contains("--format"));
    assert!(stdout.contains("--thumbnail"));
}

#[test]
fn test_version() {
    let output = imgup().arg("--version").output().expect("failed to run");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("imgup"));
}

#[test]
fn test_no_args_fails() {
    let output = imgup().output().expect("failed to run");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required"));
}

#[test]
fn test_nonexistent_file_fails() {
    let output = imgup()
        .arg("/tmp/nonexistent_image_12345.png")
        .output()
        .expect("failed to run");
    assert!(!output.status.success());
}

#[test]
fn test_missing_env_file_fails() {
    let output = imgup()
        .args(["--env-file", "/tmp/nonexistent_env_12345", "dummy.png"])
        .output()
        .expect("failed to run");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("env file"));
}

#[test]
fn test_hosting_values() {
    // Verify invalid hosting value is rejected
    let output = imgup()
        .args(["--hosting", "invalid", "dummy.png"])
        .output()
        .expect("failed to run");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid"));
}

#[test]
fn test_format_values() {
    // Verify invalid format value is rejected
    let output = imgup()
        .args(["--format", "invalid", "dummy.png"])
        .output()
        .expect("failed to run");
    assert!(!output.status.success());
}
