use std::env;
use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug)]
enum TaskError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    ParseBool(std::str::ParseBoolError),
    InvalidFormat,
}

impl From<std::io::Error> for TaskError {
    fn from(err: std::io::Error) -> Self {
        TaskError::Io(err)
    }
}

impl From<std::num::ParseIntError> for TaskError {
    fn from(err: std::num::ParseIntError) -> Self {
        TaskError::ParseInt(err)
    }
}

impl From<std::str::ParseBoolError> for TaskError {
    fn from(err: std::str::ParseBoolError) -> Self {
        TaskError::ParseBool(err)
    }
}

fn main() -> Result<(), TaskError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];

    // Load existing tasks
    let mut tasks = load_tasks(file_path)?;

    println!("Before adding new task:");
    for task in &tasks {
        println!("{:?}", task);
    }

    // Add a mock task (normally, you'd get this from user input)
    let new_task = Task {
        id: (tasks.len() + 1) as u32,
        title: "Learn Rust Error Handling".to_string(),
        completed: false,
    };
    tasks.push(new_task);

    // Save back to file
    save_tasks(file_path, &tasks)?;

    println!("\nAfter saving new task:");
    for task in &tasks {
        println!("{:?}", task);
    }

    Ok(())
}

fn load_tasks(path: &str) -> Result<Vec<Task>, TaskError> {
    let contents = fs::read_to_string(path)?;
    let mut tasks = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            continue; // skip invalid lines
        }
        let id = parts[0].parse::<u32>()?;
        // let id = match parts[0].parse::<u32>() {
        //     Ok(val) => val,
        //     Err(e) => return Err(e.into()), // convert to our custom error if needed
        // };
        let title = parts[1].to_string();
        let completed = parts[2].parse::<bool>()?;

        tasks.push(Task {
            id,
            title,
            completed,
        });
    }
    Ok(tasks)
}

fn save_tasks(path: &str, tasks: &Vec<Task>) -> Result<(), TaskError> {
    let mut contents = String::new();
    for task in tasks {
        contents.push_str(&format!("{}|{}|{}\n", task.id, task.title, task.completed));
    }

    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
