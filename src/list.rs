use crate::config;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct List {}

pub fn list() -> Result<()> {
    config::list_keys()?;
    Ok(())
}
