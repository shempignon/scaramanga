use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

use crate::countries;

#[derive(Deserialize)]
pub struct Config {
    pub http: bool,
    pub https: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub countries: Option<Vec<String>>,
}

pub fn get_config() -> crate::BoxResult<Config> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(contents.as_str())?;

    countries::validate_config(&config)?;

    match config {
        Config {
            http: false,
            https: false,
            ipv4: _,
            ipv6: __,
            countries: ___,
        } => bail!("You need to activate at least one connection protocol (http or https)"),
        Config {
            http: _,
            https: __,
            ipv4: false,
            ipv6: false,
            countries: ___,
        } => bail!("You need to activate at least one IP version"),
        config => Ok(config),
    }
}
