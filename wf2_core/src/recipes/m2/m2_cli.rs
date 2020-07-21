use crate::cli::{CliCommand, GlobalSubcommands};
use crate::context::Context;
use crate::task::TaskList;
use crate::recipes::m2::subcommands::M2Subcommands;
use crate::recipes::m2::M2;
use crate::recipes::RecipeKinds;
use anyhow::Error;
use structopt::StructOpt;

impl CliCommand for M2 {
    fn from_cli(&self, args: &Vec<String>, _ctx: &Context) -> Result<(), Error> {
        global_cli!();
        append_sub!(M2Subcommands, GlobalSubcommands);
        let cmd = Cli::from_iter_safe(args)?;
        dbg!(cmd);
        Ok(())
    }
}
