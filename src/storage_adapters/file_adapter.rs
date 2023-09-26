use crate::task_list::TaskList;

// This file is not done properly... I don't think.as
// Need to work on better result handling for save file

pub struct FileAdapter {
    pub file_path: String,
}

impl FileAdapter {
    pub fn new(file_path: &str) -> FileAdapter {
        FileAdapter {
            file_path: file_path.to_string(),
        }
    }

    pub fn save_file(&self, task_list: &mut TaskList) -> Result<bool, &str> {
        let mut contents = String::new();

        for task in &task_list.tasks {
            contents.push_str(&task);
            contents.push_str("\n");
        }

        return match std::fs::write(&self.file_path, contents) {
            Ok(_) => {
                Ok(true)
            },
            Err(str) => {
                Err("Unable to write file to at the specified path")
            }
        }
    }

    pub fn load_file(&self) -> Result<TaskList, String> {

        let mut taskList = TaskList { tasks: Vec::new() };

        match std::fs::read_to_string(&self.file_path) {
            Ok(contents) => {

                let lines = contents.lines();

                for line in lines {
                    taskList.add_task(line);
                }
            },
            Err(str) => {

            }
        }

        return Ok(taskList);
    }
}