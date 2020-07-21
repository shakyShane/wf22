use structopt::StructOpt;
use crate::task::{TaskList, Task};
use crate::context::Context;

/// Take down the containers
#[derive(StructOpt, Debug, Clone)]
pub struct Down {
    #[structopt(short, long)]
    clean: bool,
}

impl TaskList for Down {
    fn to_task_list(&self, ctx: &Context) -> Vec<Task> {
        dbg!(self);
        dbg!(ctx);
        vec![]
    }
}
