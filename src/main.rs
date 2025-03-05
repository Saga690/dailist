use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    completed: bool,
}

impl Task {
    fn new(name: &str) -> Self {
        Task {
            name: name.to_string(),
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

fn load_tasks() -> Vec<Task> {
    let path = "tasks.json";
    if Path::new(path).exists() {
        let file = File::open(path).expect("Unable to open tasks file");
        let tasks: Vec<Task> = serde_json::from_reader(file).expect("Unable to parse tasks");
        tasks
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &[Task]) {
    let file = File::create("tasks.json").expect("Unable to create tasks file");
    serde_json::to_writer_pretty(file, &tasks).expect("Unable to write tasks to file");
}

fn show_menu() -> String { //Main Menu
    println!("\n=== Task Manager (by Saga) ===");
    println!("Choose an action:");
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Mark Task as Completed");
    println!("4. Remove Task");
    println!("5. Exit");
    print!("Enter your choice: ");
    
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    choice.trim().to_string()
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        let choice = show_menu();

        match choice.as_str() {
            "1" => {
                // Add task
                print!("Enter the task name: ");
                io::stdout().flush().unwrap();
                
                let mut task_name = String::new();
                io::stdin().read_line(&mut task_name).unwrap();
                let task_name = task_name.trim();

                let new_task = Task::new(task_name);
                tasks.push(new_task);
                save_tasks(&tasks);
                println!("Task added: {}", task_name);
            }
            "2" => {
                // List all tasks
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    println!("Your tasks");
                    for (index, task) in tasks.iter().enumerate() {
                        let status = if task.completed { "Completed" } else { "Pending" };
                        println!("{}. {} [{}]", index + 1, task.name, status);
                    }
                }
            }
            "3" => {
                // Mark task as completed
                print!("Enter task number to mark as completed: ");
                io::stdout().flush().unwrap();
                
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).unwrap();
                let task_number: usize = task_number.trim().parse().unwrap_or(0);
                
                if task_number > 0 && task_number <= tasks.len() {
                    tasks[task_number - 1].mark_completed();
                    save_tasks(&tasks);
                    println!("Task marked as completed: {}", tasks[task_number - 1].name);
                } else {
                    println!("Invalid task number.");
                }
            }
            "4" => {
                // Remove task
                print!("Enter task number to remove: ");
                io::stdout().flush().unwrap();
                
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).unwrap();
                let task_number: usize = task_number.trim().parse().unwrap_or(0);
                
                if task_number > 0 && task_number <= tasks.len() {
                    tasks.remove(task_number - 1);
                    save_tasks(&tasks);
                    println!("Task removed.");
                } else {
                    println!("Invalid task number.");
                }
            }
            "5" => {
                // Exit
                println!("Exiting Task Manager...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
