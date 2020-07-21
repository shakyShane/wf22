use crate::context::Context;

#[derive(Debug)]
pub enum Task {
    Notify { message: String },
}

pub trait TaskList {
    fn to_task_list(&self, _ctx: &Context) -> Vec<Task> {
        vec![]
    }
}
