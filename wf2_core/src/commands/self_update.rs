use crate::context::Context;
use crate::task::{Task, TaskList};
use structopt::StructOpt;

/// This is REALLY nice
#[derive(StructOpt, Debug)]
pub struct SelfUpdate {
    #[structopt(long, short)]
    force: bool,
}

impl TaskList for SelfUpdate {
    fn to_task_list(&self, _ctx: &Context) -> Vec<Task> {
        vec![Task::Notify {
            message: "Hey from self-update!".to_string(),
        }]
    }
}
