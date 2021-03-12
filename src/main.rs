pub(crate) mod config;
pub(crate) mod countries;
mod rank;
mod uri;

use chrono::{DateTime, Utc};
use config::{get_config, Config};
use rank::rank_mirrors;
use std::time::Instant;
use which::which;

type BoxError = Box<dyn std::error::Error>;
type BoxResult<T> = Result<T, BoxError>;

#[tokio::main]
async fn main() -> BoxResult<()> {
    pretty_env_logger::try_init()?;

    // Check if Pacman is present
    which("pacman").expect("This package automates the process of keeping Pacman mirrorlist up to date, thus requiring the latter to be installed.");

    // Read the config from the TOML file
    let config: Config = get_config("/etc/scaramanga/config.toml")?;

    // Build the URI from the config
    let uri = uri::build_uri(&config, &countries::get_countries());

    // Request the mirrorlist
    let content = reqwest::get(&uri).await?.text().await?;

    // Backup the mirrorlist file
    let now: DateTime<Utc> = Utc::now();
    let chrono = Instant::now();

    // Only keep the revelant lines
    let relevant_lines = content
        .lines()
        .filter_map(Option::Some)
        .filter(|line| line.to_string().starts_with("#Server = "))
        .map(|line| line.replace("#Server = ", ""))
        .collect();

    // Rank the servers by speed
    let ranked_mirrors = rank_mirrors(&relevant_lines)
        .await?
        .iter()
        .map(|line| format!("Server = {}", line))
        .collect::<Vec<String>>()
        .join("\n");

    // Print the result
    println!(
        "# Generated on {} in {}ms\n{}",
        now.format("%Y-%m-%d at %H:%M"),
        chrono.elapsed().as_millis(),
        ranked_mirrors
    );

    Ok(())
}
