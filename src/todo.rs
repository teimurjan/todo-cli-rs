use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::{self, Error, ErrorKind};
use uuid::Uuid;

const TODO_FILE: &str = "todos.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: String,
    pub task: String,
    pub completed: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✅" } else { "❌" };
        write!(f, "{} {} ({})", status, self.task, self.id)
    }
}

impl Todo {
    pub fn new(task: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        Self::init().unwrap_or_default()
    }

    fn init() -> io::Result<Self> {
        match fs::read_to_string(TODO_FILE) {
            Ok(content) => {
                serde_json::from_str(&content).map_err(|e| Error::new(ErrorKind::InvalidData, e))
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e),
        }
    }

    fn flush(&self) -> io::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(TODO_FILE, content)
    }

    pub fn add(&mut self, task: String) -> io::Result<()> {
        let todo = Todo::new(task);
        self.todos.push(todo);
        self.flush()
    }

    pub fn list(&self) -> &[Todo] {
        &self.todos
    }

    pub fn complete(&mut self, id: String) -> io::Result<bool> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.complete();
            self.flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn remove(&mut self, id: String) -> io::Result<bool> {
        let initial_len = self.todos.len();
        self.todos.retain(|t| t.id != id);
        if self.todos.len() != initial_len {
            self.flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
