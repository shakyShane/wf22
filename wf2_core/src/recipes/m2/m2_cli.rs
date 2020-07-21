use crate::cli::{CliCommand, GlobalSubcommands};
use crate::context::Context;
use crate::recipes::m2::subcommands::M2Subcommands;
use crate::recipes::m2::M2;
use crate::recipes::RecipeKinds;
use crate::task::TaskList;
use anyhow::*;

use crate::recipes::m2::subcommands::composer::Composer;
use crate::task::Task;
use anyhow::Error;
use structopt::StructOpt;

impl CliCommand for M2 {
    fn from_cli(&self, args: &Vec<String>, ctx: &Context) -> Result<Vec<Task>, Error> {
        global_cli!();
        append_sub!(M2Subcommands, GlobalSubcommands);
        let cmd = Cli::from_iter_safe(args)?;
        match cmd.subcommand {
            Some(Subcommands::M2Subcommands(M2Subcommands::PassThru(args))) => {
                let first = args.get(0).map(|x| x.as_str());
                match first {
                    Some("pm2") => todo!("pm2 not implemented yet {:?}", args),
                    Some("composer") => Ok(Composer::new(args).to_task_list(&ctx)),
                    _ => todo!("pass thru is not implemented yet {:?}", args),
                }
            }
            Some(Subcommands::M2Subcommands(m2cmd)) => Ok(m2cmd.to_task_list(&ctx)),
            _ => Err(anyhow!("no-subcommand is not implemented yet for m2")),
        }
    }
}
