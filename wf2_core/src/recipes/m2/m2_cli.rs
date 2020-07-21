use crate::cli::CliCommand;
use crate::context::Context;
use crate::recipes::m2::subcommands::M2Subcommands;
use crate::recipes::m2::M2;
use anyhow::Error;
use structopt::StructOpt;

impl CliCommand for M2 {
    fn from_cli(&self, args: &Vec<String>, ctx: &Context) -> Result<(), Error> {
        let cmd = M2Cli::from_iter_safe(args)?;
        dbg!(cmd);
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub struct M2Cli {
    #[structopt(subcommand)]
    subcommand: M2Subcommands,
}
