use crate::config::Config;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct List {}

pub fn list() -> Result<()> {
    let config = Config::open()?;
    config.print();
    Ok(())
}
