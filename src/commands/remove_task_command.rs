use std::io::Error;
use crate::commands::command::Command;
use crate::task_list::TaskList;

pub struct RemoveTaskCommand {

}

impl Command for RemoveTaskCommand {

    fn execute(&self, task_list: &mut TaskList, args: &str) -> Result<bool, &str> {
        println!("Removing task: {}", args);

        let index = args.parse::<usize>().unwrap();

        if index > task_list.tasks.len() {
            return Err("Task index out of bounds");
        }

        task_list.remove_task(index-1);

        return Ok(true);
    }

    fn get_name(&self) -> &str {
        "remove_task"
    }
}