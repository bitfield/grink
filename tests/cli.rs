use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::cargo_bin("grink")
        .unwrap()
        .assert()
        .success()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_with_path_finds_urls_in_file() {
    Command::cargo_bin("grink")
        .unwrap()
        .args(["tests/data/haystack.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "https://www.youtube.com/watch?v=GmqeZl8OI2M",
        ))
        .stdout(predicate::str::contains(
            "https://slate.com/technology/2019/10/hello-world-history-programming.html",
        ));
}
