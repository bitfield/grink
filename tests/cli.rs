use std::{fs, net::TcpListener};

use assert_cmd::{assert::OutputAssertExt, cargo, Command};
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::new(cargo::cargo_bin!("grink"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_checks_urls_in_file() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    drop(listener); // this port is now not listening
    let tmp = TempDir::new().unwrap();
    let haystack = tmp.path().join("haystack.md");
    fs::write(
        &haystack,
        format!("Test link: [local test server](http://{addr}/)"),
    )
    .unwrap();
    Command::new(cargo::cargo_bin!("grink"))
        .args([haystack])
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains(addr))
        .stdout(predicate::str::contains(
            "Links: 1 (0 OK, 1 errors, 0 warnings, 0 ignored)",
        ));
}

#[test]
fn binary_ignores_domains_in_ignore_file() {
    let tmp = TempDir::new().unwrap();
    let ignore_file = tmp.path().join("ignore.txt");
    fs::write(&ignore_file, "bogus.com").unwrap();
    let haystack = tmp.path().join("haystack.md");
    fs::write(
        &haystack,
        "Test link: [local test server](http://bogus.com/)",
    )
    .unwrap();
    Command::new(cargo::cargo_bin!("grink"))
        .args([
            "--ignore",
            ignore_file.to_str().unwrap(),
            haystack.to_str().unwrap(),
        ])
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Links: 1 (0 OK, 0 errors, 0 warnings, 1 ignored)",
        ));
}
