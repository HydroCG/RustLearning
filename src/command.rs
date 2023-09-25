use crate::task_list::TaskList;

pub trait Command {
    fn execute(&self, task_list: &mut TaskList, args: &str);
    fn get_name(&self) -> &str;
}
