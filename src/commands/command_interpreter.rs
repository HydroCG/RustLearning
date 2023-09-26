use crate::commands::add_task_command::AddTaskCommand;
use crate::commands::command::Command;
use crate::commands::list_tasks_command::ListTaskCommand;
use crate::commands::remove_task_command::RemoveTaskCommand;
use crate::commands::save_command::SaveCommand;
use crate::task_list::TaskList;

pub struct CommandInterpreter {
    commands: Vec<Box<dyn Command>>
}

impl CommandInterpreter {
    pub fn new() -> CommandInterpreter {

        CommandInterpreter{
            commands:vec![
                Box::new(AddTaskCommand{}),
                Box::new(ListTaskCommand{}),
                Box::new(RemoveTaskCommand{}),
                Box::new(SaveCommand{}),
            ]
        }
    }

    pub fn interpret_command(&self, task_list: &mut TaskList, message: &str) {
        println!("> {}", message);

        for command in &self.commands {
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
}