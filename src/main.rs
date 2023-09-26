use crate::task_list::TaskList;
use crate::commands::command::Command;
use crate::commands::add_task_command::AddTaskCommand;
use crate::commands::command_interpreter::CommandInterpreter;
use crate::commands::list_tasks_command::ListTaskCommand;
use crate::commands::remove_task_command::RemoveTaskCommand;
use crate::commands::save_command::SaveCommand;
use crate::storage_adapters::file_adapter::FileAdapter;

pub mod commands {
    pub mod command_interpreter;
    pub mod command;
    pub mod add_task_command;
    pub mod list_tasks_command;
    pub mod remove_task_command;
    pub mod save_command;
}

pub mod storage_adapters {
    pub mod file_adapter;
}

pub mod tests {
    pub mod test_commands;
}

mod task_list;

fn main() {

    let file_loader = FileAdapter::new("B:\\source\\repos\\rust\\ToDoList\\test.dat");
    let mut task_list = file_loader.load_file().unwrap();
    let interpreter = CommandInterpreter::new();


    println!("Loaded {} tasks", task_list.tasks.len());
    task_list.clear(); // Clear because we're loading more commands

    interpreter.interpret_command(&mut task_list,"add_task buy eggs");
    interpreter.interpret_command(&mut task_list,"add_task buy milk");
    interpreter.interpret_command(&mut task_list,"add_task sell cheese");
    interpreter.interpret_command(&mut task_list,"list_tasks");
    interpreter.interpret_command(&mut task_list,"remove_task 5");
    interpreter.interpret_command(&mut task_list,"save");

    println!("Finished app exe");
}
