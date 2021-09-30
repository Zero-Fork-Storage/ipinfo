use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use serde::Deserialize;

use crate::models::internal::{DefaultName, NOT_AVAILABLE_MESSAGE};

#[derive(Deserialize)]
pub struct CountryContinent {
    pub(crate) code: String,
    pub(crate) geoname_id: u64,
    pub(crate) names: HashMap<String, String>,
}

impl DefaultName for CountryContinent {
    fn get_default_name(&self) -> &str {
        self.names
            .get("en")
            .map(|x| x.as_str())
            .unwrap_or(NOT_AVAILABLE_MESSAGE)
    }
}

impl Display for CountryContinent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Continent code: {}
Continent GeoName ID: {}
Continent name: {}",
            self.code,
            self.geoname_id,
            self.get_default_name()
        )
    }
}

#[derive(Deserialize)]
pub struct CountryCountry {
    pub(crate) geoname_id: u64,
    pub(crate) iso_code: String,
    pub(crate) names: HashMap<String, String>,
}

impl DefaultName for CountryCountry {
    fn get_default_name(&self) -> &str {
        self.names
            .get("en")
            .map(|x| x.as_str())
            .unwrap_or(NOT_AVAILABLE_MESSAGE)
    }
}

impl Display for CountryCountry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Country GeoName ID: {}
Country ISO code: {}
Country name: {}",
            self.geoname_id,
            self.iso_code,
            self.get_default_name()
        )
    }
}

#[derive(Deserialize)]
pub struct CountryRegisteredCountry {
    pub(crate) geoname_id: u64,
    pub(crate) iso_code: String,
    pub(crate) names: HashMap<String, String>,
}

impl DefaultName for CountryRegisteredCountry {
    fn get_default_name(&self) -> &str {
        self.names
            .get("en")
            .map(|x| x.as_str())
            .unwrap_or(NOT_AVAILABLE_MESSAGE)
    }
}

impl Display for CountryRegisteredCountry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Registered country GeoName ID: {}
Registered country ISO code: {}
Registered country name: {}",
            self.geoname_id,
            self.iso_code,
            self.get_default_name()
        )
    }
}
