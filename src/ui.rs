use std::io::{self, Write};

pub fn show_menu() -> String {
    println!("\n=== Task Manager ===");
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
