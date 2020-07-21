use crate::cli::Cli;
use structopt::StructOpt;
use crate::cli::SubCommand;
use crate::context::Context;

pub mod cli;
pub mod commands;
pub mod context;
pub mod recipes;

#[derive(Debug)]
pub struct Wf2 {
    args: Vec<String>,
}

impl Wf2 {
    pub fn from_args<A>(args: A) -> Result<(Cli, Option<()>), anyhow::Error>
    where
        A: Iterator<Item = String>,
    {
        let args = args.collect::<Vec<String>>();




        let out: Cli = Cli::from_iter_safe(args)?;

        // ctx
        let mut ctx = Context::default();
        ctx.with_recipe(out.recipe.clone());

        // selected recipe
        let recipe = out.recipe.as_ref().map(|kinds| kinds.select());

        match (recipe.as_ref(), out.subcommand.as_ref()) {
            (Some(recipe), None) => {
                dbg!("recipe + NO args");
            }
            (Some(recipe), Some(SubCommand::SubCommand(args))) => {
                // dbg!("recipe + args");
                // dbg!(recipe);
                // dbg!(args);
                let mut args_with_bin = vec!["bin".to_string()];
                args_with_bin.extend(args.clone());
                let next = recipe.from_cli(&args_with_bin, &ctx)?;
            }
            (None, Some(cmd)) => {
                dbg!("no reipce");
                dbg!(cmd);
                // no recipe, need to infer
            }
            (Some(_), Some(cmd)) => {
                dbg!("recipe given + a subcommand");
                dbg!(cmd);
                // no recipe, need to infer
            }
            (None, None) => {
                dbg!("no recipe");
                dbg!("no subcommand");
                // no recipe, need to infer
            }
        };

        Ok((out, None))
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
