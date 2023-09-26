use crate::commands::command::Command;
use crate::storage_adapters::file_adapter::FileAdapter;
use crate::task_list::TaskList;

pub struct SaveCommand {
}

impl Command for SaveCommand {
    fn execute(&self, task_list: &mut TaskList, args: &str) -> Result<bool, &str> {

        let file_adapter = FileAdapter::new("B:\\source\\repos\\rust\\ToDoList\\test.dat");

        // run save_file and return the result
        match file_adapter.save_file(task_list) {
            Ok(_) => {
                println!("Saved file successfully");
                return Ok(true);
            },
            Err(errMsg) => {
                return Err("err");
            }
        }
    }

    fn get_name(&self) -> &str {
        "save"
    }
}