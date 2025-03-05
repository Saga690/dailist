mod tasks;
mod ui;

use tasks::{load_tasks, save_tasks, Task};
use ui::show_menu;
use std::io::Write;
use chrono::NaiveDate;

fn main() {
    let mut tasks = load_tasks();

    loop {
        let choice = show_menu();

        match choice.as_str() {
            "1" => {
                // Add task
                print!("Enter the task name: ");
                std::io::stdout().flush().unwrap();
                
                let mut task_name = String::new();
                std::io::stdin().read_line(&mut task_name).unwrap();
                let task_name = task_name.trim();

                print!("Enter the due date (YYYY-MM-DD) or leave empty: ");
                std::io::stdout().flush().unwrap();
                
                let mut due_date_str = String::new();
                std::io::stdin().read_line(&mut due_date_str).unwrap();
                let due_date_str = due_date_str.trim();

                let mut new_task = Task::new(task_name);

                if !due_date_str.is_empty() {
                    match NaiveDate::parse_from_str(due_date_str, "%Y-%m-%d") {
                        Ok(due_date) => {
                            new_task.set_due_date(due_date);
                            println!("Due date set to: {}", due_date_str);
                        },
                        Err(_) => println!("Invalid due date format! Please use YYYY-MM-DD."),
                    }
                }

                tasks.push(new_task);
                save_tasks(&tasks);
                println!("Task added: {}", task_name);
            }
            "2" => {
                // List all tasks
                println!("Your tasks");
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    for (index, task) in tasks.iter().enumerate() {
                        let status = if task.completed { "Completed" } else { "Pending" };
                        let due_date = task.get_due_date().unwrap_or_else(|| "No due date".to_string());
                        println!("{}. {} [{}] Due: {}", index + 1, task.name, status, due_date);
                    }
                }
            }
            "3" => {
                // Mark task as completed
                print!("Enter task number to mark as completed: ");
                std::io::stdout().flush().unwrap();
                
                let mut task_number = String::new();
                std::io::stdin().read_line(&mut task_number).unwrap();
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
                std::io::stdout().flush().unwrap();
                
                let mut task_number = String::new();
                std::io::stdin().read_line(&mut task_number).unwrap();
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
