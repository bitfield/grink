use anyhow::Result;
use clap::Parser;

use std::path::PathBuf;

use grink::{scan, Status};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut ok = 0;
    let mut warnings = 0;
    let mut errors = 0;
    for link in scan(&args.paths).await? {
        match link.status {
            Status::OK => ok += 1,
            Status::Warning(_) => warnings += 1,
            Status::Error(_) => errors += 1,
            Status::Skipped => {}
        }
        match link.status {
            Status::Error(_) => println!("{link}"),
            Status::OK | Status::Warning(_) | Status::Skipped if args.debug => println!("{link}"),
            Status::OK | Status::Warning(_) | Status::Skipped => {}
        }
    }
    println!(
        "Links: {} ({ok} OK, {errors} errors, {warnings} warnings)",
        ok + warnings + errors
    );
    Ok(())
}
