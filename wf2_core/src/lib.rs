macro_rules! global_cli {
    () => {
        #[derive(StructOpt, Debug)]
        pub struct Cli {
            #[structopt(long, possible_values = &RecipeKinds::variants(), case_insensitive = true)]
            pub recipe: Option<RecipeKinds>,
            #[structopt(long)]
            pub dryrun: bool,
            #[structopt(long)]
            pub config: Option<std::path::PathBuf>,
            #[structopt(subcommand)]
            pub subcommand: Option<Subcommands>,
        }
    };
}

macro_rules! append_sub {
    ($ident:ident) => {
         #[derive(StructOpt, Debug)]
        pub enum Subcommands {
            #[structopt(flatten)]
            $ident($ident)
        }
    };
    ($($ident:ident),*) => {
        #[derive(StructOpt, Debug)]
        pub enum Subcommands {
            $(
                #[structopt(flatten)]
                $ident($ident)
            ),*
        }

        impl Subcommands {
            pub fn select(self) -> Box<dyn TaskList> {
                match self {
                    $(
                        Self::$ident(inner) => Box::new(inner),
                    )*
                }
            }
        }
    };
}

use crate::cli::GlobalSubcommands;
use crate::cli::*;
use crate::context::Context;
use anyhow::*;
use structopt::{clap, StructOpt};
use crate::task::TaskList;

pub mod cli;
pub mod commands;
pub mod context;
pub mod recipes;
pub mod task;

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
            Err(
                e
                @
                clap::Error {
                    kind: clap::ErrorKind::HelpDisplayed,
                    ..
                },
            ) => {
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

                // recipe
                let mut next_args = args.clone();

                if help_requested {
                    next_args.push("--help".to_string());
                }

                match (&cli.recipe, &cli.subcommand) {
                    (
                        Some(recipe_kind),
                        Some(cli::Subcommands::GlobalSubcommands(GlobalSubcommands::PassThru(
                            pass_thru_args,
                        ))),
                    ) => {
                        dbg!(pass_thru_args);
                        let mut with_bin = vec!["wf2".to_string()];
                        with_bin.extend(pass_thru_args.clone());
                        let recipe = recipe_kind.select();
                        let next = recipe.from_cli(&with_bin, &ctx)?;
                        dbg!(next);
                    }
                    (Some(_), Some(cli::Subcommands::GlobalSubcommands(none_pass_thru))) => {
                        if help_requested {
                            return Wf2::from_args(next_args.into_iter(), true);
                        }
                        dbg!("has recipe, but command is global");
                        dbg!(none_pass_thru);
                    }
                    (None, Some(cli::Subcommands::GlobalSubcommands(cmd))) => {
                        if help_requested {
                            return Wf2::from_args(next_args.into_iter(), true);
                        }
                        let tasks = cmd.to_task_list(&ctx);
                        dbg!("here");
                    }
                    (Some(recipe_kind), None) => {
                        let recipe = recipe_kind.select();
                        let _next = recipe.from_cli(&next_args, &ctx)?;
                        dbg!("recipe, no command");
                        dbg!(recipe);
                    }
                    (None, None) => {
                        if help_requested {
                            return Wf2::from_args(next_args.into_iter(), true);
                        }
                        dbg!("no recipe or command");
                    }
                }

                Ok((cli, None))
            }
        }?;

        Ok(output)
    }
}
