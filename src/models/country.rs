use std::fmt::{Display, Formatter};

use reqwest::Error;
use serde::Deserialize;

use crate::kwabang::fetchers::fetch_country;
use crate::models::internal::country_internal::{
    CountryContinent, CountryCountry, CountryRegisteredCountry,
};
use crate::models::KwabangType;

#[derive(Deserialize)]
pub struct Country {
    pub continent: CountryContinent,
    pub country: CountryCountry,
    pub registered_country: CountryRegisteredCountry,
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            self.continent, self.country, self.registered_country
        )
    }
}

impl KwabangType for Country {
    type Output = Country;

    fn mode() -> &'static str {
        "country"
    }

    fn request(arg: &str) -> Result<Self::Output, Error> {
        fetch_country(arg)
    }
}
