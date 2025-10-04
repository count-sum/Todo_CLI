use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILE: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(
    about = "A simple todo CLI in Rust",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { text: String },
    Edit { id: usize, text: String },
    Complete { id: usize },
    Reset { id: usize },
    Delete { id: usize },
    List,
}

fn load_todos() -> Vec<Todo> {
    if Path::new(FILE).exists() {
        let data = fs::read_to_string(FILE).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(&todos).expect("Unable to serialize");
    fs::write(FILE, data).expect("Unable to write file");
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add { text } => {
            let id = todos.len() + 1;
            todos.push(Todo {
                id,
                text,
                done: false,
            });
            save_todos(&todos);
            println!("Added todo #{id}");
        }
        Commands::Edit { id, text } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.text = text;
                save_todos(&todos);
                println!("Edited todo #{id}");
            } else {
                println!("Todo not found!");
            }
        }
        Commands::Complete { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.done = true;
                save_todos(&todos);
                println!("Completed todo #{id}");
            } else {
                println!("Todo not found!");
            }
        }
        Commands::Reset { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.done = false;
                save_todos(&todos);
                println!("Reseted todo #{id}");
            } else {
                println!("Todo not found!");
            }
        }
        Commands::Delete { id } => {
            let len_before = todos.len();
            todos.retain(|t| t.id != id);
            if todos.len() < len_before {
                save_todos(&todos);
                println!("Deleted todo #{id}");
            } else {
                println!("Todo not found!");
            }
        }

        Commands::List => {
            for todo in &todos {
                let status = if todo.done { "[x]" } else { "[ ]" };
                println!("{} {} - {}", todo.id, status, todo.text);
            }
        }
    }
}
