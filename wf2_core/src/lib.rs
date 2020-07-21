use structopt::{StructOpt, clap, clap::AppSettings};
use crate::cli::{Cli, Sub, CliCommand};
use crate::recipes::m2::M2;
use crate::context::Context;

pub mod cli;
pub mod recipes;
pub mod context;

#[derive(Debug)]
pub struct Wf2 {
    args: Vec<String>
}

impl Wf2 {
    pub fn from_args(args: impl Iterator<Item = String>) -> Result<(), anyhow::Error> {
        let args = args.collect::<Vec<String>>();
        match Cli::from_iter_safe(args) {
            Ok(Cli { sub: None, dryrun, recipe, config }) => {
                println!("NO COMMAND GIVEN");
                dbg!(dryrun);
                dbg!(recipe);
                dbg!(config);
            },
            Ok(Cli { sub: Some(Sub::SubCmd(sub_args)), dryrun, recipe, config }) => {
                let ctx = Context::default();
                let r = M2::from_ctx(&sub_args, &ctx);
                dbg!("r={}",r);
            },
            Err(clap::Error {
                    kind: clap::ErrorKind::HelpDisplayed,
                    message,
                    info
                }) => println!("{}", message),
            Err(clap::Error {
                    kind: clap::ErrorKind::VersionDisplayed,
                    message,
                    info,
                }) => println!("{}", message),
            Err(clap::Error {
                    kind: clap::ErrorKind::UnrecognizedSubcommand,
                    info,
                    message
                }) => eprintln!("not help or version->\n{:#?}", info),
            e @ _ => {
                dbg!(e);
                unimplemented!();
            }
        };
        Ok(())
    }
}

