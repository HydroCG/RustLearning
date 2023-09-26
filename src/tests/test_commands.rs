use crate::commands::command_interpreter::CommandInterpreter;
use crate::task_list::TaskList;

fn run_command_test(command: &str) -> TaskList {

    let interpreter = CommandInterpreter::new();
    let mut task_list = TaskList::new();

    interpreter.interpret_command(&mut task_list, command);

    return task_list;
}

#[test]
fn test_adding_task() {
    let list = run_command_test("add_task");
    assert_eq!(list.tasks.len(), 1);
}
