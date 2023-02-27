use reqwest::Url;
use serde::Deserialize;
use simple_error::bail;
use std::io::prelude::*;
use std::{fs::File, path::Path};

use crate::countries::get_countries;
use crate::{countries, BoxResult};

#[derive(Deserialize)]
pub struct Config {
    http: bool,
    https: bool,
    ipv4: bool,
    ipv6: bool,
    countries: Option<Vec<String>>,
}

impl Config {
    pub fn new<T: AsRef<Path>>(path: T) -> BoxResult<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = toml::from_str(contents.as_str())?;

        countries::validate_countries_list(&config.countries)?;

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

    pub fn uri(&self) -> BoxResult<String> {
        let countries_list = get_countries();
        let ipv4 = if self.ipv4 {
            vec![("ip_version", "4".to_string())]
        } else {
            vec![]
        };
        let ipv6 = if self.ipv6 {
            vec![("ip_version", "6".to_string())]
        } else {
            vec![]
        };
        let http = if self.http {
            vec![("protocol", "http".to_string())]
        } else {
            vec![]
        };
        let https = if self.https {
            vec![("protocol", "https".to_string())]
        } else {
            vec![]
        };
        let countries = self
            .countries
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|country| countries_list.get(&country.to_lowercase()))
            .map(|c| ("country", c.to_uppercase()))
            .collect::<Vec<(&str, String)>>();
        let url = Url::parse_with_params(
            "https://www.archlinux.org/mirrorlist/",
            [ipv4, ipv6, http, https, countries].concat(),
        )?;

        Ok(url.to_string())
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
        Config::new("tests/incoherent_protocol.toml").unwrap();
    }

    #[test]
    #[should_panic(expected = "You need to activate at least one IP version")]
    fn test_it_handles_incoherent_ip_config() {
        Config::new("tests/incoherent_ip.toml").unwrap();
    }

    #[test]
    fn it_build_uri() {
        get_configs_provider()
            .into_iter()
            .for_each(|(config, expected_uri)| assert_eq!(config.uri().unwrap(), expected_uri));
    }

    fn get_configs_provider() -> Vec<(Config, String)> {
        let config_without_countries = Config {
            http: false,
            https: true,
            ipv4: false,
            ipv6: true,
            countries: None,
        };

        let countries = Some(vec!["France".to_string()]);

        let full_config = Config {
            http: true,
            https: true,
            ipv4: true,
            ipv6: true,
            countries,
        };

        vec![
            (
                config_without_countries,
                "https://www.archlinux.org/mirrorlist/?ip_version=6&protocol=https".to_string(),
            ),
            (full_config, "https://www.archlinux.org/mirrorlist/?ip_version=4&ip_version=6&protocol=http&protocol=https&country=FR".to_string()),
        ]
    }
}
