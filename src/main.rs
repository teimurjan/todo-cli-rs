use clap::{Parser, Subcommand};
use inquire::Select;
mod todo;
use todo::{Todo, TodoList};

#[derive(Parser)]
#[command(name = "TodoApp", version, about = "Simple CLI to-do app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        #[arg()]
        task: String,
    },

    /// List all tasks
    List,

    /// Mark a task as done
    Done,

    /// Remove a task
    Remove,
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list = TodoList::new();

    match cli.command {
        Commands::Add { task } => match todo_list.add(task.clone()) {
            Ok(_) => println!("Added task: {}", task),
            Err(e) => eprintln!("Failed to add task: {}", e),
        },
        Commands::List => {
            let todos = todo_list.list();
            if todos.is_empty() {
                println!("No tasks found.");
                return;
            }

            println!("Tasks:");
            for todo in todos {
                println!("{}", todo);
            }
        }
        Commands::Done => {
            let todos = todo_list.list();
            if todos.is_empty() {
                println!("No tasks found.");
                return;
            }

            let options: Vec<Todo> = todos.iter().filter(|t| !t.completed).cloned().collect();

            if options.is_empty() {
                println!("No incomplete tasks found.");
                return;
            }

            match Select::new("Select task to mark as done:", options).prompt() {
                Ok(task) => match todo_list.complete(task.id.clone()) {
                    Ok(true) => println!("Marked task '{}' as done", task.task),
                    Ok(false) => println!("Task '{}' not found", task.task),
                    Err(e) => eprintln!("Failed to mark task as done: {}", e),
                },
                Err(_) => println!("Operation cancelled"),
            }
        }
        Commands::Remove => {
            let todos = todo_list.list();
            if todos.is_empty() {
                println!("No tasks found.");
                return;
            }

            let options: Vec<Todo> = todos.iter().cloned().collect();

            match Select::new("Select task to remove:", options).prompt() {
                Ok(task) => match todo_list.remove(task.id.clone()) {
                    Ok(true) => println!("Removed task '{}'", task.task),
                    Ok(false) => println!("Task '{}' not found", task.task),
                    Err(e) => eprintln!("Failed to remove task: {}", e),
                },
                Err(_) => println!("Operation cancelled"),
            }
        }
    }
}
