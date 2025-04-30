# todo-cli-rs

A command-line todo list manager written in Rust. This CLI application allows you to manage your tasks efficiently with features like adding, completing, listing, and removing todos.

## Features

- Add new todos with descriptions
- Mark todos as complete/incomplete
- List all todos with their status
- Remove todos
- Persistent storage using JSON
- Interactive CLI interface
- Unique IDs for each todo

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/teimurjan/todo-cli-rs.git
cd todo-cli-rs
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available in `target/release/todo-cli-rs`

## Usage

Here are the main commands available:

```bash
# Add a new todo
todo-cli-rs add "Your todo description"

# List all todos
todo-cli-rs list

# Complete a todo (opens an interactive select)
todo-cli-rs complete

# Remove a todo (opens an interactive select)
todo-cli-rs remove
```

## Dependencies

- `clap` (v4) - Command line argument parsing
- `serde` (v1.0) - Serialization/deserialization framework
- `serde_json` (v1.0) - JSON support
- `uuid` (v1.16.0) - Unique ID generation
- `inquire` (v0.7) - Interactive CLI interface

## Project Structure

```
todo-cli-rs/
├── src/
│   ├── main.rs    # Application entry point and CLI handling
│   └── todo.rs    # Todo item implementation and storage logic
├── Cargo.toml     # Project manifest and dependencies
└── todos.json     # Persistent storage for todos
```
