use crate::config::Config;
use crate::BoxResult;
use std::collections::HashMap;

pub fn build_uri(config: &Config, countries_list: &HashMap<String, String>) -> BoxResult<String> {
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

    Ok(format!(
        "{}{}{}{}{}{}",
        base_uri, ipv4, ipv6, http, https, countries
    ))
}
