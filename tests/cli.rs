use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn default_runs_successfully() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();
    cmd.assert().success();
}

#[test]
fn generates_correct_number_of_passwords() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();

    let output = cmd
        .args(["--count", "3"])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.lines().count(), 3);
}

#[test]
fn rejects_invalid_length_with_require_each_class() {
    let mut cmd = Command::cargo_bin("passgen").unwrap();

    cmd.args(["--length", "2", "--require-each-class"])
        .assert()
        .failure()
        .stderr(contains("Password length must be at least"));
}
