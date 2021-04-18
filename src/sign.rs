use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Sign {
    pub bar: Option<String>,
}
