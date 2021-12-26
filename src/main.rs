use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("Failed to flush");
    stdin().read_line(input).expect("Failed to read");
}

fn main() {
    println!("To-Do list of sorts!");
    println!("-----------------------");

    loop {
        let mut action = String::new();
        let mut task = String::new();
        let mut state = String::new();
        let mut task_list: HashMap<&str, TaskState> = HashMap::new();

        print!("Are you adding, removing or modifying a task? (add, rem, mod");
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

            if action == "rem" {
                task.rem(task);
                match task.rem() {
                    Ok(_) => println!("Task Successfully Removed"),
                    Err(why) => println!("An Error Occurred: {} ", why),
                }
            }
            if action == "mod" {
                task.change_state(item);
                match task.change_state() {
                    Ok(_) => println!("Task Successfully Modified"),
                    Err(why) => println!("Task Failed: {} ", why),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Task {
    map: HashMap<String, TaskState>,
}

impl Task {
    fn add(&mut self, key: String, state: TaskState) {
        self.map.insert(key, TaskState::Incomplete);
    }
    fn change_state(&mut self, input_key: String, input_state: TaskState) {
        match self.get(input_key) {
            Some(val) => self.update(input_key, input_state),
            None => {
                // TODO: Put code to handle the None case here
            }
        }
    }
    fn rem(&mut self, key: String) {
        match self.get(key) {
            Some(val) => {
                // TODO: I'm not sure what you're trying to express here, so I've commented it out. The problem was that
                // you didn't have a Some arm for the match expression. You had it up above, just not here.
                //self.insert(key) => self.remove(key)
            }
            None => println!("Task doesn't exist"),
        }
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{:?}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }

    fn get(&mut self, k: &str) {
        unimplemented!()
    }

    fn update(&mut self, k: &str, v: TaskState) {
        unimplemented!()
    }
}

#[derive(Debug)]
enum TaskState {
    Incomplete,
    InProgress,
    Complete,
}
