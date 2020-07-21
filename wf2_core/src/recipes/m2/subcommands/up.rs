use crate::context::Context;
use crate::task::{Task, TaskList};
use structopt::StructOpt;

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
    fn to_task_list(&self, _ctx: &Context) -> Vec<Task> {
        vec![Task::Notify {
            message: String::from("hello from up"),
        }]
    }
}
