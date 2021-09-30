use std::fmt::{Display, Formatter};

use reqwest::Error;
use serde::Deserialize;

use crate::kwabang::fetchers::fetch_asn;
use crate::models::KwabangType;

#[derive(Deserialize)]
pub struct Asn {
    pub autonomous_system_number: u64,
    pub autonomous_system_organization: String,
}

impl Display for Asn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "AS Number: AS{}
Organization: {}",
            self.autonomous_system_number, &self.autonomous_system_organization
        );
    }
}

impl KwabangType for Asn {
    type Output = Asn;

    fn mode() -> &'static str {
        "asn"
    }

    fn request(arg: &str) -> Result<Self::Output, Error> {
        fetch_asn(arg)
    }
}
