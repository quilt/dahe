mod config;
mod hex;
mod import;
mod list;
mod sign;

use crate::import::{import, Import};
use crate::list::{list, List};
use crate::sign::{sign, Sign};
use anyhow::Result;
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dahe")]
pub enum Args {
    Import(Import),
    #[structopt(alias = "ls")]
    List(List),
    Sign(Sign),
}

fn try_main() -> Result<()> {
    match Args::from_args() {
        Args::Import(ctx) => import(&ctx),
        Args::List(_) => list(),
        Args::Sign(ctx) => sign(&ctx),
    }
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error: {}", err.to_string());
        exit(1);
    }
}
