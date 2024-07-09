use std::{fs, net::TcpListener};

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::cargo_bin("grink")
        .unwrap()
        .assert()
        .success()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_checks_urls_in_file() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    drop(listener); // this port is now not listening
    let tmp = TempDir::new().unwrap();
    let haystack = tmp.path().with_file_name("haystack.md");
    fs::write(
        &haystack,
        format!("Test link: [local test server](http://{addr}/)"),
    )
    .unwrap();
    Command::cargo_bin("grink")
        .unwrap()
        .args([haystack])
        .assert()
        .success()
        .stdout(predicate::str::contains(addr))
        .stdout(predicate::str::ends_with(
            "1 files scanned, 1 links found, 0 OK, 1 broken\n",
        ));
}
