#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;

pub(crate) mod config;
pub(crate) mod countries;
mod uri;

use chrono::{DateTime, Utc};
use config::{get_config, Config};
use regex::RegexBuilder;
use std::fs;
use which::which;

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> BoxResult<()> {
    // Check if running as root
    sudo::escalate_if_needed()?;
    info!("Running as root !");

    // Check if Pacman is present
    which("pacman").expect("This package automates the process of keeping Pacman mirrorlist up to date, thus requiring the latter to be installed.");

    // Read the config from the TOML file
    let config: Config = get_config()?;

    // Build the URI from the config
    let uri = uri::build_uri(&config, &countries::get_countries())?;
    println!("{}", uri);

    // Request the mirrorlist
    let content = reqwest::get(&uri).await?.text().await?;

    // Backup the mirrorlist file
    let now: DateTime<Utc> = Utc::now();

    fs::copy(
        "/etc/pacman.d/mirrorlist",
        format!("/etc/pacman.d/mirrorlist-backup-{}", now.timestamp()),
    )?;

    // Remove the #s
    let re = RegexBuilder::new(r"^#").multi_line(true).build()?;
    let replaced = re.replace_all(content.as_str(), "").to_string();

    // Paste the new content
    fs::write("/etc/pacman.d/mirrorlist", replaced)?;

    Ok(())
}
