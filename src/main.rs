use crate::task_list::TaskList;
use crate::commands::add_task_command::AddTaskCommand;
use crate::commands::command::Command;
use crate::commands::list_tasks_command::ListTaskCommand;
use crate::commands::remove_task_command::RemoveTaskCommand;

pub mod commands {
    pub mod command;
    pub mod add_task_command;
    pub mod list_tasks_command;
    pub mod remove_task_command;
}

mod task_list;

fn interpret_command(task_list: &mut TaskList, commands: &Vec<Box<dyn Command>>, message: &str) {
    println!("> {}", message);

    for command in commands {
        if message.starts_with(command.get_name()) {
            match command.execute(task_list,&message[command.get_name().len()..].trim()) {
                Ok(_) => {},
                Err(e) => println!("Error: {}", e),
            }

            return;
        }
    }

    println!("Unknown command");
}

fn main() {

    let mut task_list = TaskList{ tasks: vec![] };

    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(AddTaskCommand{}),
        Box::new(ListTaskCommand{}),
        Box::new(RemoveTaskCommand{}),
    ];

    interpret_command(&mut task_list, &commands, "add_task buy eggs");
    interpret_command(&mut task_list, &commands, "add_task buy milk");
    interpret_command(&mut task_list, &commands, "add_task sell cheese");
    interpret_command(&mut task_list, &commands, "list_tasks");
    interpret_command(&mut task_list, &commands, "remove_task 5");
    interpret_command(&mut task_list, &commands, "list_tasks");
    interpret_command(&mut task_list, &commands, "piss");

    println!("Finished app exe");
}
