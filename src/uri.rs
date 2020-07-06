use crate::config::Config;
use std::collections::HashMap;

pub fn build_uri(config: &Config, countries_list: &HashMap<String, String>) -> String {
    let base_uri = "https://www.archlinux.org/mirrorlist/?";
    let ipv4 = if config.ipv4 { "&ip_version=4" } else { "" };
    let ipv6 = if config.ipv6 { "&ip_version=6" } else { "" };
    let http = if config.http { "&protocol=http" } else { "" };
    let https = if config.https { "&protocol=https" } else { "" };
    let countries = config
        .countries
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|country| countries_list.get(&country.to_lowercase()))
        .fold("".to_string(), |acc, country| {
            format!("{}&country={}", acc, country.to_uppercase())
        });

    format!("{}{}{}{}{}{}", base_uri, ipv4, ipv6, http, https, countries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_build_uri() {
        let countries_list = crate::countries::get_countries();

        get_configs_provider()
            .into_iter()
            .for_each(|(config, expected_uri)| {
                assert_eq!(build_uri(&config, &countries_list), expected_uri)
            });
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
                "https://www.archlinux.org/mirrorlist/?&ip_version=6&protocol=https".to_string(),
            ),
            (full_config, "https://www.archlinux.org/mirrorlist/?&ip_version=4&ip_version=6&protocol=http&protocol=https&country=FR".to_string()),
        ]
    }
}
