use crate::commands::self_update::SelfUpdate;
use crate::context::Context;
use crate::recipes::RecipeKinds;
use structopt::StructOpt;
use crate::task::{TaskList, Task};

pub trait CliCommand {
    fn from_cli(&self, args: &Vec<String>, ctx: &Context) -> Result<(), anyhow::Error>;
}

global_cli!();
append_sub!(GlobalSubcommands);

#[wf2_derive::task_list]
#[derive(Debug, StructOpt)]
pub enum GlobalSubcommands {
    SelfUpdate(SelfUpdate),
    #[structopt(external_subcommand)]
    PassThru(Vec<String>),
}
