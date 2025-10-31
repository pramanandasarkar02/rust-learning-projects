use std::io::{Write, stdin, stdout};

struct Task {
    title: String,
    duration: u32,
    is_done: bool,
}

impl Task {
    fn new_task(title: String, duration: u32) -> Task {
        Task {
            title,
            duration,
            is_done: false,
        }
    }

    fn print_task(&self) {
        let task_status = if self.is_done { "⚔️" } else { "🔴" };
        println!("\t{}: \t\t({})\t\t{}", self.title, self.duration, task_status);
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn add_task(&mut self, title: String, duration: u32) {
        let new_task = Task::new_task(title, duration);
        self.tasks.push(new_task);
    }

    fn delete_task(&mut self, task_id: u32) {
        if (task_id as usize) < self.tasks.len() {
            self.tasks.remove(task_id as usize);
        } else {
            println!("Invalid task ID: {}", task_id);
        }
    }

    fn print_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            print!("{}. ", index);
            task.print_task();
        }
    }

    fn mark_task(&mut self, task_id: u32, is_done: bool) {
        if (task_id as usize) < self.tasks.len() {
            self.tasks[task_id as usize].is_done = is_done;
        } else {
            println!("Invalid task ID: {}", task_id);
        }
    }
}

fn print_action() {
    println!("Actions:");
    println!("1. Add task");
    println!("2. Show task list");
    println!("3. Delete task");
    println!("4. Mark task as done");
    println!("5. Save tasks");
    println!("6. Show actions");
    println!("0. Exit");
}

fn action_parser(tm: &mut TaskManager, action: i32) {
    match action {
        1 => {
            let mut title = String::new();
            let mut duration_str = String::new();
            println!("Adding new task...");
            print!("Enter Title: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut title).unwrap();
            let title = title.trim().to_string();

            print!("Enter Duration: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut duration_str).unwrap();
            let duration: u32 = match duration_str.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Invalid duration input!");
                    return;
                }
            };
            tm.add_task(title, duration);
        }
        2 => {
            tm.print_tasks();
        }
        3 => {
            let mut task_id_str = String::new();
            println!("Deleting task...");
            print!("Enter Task ID: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut task_id_str).unwrap();
            let task_id: u32 = match task_id_str.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Invalid task ID!");
                    return;
                }
            };
            tm.delete_task(task_id);
        }
        4 => {
            let mut task_id_str = String::new();
            println!("Marking task as done...");
            print!("Enter Task ID: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut task_id_str).unwrap();
            let task_id: u32 = match task_id_str.trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    println!("Invalid task ID!");
                    return;
                }
            };
            tm.mark_task(task_id, true);
        }
        5 => {
            println!("Saving tasks...");
        }
        6 => {
            print_action();
        }
        _ => {
            print_action();
        }
    }
}

fn main() {
    println!("To-Do List");

    let mut tm = TaskManager { tasks: Vec::new() };
    print_action();
    loop {
        let mut action_str = String::new();
        print!("Enter action (0 - 6): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut action_str).unwrap();
        let trimmed = action_str.trim();

        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }

        match trimmed.parse::<i32>() {
            Ok(action_number) => {
                if action_number == 0 {
                    break;
                }
                action_parser(&mut tm, action_number);
            }
            Err(_) => {
                println!("Invalid input, please enter a number (0-6) or type 'exit'.");
            }
        }
    }
    println!("Exiting To-Do List");
}
