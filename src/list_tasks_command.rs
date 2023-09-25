use crate::command::Command;
use crate::task_list::TaskList;

pub struct ListTaskCommand {

}

impl Command for ListTaskCommand {
    fn execute(&self, task_list: &mut TaskList, args: &str) {
        println!("Listing tasks:");
        for task in &task_list.tasks {
            println!("\t{task}");
        }
    }

    fn get_name(&self) -> &str {
        "list_tasks"
    }
}
