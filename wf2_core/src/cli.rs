use structopt::{StructOpt, clap, clap::AppSettings};
use crate::context::Context;

pub trait CliCommand {
    fn from_ctx(args: &Vec<String>, ctx: &Context) -> Result<(), anyhow::Error>;
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(long)]
    pub recipe: Option<String>,
    #[structopt(long)]
    pub dryrun: bool,
    #[structopt(long)]
    pub config: Option<std::path::PathBuf>,
    #[structopt(subcommand)]
    pub sub: Option<Sub>
}

#[derive(Debug, StructOpt)]
pub enum Sub {
    #[structopt(external_subcommand)]
    SubCmd(Vec<String>)
}
