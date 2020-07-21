use crate::context::Context;
use crate::task::{Task, TaskList};
use structopt::StructOpt;

pub mod composer;
pub mod down;
pub mod up;

#[wf2_derive::task_list]
#[derive(StructOpt, Debug)]
pub enum M2Subcommands {
    Up(up::Up),
    Down(down::Down),
    /// Pass through commands like `wf2 composer install -vvv`
    #[structopt(external_subcommand)]
    PassThru(Vec<String>),
}
