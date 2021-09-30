use std::fmt::Display;

pub(crate) mod asn;
pub(crate) mod city;
pub(crate) mod country;
pub(crate) mod geolocation;
mod internal;

/// KwabangType is one of the responses from Kwabang API.
pub trait KwabangType {
    type Output: Display;
    fn mode() -> &'static str;
    fn request(arg: &str) -> Result<Self::Output, reqwest::Error>;
}
