use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Error};
use grink::UrlMatcher;

fn main() -> anyhow::Result<(), Error> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("Usage: grink [PATH, ...]");
        return Ok(());
    }
    let matcher = UrlMatcher::new();
    for path in paths {
        let file = BufReader::new(File::open(&path).context(format!("reading {path}"))?);
        for line_res in file.lines() {
            let line = line_res?;
            for url in matcher.urls(&line) {
                println!("{url}");
            }
        }
    }
    Ok(())
}
