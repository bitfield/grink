use anyhow::Result;
use clap::Parser;
use futures::StreamExt;

use std::path::PathBuf;

use grink::{read_ignore_domains, scan, Status};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    ignore: Option<PathBuf>,
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut ok = 0;
    let mut warnings = 0;
    let mut errors = 0;
    let mut ignored = 0;

    let ignore_domains = if let Some(ignore_file) = args.ignore {
        read_ignore_domains(ignore_file)?
    } else {
        Vec::new()
    };
    let mut stream = Box::pin(scan(&args.paths, &ignore_domains)?);
    while let Some(link_result) = stream.next().await {
        let link = link_result?;
        match link.status {
            Status::OK => ok += 1,
            Status::Warning(_) => warnings += 1,
            Status::Error(_) => errors += 1,
            Status::Ignored => ignored += 1,
        }
        match link.status {
            Status::Error(_) => println!("{link}"),
            Status::OK | Status::Warning(_) | Status::Ignored if args.debug => println!("{link}"),
            Status::OK | Status::Warning(_) | Status::Ignored => {}
        }
    }
    println!(
        "Links: {} ({ok} OK, {errors} errors, {warnings} warnings, {ignored} ignored)",
        ok + warnings + errors + ignored
    );
    Ok(())
}
