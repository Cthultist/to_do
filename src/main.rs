// use crossterm::{event, cursor, style, terminal};
// use std::io::{stdout, Write, Read};
// use std::collections::HashMap;

fn main() {
    println!("You're doing it!")
}

struct Task {
    description: String,
    state: TaskState,
}

enum TaskState {
    Incomplete,
    InProgress,
    Complete,
}
