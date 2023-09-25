pub struct TaskList {

    pub tasks: Vec<String>

}

impl TaskList {
    pub fn add_task(&mut self, task: &str) {
        println!("Adding task: {}", task);
        self.tasks.push(task.to_string());
    }
}
