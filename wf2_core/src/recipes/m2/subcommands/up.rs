use structopt::StructOpt;
use crate::task::{TaskList, Task};
use crate::context::Context;

/// Bring up the containers
#[derive(StructOpt, Debug, Clone)]
pub struct Up {
    #[structopt(short, long)]
    attached: bool,
    #[structopt(short, long)]
    clean: bool,
    #[structopt(short, long)]
    build: bool,
    #[structopt(short, long)]
    sync: Option<Vec<std::path::PathBuf>>,
}

impl TaskList for Up {
    fn to_task_list(&self, ctx: &Context) -> Vec<Task> {
        dbg!(self);
        dbg!(ctx);
        vec![]
    }
}
