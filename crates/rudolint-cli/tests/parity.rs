use std::process::Command;

#[test]
#[ignore = "requires a pinned external oracle binary"]
fn parity_oracle_is_available() {
    let output = Command::new("hadolint")
        .arg("--version")
        .output()
        .expect("hadolint must be installed for parity tests");
    assert!(output.status.success());
}
