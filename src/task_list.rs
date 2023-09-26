pub struct TaskList {

    pub tasks: Vec<String>

}

impl TaskList {

    pub fn new() -> TaskList {
        TaskList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: &str) {
        println!("Adding task: {}", task);
        self.tasks.push(task.to_string());
    }

    pub fn remove_task(&mut self, task_index: usize) -> Result<bool, String> {

        if task_index > self.tasks.len() {
            return Err(String::from("Task index out of bounds"));
        }

        self.tasks.remove(task_index);
        return Ok(true);
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}

