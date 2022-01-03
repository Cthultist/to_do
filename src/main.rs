use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

#[derive(Debug)]
struct Task {
    description: String,
    state: TaskState,
}

impl Task {}

#[derive(Debug)]
enum TaskState {
    Incomplete,
    InProgress,
    Complete,
}
impl std::str::FromStr for TaskState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Complete" => Ok(TaskState::Complete),
            "InProgress" => Ok(TaskState::InProgress),
            "Incomplete" => Ok(TaskState::Incomplete),
            _ => Err(format!("'{}' is not a valid state", s)),
        }
    }
}
fn read(input: &mut String) {
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read");
}

fn main() {
    println!("To-Do list of sorts!");
    println!("-----------------------");

    loop {
        let mut action = String::new();
        let mut task_input = String::new();
        let mut state = String::new();
        let mut tasks = HashMap::<String, Task>::new();

        print!("Are you adding, removing or modifying a task? (add, rem, mod): ");
        read(&mut action);

        print!("What is the name of the Task?: ");
        read(&mut task_input);

        print!("What is the state of the task? i.e. Incomplete, InProgress, or Complete?: ");
        read(&mut state);

        println!("this is the action: {}", action.trim());
        println!("this is the task name: {}", task_input.trim());
        println!("this is the state: {}", state.trim());

        if action.trim() == "quit" {
            panic!()
        }
        if action.trim() == "add" || action.trim() == "mod" {
            let ts = state.trim();
            let ti: String = task_input.trim().to_string();
            let task = Task {
                description: ti,
                state: TaskState::from_str(&ts).unwrap(),
            };
            tasks.insert(task_input, task);
            //TODO: Print list of current tasks
            //TODO: Print "Task successfully added"
            for (key, value) in &tasks {
                println!("{}: {:?}", key, value);
            }
            println!("Task Successfully Added!");
        } else if action.trim() == "rem" {
            tasks.remove(&task_input.clone());
            //TODO: Print list of current tasks
            //TODO: print "Task successfully removed"
            for (key, value) in &tasks {
                println!("{}: {:?}", key, value);
            }
            println!("Task Successfully Removed!");
        }
    }
}
