use serde::Deserialize;
use simple_error::bail;
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

pub fn get_config(file: &str) -> crate::BoxResult<Config> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(contents.as_str())?;

    countries::validate_config(&config)?;

    match config {
        Config {
            http: false,
            https: false,
            ipv4: _,
            ipv6: _,
            countries: _,
        } => bail!("You need to activate at least one connection protocol (http or https)"),
        Config {
            http: _,
            https: _,
            ipv4: false,
            ipv6: false,
            countries: _,
        } => bail!("You need to activate at least one IP version"),
        config => Ok(config),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(
        expected = "You need to activate at least one connection protocol (http or https)"
    )]
    fn test_it_handles_incoherent_protocol_config() {
        get_config("tests/incoherent_protocol.toml").unwrap();
    }

    #[test]
    #[should_panic(expected = "You need to activate at least one IP version")]
    fn test_it_handles_incoherent_ip_config() {
        get_config("tests/incoherent_ip.toml").unwrap();
    }
}
