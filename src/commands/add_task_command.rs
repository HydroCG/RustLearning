use crate::commands::command::Command;
use crate::task_list::TaskList;

pub struct AddTaskCommand {
}

impl Command for AddTaskCommand {
    fn execute(&self, task_list: &mut TaskList, args: &str) -> Result<bool, String> {
        task_list.add_task(&args);

        return Ok(true);
    }

    fn get_name(&self) -> &str {
        "add_task"
    }
}