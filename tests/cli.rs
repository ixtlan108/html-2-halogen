use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_parse_html() {
    let mut cmd = Command::cargo_bin("html-2-halogen").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}
