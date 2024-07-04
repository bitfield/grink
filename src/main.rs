use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{anyhow, Context, Error};
use regex::Regex;

const MARKDOWN_URL_PATTERN: &str =
    r#"(http://|https://)[a-z0-9]+([-.]{1}[a-z0-9]+)*(.[a-z]{2,5})?(:[0-9]{1,5})?(/.*)?"#;

fn main() -> anyhow::Result<(), Error> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        return Err(anyhow!("Usage: grink [PATH, ...]"));
    }
    let re = Regex::new(MARKDOWN_URL_PATTERN).unwrap();
    for path in paths {
        let file = BufReader::new(File::open(&path).context(format!("reading {path}"))?);
        for line_res in file.lines() {
            let line = line_res?;
            if let Some(urls) = re.captures(&line) {
                println!("{urls:?}");
            }
        }
    }
    Ok(())
}
