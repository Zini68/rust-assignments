use clap::Parser;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Local};
use std::io::{self, Read};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Snippet {
    content: String,
    created_at: DateTime<Local>,
}

trait SnippetStorage {
    fn add(&mut self, name: &str, content: String);
    fn get(&self, name: &str) -> Option<String>;
    fn delete(&mut self, name: &str) -> bool;
}

struct JsonStorage {
    file_path: String,
    snippets: HashMap<String, Snippet>,
}

impl JsonStorage {
    fn new(path: String) -> Self {
        let snippets = if Path::new(&path).exists() {
            let data = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        };
        
        Self { file_path: path, snippets }
    }

    fn save_to_file(&self) {
        let data = serde_json::to_string_pretty(&self.snippets).expect("Error serializing JSON");
        fs::write(&self.file_path, data).expect("Error writing file");
    }
}

impl SnippetStorage for JsonStorage {
    fn add(&mut self, name: &str, content: String) {
        let snippet = Snippet {
            content,
            created_at: Local::now(),
        };
        self.snippets.insert(name.to_string(), snippet);
        self.save_to_file();
    }

    fn get(&self, name: &str) -> Option<String> {
        if let Some(s) = self.snippets.get(name) {
            Some(format!("{}\n[Created at: {}]", s.content, s.created_at))
        } else {
            None
        }
    }

    fn delete(&mut self, name: &str) -> bool {
        if self.snippets.remove(name).is_some() {
            self.save_to_file();
            true
        } else {
            false
        }
    }
}

struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    fn new(path: String) -> Self {
        let conn = Connection::open(path).expect("Could not open DB");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                name TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        ).expect("Failed to create table");
        
        Self { conn }
    }
}

impl SnippetStorage for SqliteStorage {
    fn add(&mut self, name: &str, content: String) {
        let date = Local::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO snippets (name, content, created_at) VALUES (?1, ?2, ?3)",
            params![name, content, date],
        ).expect("DB Error: insert failed");
    }

    fn get(&self, name: &str) -> Option<String> {
        let mut stmt = self.conn.prepare("SELECT content, created_at FROM snippets WHERE name = ?1").ok()?;
        let mut rows = stmt.query(params![name]).ok()?;

        if let Some(row) = rows.next().ok()? {
            let content: String = row.get(0).unwrap();
            let date: String = row.get(1).unwrap();
            Some(format!("{}\n[Created at: {}]", content, date))
        } else {
            None
        }
    }

    fn delete(&mut self, name: &str) -> bool {
        let count = self.conn.execute(
            "DELETE FROM snippets WHERE name = ?1",
            params![name],
        ).unwrap_or(0);
        count > 0
    }
}

#[derive(Parser)]
#[command(name = "snippets-app")]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(long)]
    read: Option<String>,

    #[arg(long)]
    delete: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let env_val = env::var("SNIPPETS_APP_STORAGE").unwrap_or_else(|_| "JSON:snippets.json".to_string());
    
    let parts: Vec<&str> = env_val.splitn(2, ':').collect();
    let storage_type = parts[0];
    let storage_path = if parts.len() > 1 { parts[1] } else { "snippets.json" };

    let mut storage: Box<dyn SnippetStorage> = match storage_type {
        "SQLITE" => Box::new(SqliteStorage::new(storage_path.to_string())),
        _ => Box::new(JsonStorage::new(storage_path.to_string())),
    };

    if let Some(read_name) = args.read {
        match storage.get(&read_name) {
            Some(text) => println!("{}", text),
            None => println!("Snippet not found."),
        }
    } else if let Some(del_name) = args.delete {
        if storage.delete(&del_name) {
            println!("Deleted successfully.");
        } else {
            println!("Snippet not found.");
        }
    } else if let Some(create_name) = args.name {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_default();
        let content = buffer.trim().to_string();

        if content.is_empty() {
            println!("Error: Content is empty.");
        } else {
            storage.add(&create_name, content);
            println!("Saved!");
        }
    } else {
        println!("Please use --help to see available commands.");
    }
}
