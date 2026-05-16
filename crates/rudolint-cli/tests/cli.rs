use assert_cmd::Command;

#[test]
fn emits_json_findings_for_stdin() {
    let mut cmd = Command::cargo_bin("rudolint").unwrap();
    cmd.args(["check", "--format", "json", "--failure-threshold", "error"])
        .write_stdin("FROM alpine:latest\nWORKDIR app\n")
        .assert()
        .failure()
        .stdout(predicates::str::contains("RDL3000"))
        .stdout(predicates::str::contains("RDL3007"));
}
