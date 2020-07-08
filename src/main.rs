extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error;

pub(crate) mod config;
pub(crate) mod countries;
mod rank;
mod uri;

use chrono::{DateTime, Utc};
use config::{get_config, Config};
use rank::rank_mirrors;
use std::fs;
use which::which;

type BoxError = Box<dyn std::error::Error>;
type BoxResult<T> = Result<T, BoxError>;

#[tokio::main]
async fn main() -> BoxResult<()> {
    pretty_env_logger::init();
    // Check if running as root
    sudo::escalate_if_needed()?;
    info!("Running as root !");

    // Check if Pacman is present
    which("pacman").expect("This package automates the process of keeping Pacman mirrorlist up to date, thus requiring the latter to be installed.");

    // Read the config from the TOML file
    let config: Config = get_config("config.toml")?;

    // Build the URI from the config
    let uri = uri::build_uri(&config, &countries::get_countries());
    // Request the mirrorlist
    let content = reqwest::get(&uri).await?.text().await?;

    // Backup the mirrorlist file
    let now: DateTime<Utc> = Utc::now();

    fs::copy(
        "/etc/pacman.d/mirrorlist",
        format!("/etc/pacman.d/mirrorlist-backup-{}", now.timestamp()),
    )?;

    // Only keep the revelant lines
    let relevant_lines = content
        .lines()
        .filter_map(Option::Some)
        .filter(|line| line.to_string().starts_with("#Server = "))
        .map(|line| line.replace("#Server = ", ""))
        .collect();

    let ranked_mirrors = rank_mirrors(&relevant_lines)
        .await?
        .iter()
        .map(|line| format!("Server = {}", line))
        .collect::<Vec<String>>()
        .join("\n");

    // Rank the servers by speed
    // Paste the new content
    fs::write("/etc/pacman.d/mirrorlist", ranked_mirrors)?;

    Ok(())
}
