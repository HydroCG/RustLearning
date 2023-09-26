use crate::commands::command::Command;
use crate::task_list::TaskList;

pub struct RemoveTaskCommand {

}

impl Command for RemoveTaskCommand {

    fn execute(&self, task_list: &mut TaskList, args: &str) -> Result<bool, String> {
        println!("Removing task: {}", args);

        let index = args.parse::<usize>().unwrap();

        return task_list.remove_task(index-1);
    }

    fn get_name(&self) -> &str {
        "remove_task"
    }
}