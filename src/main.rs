use crate::add_task_command::AddTaskCommand;
use crate::command::Command;
use crate::list_tasks_command::ListTaskCommand;
use crate::task_list::TaskList;

mod add_task_command;
mod command;
mod list_tasks_command;
mod task_list;

fn interpret_command(task_list: &mut TaskList, commands: &Vec<Box<dyn Command>>, message: &str) {

    for command in commands {
        if message.starts_with(command.get_name()) {
            command.execute(task_list,&message[command.get_name().len()..].trim());
            break;
        }
    }
}

fn main() {

    let mut task_list = TaskList{ tasks: vec![] };

    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(AddTaskCommand{}),
        Box::new(ListTaskCommand{})
    ];

    interpret_command(&mut task_list, &commands, "add_task buy eggs");
    interpret_command(&mut task_list, &commands, "add_task buy milk");
    interpret_command(&mut task_list, &commands, "add_task sell cheese");
    interpret_command(&mut task_list, &commands, "list_tasks");


    println!("Finished app exe");
}
