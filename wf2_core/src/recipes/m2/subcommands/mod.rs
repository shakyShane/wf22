use structopt::StructOpt;
use crate::task::{TaskList, Task};
use crate::context::Context;

pub mod up;
pub mod down;

#[wf2_derive::task_list]
#[derive(StructOpt, Debug)]
pub enum M2Subcommands {
    Up(up::Up),
    Down(down::Down),
    /// Pass through commands like `wf2 composer install -vvv`
    #[structopt(external_subcommand)]
    PassThru(Vec<String>),
}
