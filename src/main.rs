use std::io;

fn main() {
    let task1 = Task {
        description: String::from("Make Dinner"),
        state: TaskState::Incomplete,
    };
    println!("You're doing it!, {:?}", task1)
}

#[derive(Debug)]
struct Task {
    description: String,
    state: TaskState,
}

impl Task {
    fn new(self) -> Self {
        Task {
            description: String::from("New Task"),
            state: TaskState::Incomplete,
        }
    }
}

#[derive(Debug)]
enum TaskState {
    Incomplete,
    InProgress,
    Complete,
}

fn new_task(task_name: &str) -> io::Result<()> {
    let mut task_name = String::new();
    let task_name = io::stdin().read_line(&mut task_name).unwrap();
    Ok(())
}
