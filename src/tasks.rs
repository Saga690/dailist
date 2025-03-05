use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Task {
            name: name.to_string(),
            completed: false,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}

pub fn load_tasks() -> Vec<Task> {
    let path = "tasks.json";
    if Path::new(path).exists() {
        let file = File::open(path).expect("Unable to open tasks file");
        let tasks: Vec<Task> = serde_json::from_reader(file).expect("Unable to parse tasks");
        tasks
    } else {
        Vec::new()
    }
}

pub fn save_tasks(tasks: &[Task]) {
    let file = File::create("tasks.json").expect("Unable to create tasks file");
    serde_json::to_writer_pretty(file, &tasks).expect("Unable to write tasks to file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task");
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.completed, false);
    }

    #[test]
    fn test_mark_completed() {
        let mut task = Task::new("Test Task");
        task.mark_completed();
        assert_eq!(task.completed, true);
    }
}
