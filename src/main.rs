use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Error};
use grink::UrlMatcher;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() -> anyhow::Result<(), Error> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("Usage: grink [PATH, ...]");
        return Ok(());
    }
    let matcher = UrlMatcher::new();
    let http = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .build()?;
    paths
        .par_iter()
        .map(|path| {
            let file = BufReader::new(File::open(path).context(format!("reading {path}"))?);
            for line_res in file.lines() {
                let line = line_res?;
                matcher.urls(&line).par_iter().for_each(|url| {
                    match http.get(*url).send() {
                        Err(e) => {
                            println!("{path}: {e}");
                        }
                        Ok(resp) => {
                            if let Err(e) = resp.error_for_status() {
                                println!("{path}: {e}");
                            }
                        }
                    }
                });
            }
            anyhow::Ok(())
        })
        .collect::<Result<(), Error>>()
}
