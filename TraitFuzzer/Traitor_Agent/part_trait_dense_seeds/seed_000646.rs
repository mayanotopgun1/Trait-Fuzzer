#![feature(specialization)]
#![allow(non_camel_case_types)]

pub type task_id = isize;

#[derive(PartialEq)]
pub enum Task {
    TaskHandle(task_id)
}

trait TaskTrait {
    fn get_task_id(&self) -> task_id;
}

default impl<T> TaskTrait for T {
    default fn get_task_id(&self) -> task_id {
        0
    }
}

impl TaskTrait for Task {
    fn get_task_id(&self) -> task_id {
        match self {
            Task::TaskHandle(id) => *id,
        }
    }
}

pub fn main() { }