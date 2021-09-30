use std::fmt::{Display, Formatter};

use reqwest::Error;
use serde::Deserialize;

use crate::kwabang::fetchers::fetch_city;
use crate::models::internal::city_internal::CityLocation;
use crate::models::KwabangType;

#[derive(Deserialize)]
pub struct City {
    pub location: CityLocation,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.location)
    }
}

impl KwabangType for City {
    type Output = City;

    fn mode() -> &'static str {
        "city"
    }

    fn request(arg: &str) -> Result<Self::Output, Error> {
        fetch_city(arg)
    }
}
