use std::process::Command;

#[test]
fn test_cli_help_flag() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--help"])
        .output()
        .expect("Failed to run cargo run -- --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Project Obsidian"),
        "Missing title: {}",
        stdout
    );
}

#[test]
fn test_cli_status_json() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--json", "status"])
        .output()
        .expect("Failed to run cargo run -- --json status");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // It should output valid JSON when --json is used
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("Status output was not valid JSON");

    assert!(parsed.get("os").is_some(), "JSON missing 'os' field");
    assert!(parsed.get("cpu").is_some(), "JSON missing 'cpu' field");
}

#[test]
fn test_cli_analyze_dry_run() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "analyze"])
        .output()
        .expect("Failed to run cargo run -- analyze");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("INITIATING NON-DESTRUCTIVE DRY-RUN ANALYSIS"));
}
