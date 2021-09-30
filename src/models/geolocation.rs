use std::fmt::{Display, Formatter};

use reqwest::Error;
use serde::Deserialize;

use crate::kwabang::fetchers::fetch_geolocation;
use crate::models::internal::city_internal::CityLocation;
use crate::models::KwabangType;

#[derive(Deserialize)]
pub struct GeoLocation {
    #[serde(rename = "ISP")]
    pub isp: String,
    pub registered_country_code: String,
    pub registered_country: String,
    pub city: Option<String>,
    pub country_code: String,
    pub country: String,
    pub continent_code: String,
    pub continent: String,
    pub location: CityLocation,
}

impl Display for GeoLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ISP: {}
Registered country code: {}
Registered country: {}
City: {}
Country code: {}
Country: {}
Continent code: {}
Continent: {}
{}",
            self.isp,
            self.registered_country_code,
            self.registered_country,
            self.city.as_ref().unwrap_or(&"None".to_string()),
            self.country_code,
            self.country,
            self.continent_code,
            self.continent,
            self.location
        )
    }
}

impl KwabangType for GeoLocation {
    type Output = GeoLocation;

    fn mode() -> &'static str {
        "geolocation"
    }

    fn request(arg: &str) -> Result<Self::Output, Error> {
        fetch_geolocation(arg)
    }
}
