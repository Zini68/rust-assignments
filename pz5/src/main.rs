use clap::Parser;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fs::File as StdFile;
use std::io::{self, Read, Write};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct AppConfig {
    debug: bool,
    #[serde(default = "default_db")]
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    port: u16,
}

fn default_db() -> DatabaseConfig {
    DatabaseConfig { url: "localhost".to_string(), port: 5432 }
}

#[derive(Parser, Debug)]
#[command(version, about = "Snippets App with Config and Logging")]
struct Args {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, env = "CONF_FILE", default_value = "config.toml")]
    conf: String,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(long)]
    download: Option<String>,
}

fn init_logging() {

    let log_level = std::env::var("SNIPPETS_APP_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level = match log_level.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    FmtSubscriber::builder()
        .with_max_level(level)
        .init();
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    init_logging();

    let settings = Config::builder()
        .set_default("debug", false)?
        .add_source(File::with_name(&args.conf).required(false))
        .add_source(Environment::with_prefix("CONF").separator("__"))
        .set_override_option("debug", if args.debug { Some(true) } else { None })?
        .build()?;

    let config: AppConfig = settings.try_deserialize()?;
    
    info!("Configuration loaded: {:?}", config);

    if let (Some(name), Some(url)) = (args.name, args.download) {
        info!("Downloading snippet '{}' from {}", name, url);
        let response = reqwest::blocking::get(url)?;
        
        if response.status().is_success() {
            let content = response.text()?;
            let mut file = StdFile::create(format!("{}.txt", name))?;
            file.write_all(content.as_bytes())?;
            info!("Snippet saved successfully!");
        } else {
            error!("Failed to download snippet: {}", response.status());
        }
    } else {
        println!("App config: {:#?}", config);
        println!("Usage hint: use --name and --download to fetch a snippet.");
    }

    Ok(())
}
