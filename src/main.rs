mod config;
mod import;
mod sign;

use anyhow::Result;
use import::{import, Import};
use sign::{sign, Sign};
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dahe")]
pub enum Args {
    Import(Import),
    Sign(Sign),
}

fn try_main() -> Result<()> {
    match Args::from_args() {
        Args::Import(ctx) => import(&ctx),
        Args::Sign(ctx) => sign(&ctx),
    }
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error: {}", err.to_string());
        exit(1)
    }
}
