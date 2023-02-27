use crate::BoxResult;
use simple_error::bail;
use std::collections::HashMap;

pub fn validate_countries_list(list: &Option<Vec<String>>) -> BoxResult<()> {
    let countries = get_countries();

    for country in list.as_ref().unwrap_or(&vec![]) {
        if countries.get(&country.to_lowercase()).is_none() {
            bail!("{} does not provide any pacman mirror", country);
        }
    }

    Ok(())
}

macro_rules! lowercase_hm {
    ($($key: expr => $val: expr),+ $(,)?) => {
        {
            let mut hm = HashMap::new();
            $(
                hm.insert($key.to_string().to_lowercase(), $val.to_string().to_lowercase());
            )*
            hm
        }
    };
}

pub fn get_countries() -> HashMap<String, String> {
    lowercase_hm! (
    "Australia" => "AU",
    "Austria" => "AT",
    "Bangladesh" => "BD",
    "Belarus" => "BY",
    "Belgium" => "BE",
    "Bosnia and Herzegovina" => "BA",
    "Brazil" => "BR",
    "Bulgaria" => "BG",
    "Canada" => "CA",
    "Chile" => "CL",
    "China" => "CN",
    "Colombia" => "CO",
    "Croatia" => "HR",
    "Czechia" => "CZ",
    "Denmark" => "DK",
    "Ecuador" => "EC",
    "France" => "FR",
    "Finland" => "FI",
    "Georgia" => "GE",
    "Germany" => "DE",
    "Greece" => "GR",
    "Hong Kong" => "HK",
    "Hungary" => "HU",
    "Iceland" => "IS",
    "India" => "IN",
    "Indonesia" => "ID",
    "Iran" => "IR",
    "Ireland" => "IE",
    "Israel" => "IL",
    "Italy" => "IT",
    "Japan" => "JP",
    "Kazakhstan" => "KZ",
    "Kenya" => "KE",
    "Latvia" => "LV",
    "Lithuania" => "LT",
    "Luxembourg" => "LU",
    "Netherlands" => "NL",
    "New Caledonia" => "NC",
    "New Zealand" => "NZ",
    "North Macedonia" => "MK",
    "Norway" => "NO",
    "Paraguay" => "PY",
    "Philippines" => "PH",
    "Poland" => "PL",
    "Portugal" => "PT",
    "Romania" => "RO",
    "Russia" => "RU",
    "Serbia" => "RS",
    "Singapore" => "SG",
    "Slovakia" => "SK",
    "Slovenia" => "SI",
    "South Africa" => "ZA",
    "South Korea" => "KR",
    "Spain" => "ES",
    "Sweden" => "SE",
    "Switzerland" => "CH",
    "Taiwan" => "TW",
    "Thailand" => "TH",
    "Turkey" => "TR",
    "Ukraine" => "UA",
    "United Kingdom" => "GB",
    "United States" => "US",
    "Vietnam" => "VN",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_countries() {
        assert_eq!(63, get_countries().len());
    }

    #[test]
    fn it_can_validate_config() {
        let countries = Some(vec!["France".to_string()]);
        assert_eq!((), validate_countries_list(&countries).unwrap());
    }

    #[test]
    #[should_panic(expected = "not a country does not provide any pacman mirror")]
    fn it_invalidates_config() {
        let countries = Some(vec!["not a country".to_string()]);
        validate_countries_list(&countries).unwrap();
    }
}
