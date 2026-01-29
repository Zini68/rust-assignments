use clap::Parser;
use config::{Config, Environment, File as ConfigFile};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use tracing::{info, error};
use pz6::{format_filename, is_valid_url}; 

#[derive(Debug, Deserialize)]
struct AppConfig {
    debug: bool,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, env = "CONF_FILE", default_value = "config.toml")]
    conf: String,
    #[arg(short, long)]
    name: Option<String>,
    #[arg(long)]
    download: Option<String>,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let settings = Config::builder()
        .set_default("debug", false)?
        .add_source(ConfigFile::with_name(&args.conf).required(false))
        .add_source(Environment::with_prefix("CONF"))
        .build()?;

    let config: AppConfig = settings.try_deserialize()?;
    info!("Config loaded: debug={}", config.debug);

    if let (Some(name), Some(url)) = (args.name, args.download) {
        if !is_valid_url(&url) {
            error!("Invalid URL provided!");
            return Ok(());
        }

        let content = reqwest::blocking::get(url)?.text()?;
        let filename = format_filename(&name);
        
        let mut file = File::create(&filename)?;
        file.write_all(content.as_bytes())?;
        info!("Saved snippet to {}", filename);
    }

    Ok(())
}
