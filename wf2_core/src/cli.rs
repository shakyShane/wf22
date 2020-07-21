use crate::context::Context;
use structopt::StructOpt;
use crate::recipes::RecipeKinds;
use crate::commands::self_update::SelfUpdate;

pub trait CliCommand {
    fn from_cli(&self, args: &Vec<String>, ctx: &Context) -> Result<(), anyhow::Error>;
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(long, possible_values = &RecipeKinds::variants(), case_insensitive = true)]
    pub recipe: Option<RecipeKinds>,
    #[structopt(long)]
    pub dryrun: bool,
    #[structopt(long)]
    pub config: Option<std::path::PathBuf>,
    #[structopt(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    SelfUpdate(SelfUpdate),
    #[structopt(external_subcommand)]
    SubCommand(Vec<String>),
}
