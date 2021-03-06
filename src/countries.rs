use crate::config::Config;
use crate::BoxResult;
use simple_error::bail;
use std::collections::HashMap;

pub fn validate_config(config: &Config) -> BoxResult<()> {
    let countries = get_countries();

    for country in config.countries.as_ref().unwrap_or(&vec![]) {
        if countries.get(&country.to_lowercase()).is_none() {
            bail!("{} does not provide any pacman mirror", country);
        }
    }

    Ok(())
}

pub fn get_countries() -> HashMap<String, String> {
    let mut countries = HashMap::new();

    countries.insert(
        "Australia".to_string().to_lowercase(),
        "AU".to_string().to_lowercase(),
    );
    countries.insert(
        "Austria".to_string().to_lowercase(),
        "AT".to_string().to_lowercase(),
    );
    countries.insert(
        "Bangladesh".to_string().to_lowercase(),
        "BD".to_string().to_lowercase(),
    );
    countries.insert(
        "Belarus".to_string().to_lowercase(),
        "BY".to_string().to_lowercase(),
    );
    countries.insert(
        "Belgium".to_string().to_lowercase(),
        "BE".to_string().to_lowercase(),
    );
    countries.insert(
        "Bosnia and Herzegovina".to_string().to_lowercase(),
        "BA".to_string().to_lowercase(),
    );
    countries.insert(
        "Brazil".to_string().to_lowercase(),
        "BR".to_string().to_lowercase(),
    );
    countries.insert(
        "Bulgaria".to_string().to_lowercase(),
        "BG".to_string().to_lowercase(),
    );
    countries.insert(
        "Canada".to_string().to_lowercase(),
        "CA".to_string().to_lowercase(),
    );
    countries.insert(
        "Chile".to_string().to_lowercase(),
        "CL".to_string().to_lowercase(),
    );
    countries.insert(
        "China".to_string().to_lowercase(),
        "CN".to_string().to_lowercase(),
    );
    countries.insert(
        "Colombia".to_string().to_lowercase(),
        "CO".to_string().to_lowercase(),
    );
    countries.insert(
        "Croatia".to_string().to_lowercase(),
        "HR".to_string().to_lowercase(),
    );
    countries.insert(
        "Czechia".to_string().to_lowercase(),
        "CZ".to_string().to_lowercase(),
    );
    countries.insert(
        "Denmark".to_string().to_lowercase(),
        "DK".to_string().to_lowercase(),
    );
    countries.insert(
        "Ecuador".to_string().to_lowercase(),
        "EC".to_string().to_lowercase(),
    );
    countries.insert(
        "Finland".to_string().to_lowercase(),
        "FI".to_string().to_lowercase(),
    );
    countries.insert(
        "France".to_string().to_lowercase(),
        "FR".to_string().to_lowercase(),
    );
    countries.insert(
        "Georgia".to_string().to_lowercase(),
        "GE".to_string().to_lowercase(),
    );
    countries.insert(
        "Germany".to_string().to_lowercase(),
        "DE".to_string().to_lowercase(),
    );
    countries.insert(
        "Greece".to_string().to_lowercase(),
        "GR".to_string().to_lowercase(),
    );
    countries.insert(
        "Hong Kong".to_string().to_lowercase(),
        "HK".to_string().to_lowercase(),
    );
    countries.insert(
        "Hungary".to_string().to_lowercase(),
        "HU".to_string().to_lowercase(),
    );
    countries.insert(
        "Iceland".to_string().to_lowercase(),
        "IS".to_string().to_lowercase(),
    );
    countries.insert(
        "India".to_string().to_lowercase(),
        "IN".to_string().to_lowercase(),
    );
    countries.insert(
        "Indonesia".to_string().to_lowercase(),
        "ID".to_string().to_lowercase(),
    );
    countries.insert(
        "Iran".to_string().to_lowercase(),
        "IR".to_string().to_lowercase(),
    );
    countries.insert(
        "IE".to_string().to_lowercase(),
        "Ireland".to_string().to_lowercase(),
    );
    countries.insert(
        "IL".to_string().to_lowercase(),
        "Israel".to_string().to_lowercase(),
    );
    countries.insert(
        "Italy".to_string().to_lowercase(),
        "IT".to_string().to_lowercase(),
    );
    countries.insert(
        "Japan".to_string().to_lowercase(),
        "JP".to_string().to_lowercase(),
    );
    countries.insert(
        "Kazakhstan".to_string().to_lowercase(),
        "KZ".to_string().to_lowercase(),
    );
    countries.insert(
        "Kenya".to_string().to_lowercase(),
        "KE".to_string().to_lowercase(),
    );
    countries.insert(
        "Latvia".to_string().to_lowercase(),
        "LV".to_string().to_lowercase(),
    );
    countries.insert(
        "Lithuania".to_string().to_lowercase(),
        "LT".to_string().to_lowercase(),
    );
    countries.insert(
        "Luxembourg".to_string().to_lowercase(),
        "LU".to_string().to_lowercase(),
    );
    countries.insert(
        "Netherlands".to_string().to_lowercase(),
        "NL".to_string().to_lowercase(),
    );
    countries.insert(
        "New Caledonia".to_string().to_lowercase(),
        "NC".to_string().to_lowercase(),
    );
    countries.insert(
        "New Zealand".to_string().to_lowercase(),
        "NZ".to_string().to_lowercase(),
    );
    countries.insert(
        "North Macedonia".to_string().to_lowercase(),
        "MK".to_string().to_lowercase(),
    );
    countries.insert(
        "Norway".to_string().to_lowercase(),
        "NO".to_string().to_lowercase(),
    );
    countries.insert(
        "Paraguay".to_string().to_lowercase(),
        "PY".to_string().to_lowercase(),
    );
    countries.insert(
        "Philippines".to_string().to_lowercase(),
        "PH".to_string().to_lowercase(),
    );
    countries.insert(
        "Poland".to_string().to_lowercase(),
        "PL".to_string().to_lowercase(),
    );
    countries.insert(
        "Portugal".to_string().to_lowercase(),
        "PT".to_string().to_lowercase(),
    );
    countries.insert(
        "Romania".to_string().to_lowercase(),
        "RO".to_string().to_lowercase(),
    );
    countries.insert(
        "RU".to_string().to_lowercase(),
        "Russia".to_string().to_lowercase(),
    );
    countries.insert(
        "RS".to_string().to_lowercase(),
        "Serbia".to_string().to_lowercase(),
    );
    countries.insert(
        "SG".to_string().to_lowercase(),
        "Singapore".to_string().to_lowercase(),
    );
    countries.insert(
        "SK".to_string().to_lowercase(),
        "Slovakia".to_string().to_lowercase(),
    );
    countries.insert(
        "Slovenia".to_string().to_lowercase(),
        "SI".to_string().to_lowercase(),
    );
    countries.insert(
        "South Africa".to_string().to_lowercase(),
        "ZA".to_string().to_lowercase(),
    );
    countries.insert(
        "South Korea".to_string().to_lowercase(),
        "KR".to_string().to_lowercase(),
    );
    countries.insert(
        "Spain".to_string().to_lowercase(),
        "ES".to_string().to_lowercase(),
    );
    countries.insert(
        "Sweden".to_string().to_lowercase(),
        "SE".to_string().to_lowercase(),
    );
    countries.insert(
        "Switzerland".to_string().to_lowercase(),
        "CH".to_string().to_lowercase(),
    );
    countries.insert(
        "Taiwan".to_string().to_lowercase(),
        "TW".to_string().to_lowercase(),
    );
    countries.insert(
        "Thailand".to_string().to_lowercase(),
        "TH".to_string().to_lowercase(),
    );
    countries.insert(
        "Turkey".to_string().to_lowercase(),
        "TR".to_string().to_lowercase(),
    );
    countries.insert(
        "Ukraine".to_string().to_lowercase(),
        "UA".to_string().to_lowercase(),
    );
    countries.insert(
        "United Kingdom".to_string().to_lowercase(),
        "GB".to_string().to_lowercase(),
    );
    countries.insert(
        "United States".to_string().to_lowercase(),
        "US".to_string().to_lowercase(),
    );
    countries.insert(
        "Vietnam".to_string().to_lowercase(),
        "VN".to_string().to_lowercase(),
    );

    countries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_countries() {
        assert_eq!(63, get_countries().len());
    }

    #[test]
    fn it_can_validate_config() -> BoxResult<()> {
        let countries = Some(vec!["France".to_string()]);

        let config: Config = Config {
            http: false,
            https: true,
            ipv4: false,
            ipv6: true,
            countries,
        };

        assert_eq!((), validate_config(&config)?);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "not a country does not provide any pacman mirror")]
    fn it_invalidates_config() {
        let countries = Some(vec!["not a country".to_string()]);

        let config: Config = Config {
            http: false,
            https: true,
            ipv4: false,
            ipv6: true,
            countries,
        };

        validate_config(&config).unwrap();
    }
}
