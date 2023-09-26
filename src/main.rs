use crate::task_list::TaskList;
use crate::commands::command::Command;
use crate::commands::add_task_command::AddTaskCommand;
use crate::commands::list_tasks_command::ListTaskCommand;
use crate::commands::remove_task_command::RemoveTaskCommand;
use crate::commands::save_command::SaveCommand;
use crate::storage_adapters::file_adapter::FileAdapter;

pub mod commands {
    pub mod command;
    pub mod add_task_command;
    pub mod list_tasks_command;
    pub mod remove_task_command;
    pub mod save_command;
}

pub mod storage_adapters {
    pub mod file_adapter;
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

    let file_loader = FileAdapter::new("B:\\source\\repos\\rust\\ToDoList\\test.dat");
    let mut task_list = file_loader.load_file().unwrap();

    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(AddTaskCommand{}),
        Box::new(ListTaskCommand{}),
        Box::new(RemoveTaskCommand{}),
        Box::new(SaveCommand{}),
    ];

    println!("Loaded {} tasks", task_list.tasks.len());
    task_list.clear(); // Clear because we're loading more commands

    interpret_command(&mut task_list, &commands, "add_task buy eggs");
    interpret_command(&mut task_list, &commands, "add_task buy milk");
    interpret_command(&mut task_list, &commands, "add_task sell cheese");
    interpret_command(&mut task_list, &commands, "list_tasks");
    interpret_command(&mut task_list, &commands, "remove_task 5");
    interpret_command(&mut task_list, &commands, "save");

    println!("Finished app exe");
}
