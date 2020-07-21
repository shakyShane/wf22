use crate::cli::Cli;
use structopt::{StructOpt, clap};
use crate::cli::SubCommand;
use crate::context::Context;
use anyhow::*;

pub mod cli;
pub mod commands;
pub mod context;
pub mod recipes;

#[derive(Debug)]
pub struct Wf2 {
    args: Vec<String>,
}

impl Wf2 {
    pub fn from_args<A>(args: A, help_requested: bool) -> Result<(Cli, Option<()>), anyhow::Error>
    where
        A: Iterator<Item = String>,
    {
        let args = args.collect::<Vec<String>>();

        let output = match Cli::from_iter_safe(&args) {
            Err(e @ clap::Error {
                kind: clap::ErrorKind::HelpDisplayed,
                ..
            }) => {
                let without: Vec<String> = args
                    .into_iter()
                    .filter(|arg| &arg[..] != "--help")
                    .filter(|arg| &arg[..] != "-h")
                    .collect();
                if help_requested {
                    return Err(anyhow!("{}", e));
                }
                return Wf2::from_args(without.into_iter(), true);
            }
            Err(e) => Err(anyhow!("{}", e)),
            Ok(cli) => {
                // create the context for recipes
                let mut ctx = Context::default();
                ctx.with_recipe(cli.recipe.clone());

                // selected recipe
                let recipe = cli.recipe.as_ref().map(|kinds| kinds.select());

                match (recipe.as_ref(), cli.subcommand.as_ref()) {
                    (Some(recipe), None) => {
                        // dbg!("recipe + args");
                        // dbg!(recipe);
                        // dbg!(args);
                        let mut args_with_bin = vec!["wf2".to_string()];
                        if help_requested {
                            args_with_bin.push("--help".to_string());
                        }
                        let next = recipe.from_cli(&args_with_bin, &ctx)?;
                        println!("here");
                        dbg!(next);
                        Ok((cli, None))
                    }
                    (Some(recipe), Some(SubCommand::SubCommand(args))) => {
                        // dbg!("recipe + args");
                        // dbg!(recipe);
                        // dbg!(args);
                        let mut args_with_bin = vec!["wf2".to_string()];
                        args_with_bin.extend(args.clone());
                        let next = recipe.from_cli(&args_with_bin, &ctx)?;
                        println!("here");
                        dbg!(next);
                        Ok((cli, None))
                    }
                    (None, Some(SubCommand::SelfUpdate(self_update))) => {
                        dbg!("SelfUpdate");
                        // no recipe, need to infer
                        Ok((cli, None))
                    }
                    (None, Some(cmd)) => {
                        dbg!("SelfUpdate");
                        // no recipe, need to infer
                        Ok((cli, None))
                    }
                    (Some(_), Some(cmd)) => {
                        dbg!("recipe given + a random subcommand");
                        dbg!(cmd);
                        // no recipe, need to infer
                        Err(anyhow!("Subcommand not recognised {:?}", cmd))
                    }
                    (None, None) => {
                        let mut args = args.clone();
                        if help_requested {
                            args.push("--help".into());
                        }
                        Wf2::from_args(args.into_iter(), true)
                    }
                }
            }
        }?;

        Ok(output)
    }
}

// fn from_struct_opt<T: StructOpt>(args: Vec<String>) -> Result<T, Error> {
//     match T::from_iter_safe(args) {
//         Ok(t) => Ok(t),
//         Err(clap::Error {
//                 kind: clap::ErrorKind::HelpDisplayed,
//                 message,
//                 info: _,
//             }) => eprintln!("{}", message),
//         Err(clap::Error {
//                 kind: clap::ErrorKind::VersionDisplayed,
//                 message,
//                 info: _,
//             }) => eprintln!("{}", message),
//         Err(clap::Error {
//                 message,
//                 ..
//             }) => eprintln!("{}", message),
//     }
// }
