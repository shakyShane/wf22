use structopt::{StructOpt, clap, clap::AppSettings};
use crate::recipes::m2::subcommands::M2Subcommands;
use crate::cli::CliCommand;
use anyhow::Error;
use crate::context::Context;

pub mod subcommands;

#[derive(StructOpt, Debug)]
pub struct M2 {
    #[structopt(subcommand)]
    subcommand: M2Subcommands
}

impl CliCommand for M2 {
    fn from_ctx(args: &Vec<String>, ctx: &Context) -> Result<(), Error> {
        let mut bin = vec!["m2".to_string()];
        bin.extend(args.clone());
        match M2::from_iter_safe(bin) {
            Ok(m2) => {
                dbg!(m2);
            },
            Err(e) => {
                println!("{}", e);
            }
        };
        Ok(())
    }
}

