use structopt::StructOpt;
use crate::task::{TaskList, Task};
use crate::context::Context;

/// This is REALLY nice
#[derive(StructOpt, Debug)]
pub struct SelfUpdate {
    #[structopt(long, short)]
    force: bool,
}

impl TaskList for SelfUpdate {
    fn to_task_list(&self, ctx: &Context) -> Vec<Task> {
        dbg!(self);
        dbg!(ctx);
        vec![]
    }
}
