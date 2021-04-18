mod config;
mod import;
mod sign;

use anyhow::Result;
use import::{import, Import};
use std::process::exit;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Sign {
    pub bar: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Import(Import),
    Sign(Sign),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "dahe")]
pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,
}

fn try_main() -> Result<()> {
    let args = Args::from_args();
    match args.command {
        Command::Import(ctx) => import(&ctx)?,
        _ => unimplemented!(),
    };

    Ok(())
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("error: {}", err.to_string());
        exit(1)
    }
}
