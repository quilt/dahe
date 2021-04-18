use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Sign {
    pub bar: Option<String>,
}

pub fn sign(_: &Sign) -> Result<()> {
    unimplemented!()
}
