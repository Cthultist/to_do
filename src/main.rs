use std::io::{stdin, stdout, Write};
use std::collections::HashMap;

fn read(input: &mut String) {
    stdout().flush()
        .expect("Failed to flush");
    stdin().read_line(input)
        .expect("Failed to read");
}

fn main() {
    println!("To-Do list of sorts!");
    println!("-----------------------");

    loop {
        let mut action = String::new();
        let mut task = String::new();
        let mut state = String::new();
        let mut task_list: HashMap<&str, TaskState> = Task::new();

        print!("Are you adding, removing or modifying a task? (add, rem, mod" );
        read(&mut action);

        print!("What is the name of the Task? ");
        read(&mut task);

        print!("What is the state of the task? i.e. Incomplete, InProgress, or Complete? ");
        read(&mut state);

        let action: String = action.to_string();
        let task: String = action.to_string();
        let state: String = state.to_string();

        if action == "add" {
            task.add.save(task);
                match task.save() {
                    Ok(_) => println!("Successfully Added"),
                    Err(why) => println!("An Error Occurred: {} ", why),
                }
            }
        if action == "rem" {
            task.rem(item);
                match task.rem() {
                    Ok(_) => println!("Task Successfully Removed"),
                    Err(why) => println!("An Error Occurred: {} ", why),
                }
            }
        if action == "mod" {
            task.change_state(item);
                match task.change_state() {
                    Ok(_) => println!("Task Successfully Modified"),
                    Err(why) => println("Task Failed: {} ", why),
                }
        }
    }
}

//todo add float point numbers to tasks for task nesting and possibly for ease of editing or removing tasks.
#[derive(Debug)]
struct Task {
    map: HashMap<String, TaskState>
}

impl Task {
    fn add(&mut self, key: String, state: TaskState) {
        self.map.insert(key, TaskState::Incomplete);
    }
    fn change_state(&mut self, input_key: String, input_state: TaskState) {
        match self.get("key") {
            Some(val) => self.update(key: input_key, state: input_state)
        }
    }
    fn rem(&mut self, key: String)
        match self.remove(key);
}

#[derive(Debug)]
enum TaskState {
    Incomplete,
    InProgress,
    Complete,
}

