use crate::context::Context;
use crate::task::{Task, TaskList};

pub struct Composer {
    pub args: Vec<String>,
}

impl Composer {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

impl TaskList for Composer {
    fn to_task_list(&self, _ctx: &Context) -> Vec<Task> {
        dbg!("args");
        dbg!(&self.args);
        vec![Task::Notify {
            message: String::from("hello from composer"),
        }]
    }
}
