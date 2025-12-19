use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use thiserror::Error;

const DATA_FILE: &str = "snippets.json";

#[derive(Error, Debug)]
enum SnippetError {
    #[error("Failed to interact with the database file: {0}")]
    DatabaseError(#[from] std::io::Error),

    #[error("Failed to parse JSON data: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Snippet with name '{0}' not found.")]
    NotFound(String),

    #[error("Snippet with name '{0}' already exists.")]
    AlreadyExists(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    name: String,
    content: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        content: String,
    },
    Get {
        name: String,
    },
    List,
}


fn load_snippets() -> Result<HashMap<String, String>, SnippetError> {
    if !std::path::Path::new(DATA_FILE).exists() {
        return Ok(HashMap::new());
    }

    let file = File::open(DATA_FILE)?;
    let reader = BufReader::new(file);
    let map: HashMap<String, String> = serde_json::from_reader(reader)?;
    Ok(map)
}

fn save_snippets(map: &HashMap<String, String>) -> Result<(), SnippetError> {
    let file = File::create(DATA_FILE)?;
    serde_json::to_writer_pretty(file, map)?;
    Ok(())
}

fn cmd_add(name: String, content: String) -> Result<(), SnippetError> {
    let mut map = load_snippets()?;
    if map.contains_key(&name) {
        return Err(SnippetError::AlreadyExists(name));
    }
    map.insert(name.clone(), content);
    save_snippets(&map)?;
    println!("✅ Snippet '{}' added successfully.", name);
    Ok(())
}

fn cmd_get(name: String) -> Result<(), SnippetError> {
    let map = load_snippets()?;
    match map.get(&name) {
        Some(content) => {
            println!("Snippet '{}':\n----------------\n{}", name, content);
            Ok(())
        }
        None => Err(SnippetError::NotFound(name)),
    }
}

fn cmd_list() -> Result<(), SnippetError> {
    let map = load_snippets()?;
    if map.is_empty() {
        println!("No snippets found.");
        return Ok(());
    }
    println!("Available snippets:");
    for name in map.keys() {
        println!("- {}", name);
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, content } => {
            cmd_add(name, content).context("Failed to execute 'add' command")?;
        }
        Commands::Get { name } => {
            cmd_get(name).context("Failed to execute 'get' command")?;
        }
        Commands::List => {
            cmd_list().context("Failed to execute 'list' command")?;
        }
    }

    Ok(())
}
