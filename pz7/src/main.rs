#![warn(missing_docs)]

use clap::Parser;
use pz7::{format_filename, is_valid_url};

#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {

    pub debug: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, env = "CONF_FILE", default_value = "config.toml")]
    pub conf: String,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(long)]
    pub download: Option<String>,
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
