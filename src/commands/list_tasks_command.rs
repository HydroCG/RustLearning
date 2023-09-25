use crate::commands::command::Command;
use crate::task_list::TaskList;

pub struct ListTaskCommand {

}

impl Command for ListTaskCommand {
    fn execute(&self, task_list: &mut TaskList, _args: &str) -> Result<bool, &str> {
        println!("Listing commands:");
        let mut idx = 0;

        for task in &task_list.tasks {
            idx += 1;
            println!("\t [Item {idx}]: {task}");
        }

        return Ok(true);
    }

    fn get_name(&self) -> &str {
        "list_tasks"
    }
}
