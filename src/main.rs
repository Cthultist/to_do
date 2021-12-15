use std::io::Read;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action.");
    let task_name = std::env::args().nth(2).expect("Please specify an item");
    
    if action == "add" {
        let mut new_task = Task::new().expect("Initialization of db failed");
        match task.save() {
            Ok(_) => println!("Task Saved!"),
            Err(why) => println!("An error occurred: {}", why)
        } 
    } else if action == "complete" {
        match task.complete(&task_name) {
            None => println!("'{}' is not present in the list", task_name),
            Some(_) => match task.save() {
                Ok(_) => println!("To-do updated!"),
                Err(why) => println!("An error Occurred: {}", why),
            }
        }
    }
    // println!("{:?}, {:?}", task_name, task_state);
    // let task1 = Task {
    //     description: String::from("Make Dinner"),
    //     state: TaskState::Incomplete,
    // };
    // println!("You're doing it!, {:?}", task1)
}

//todo add float point numbers to tasks for task nesting and possibly for ease of editing or removing tasks.
#[derive(Debug)]
struct Task {
    description: String,
    state: TaskState,
}

impl Task {
    fn new() -> Result<Task, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        std::fs::read_to_string("db.txt")
            .expect("file not found!")
            .lines()
            .map(|x| x.parse())
            .collect()
    }
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = self.new_task;
        
        std::fs::write("db.txt", self)
    }
}

#[derive(Debug)]
enum TaskState {
    Incomplete,
    InProgress,
    Complete,
}

// fn new_task(task_name: &str) -> io::Result<()> {
//     let mut task_name = String::new();
//     let task = io::stdin().read_line(&mut task_name).unwrap();
//     Ok(())
// }
