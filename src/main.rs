pub(crate) mod config;
pub(crate) mod countries;
mod rank;
mod uri;

use config::{get_config, Config};
use rank::Ranker;
use time::{format_description::parse, OffsetDateTime};

type BoxError = Box<dyn std::error::Error>;
type BoxResult<T> = Result<T, BoxError>;

#[tokio::main]
async fn main() -> BoxResult<()> {
    pretty_env_logger::try_init()?;

    // Read the config from the TOML file
    let config: Config = get_config("/etc/scaramanga/config.toml")?;

    // Build the URI from the config
    let uri = uri::build_uri(&config, &countries::get_countries());

    // Request the mirrorlist
    let content = reqwest::get(&uri).await?.text().await?;

    // Only keep the revelant lines
    let relevant_lines: Vec<String> = content
        .lines()
        .filter_map(Option::Some)
        .filter(|line| line.to_string().starts_with("#Server = "))
        .map(|relevant_line| relevant_line.replace("#Server = ", ""))
        .collect();

    let mirrors: Vec<&str> = relevant_lines.iter().map(String::as_ref).collect();

    // Rank the servers by speed
    let ranked_mirrors = Ranker::new()?
        .rank_mirrors(mirrors.as_slice())
        .await?
        .iter()
        .map(|ranked_mirror| ranked_mirror.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    // Print the result
    let format = parse("[year]-[month]-[day] at [hour]:[minute]")?;
    let now = OffsetDateTime::now_utc().format(&format)?;

    println!("# Generated on {now}\n{ranked_mirrors}");

    Ok(())
}
