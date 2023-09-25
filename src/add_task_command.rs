use crate::command::Command;
use crate::task_list::TaskList;

pub struct AddTaskCommand {
}

impl Command for AddTaskCommand {
    fn execute(&self, task_list: &mut TaskList, args: &str) {
        task_list.add_task(&args);
    }

    fn get_name(&self) -> &str {
        "add_task"
    }
}