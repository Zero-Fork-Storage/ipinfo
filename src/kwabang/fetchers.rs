use reqwest::Error;
use serde::de::DeserializeOwned;

use crate::kwabang;
use crate::models::asn::Asn;
use crate::models::city::City;
use crate::models::country::Country;
use crate::models::geolocation::GeoLocation;
use crate::models::KwabangType;

pub fn fetch_asn(asn: &str) -> Result<Asn, Error> {
    return fetch_internal(asn);
}

pub fn fetch_country(country: &str) -> Result<Country, Error> {
    return fetch_internal(country);
}

pub fn fetch_city(city: &str) -> Result<City, Error> {
    return fetch_internal(city);
}

pub fn fetch_geolocation(geolocation: &str) -> Result<GeoLocation, Error> {
    return fetch_internal(geolocation);
}

fn fetch_internal<T: KwabangType + DeserializeOwned>(arg: &str) -> Result<T, Error> {
    let url = kwabang::kwabang_api(T::mode(), arg);
    let req = reqwest::blocking::get(url)?.json::<T>();
    return req;
}
