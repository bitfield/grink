use anyhow::Result;
use clap::Parser;

use std::path::PathBuf;

use grink::scan;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    for failure in scan(&args.paths).await? {
        println!("{failure}");
    }
    Ok(())
}
