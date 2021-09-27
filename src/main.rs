use reqwest;
use clap::clap_app;
use serde::{Deserialize};
use std::collections::HashMap;


pub fn kwabang_api(mode: &str, query: &str) -> String {
    let url = String::from("https://api.kwabang.net/");
    return format!("{}/{}/{}", url, mode ,query);
}


#[derive(Deserialize)]
struct Asn {
    autonomous_system_number: u64,
    autonomous_system_organization: String,
}

#[derive(Deserialize)]
struct CountryContinent {
    code: String,
    geoname_id: u64,
    names: HashMap<String, String>,
}

#[derive(Deserialize)]
struct CountryCountry {
    geoname_id: u64,
    iso_code: String,
    names: HashMap<String, String>,
}

#[derive(Deserialize)]
struct CountryRegisteredCountry {
    geoname_id: u64,
    iso_code: String,
    names: HashMap<String, String>,
}


#[derive(Deserialize)]
struct Country {
    continent: CountryContinent,
    country: CountryCountry,
    registered_country: CountryRegisteredCountry,
}

#[derive(Deserialize)]
struct CityLocation {
    accuracy_radius: u64,
    latitude: f64,
    longitude: f64,
    time_zone: String,
}

#[derive(Deserialize)]
struct City {
    location: CityLocation,
}

#[derive(Deserialize)]
struct GeoLocation {
    ISP: String,
    registered_country_code: String,
    registered_country: String,
    city: Option<String>,
    country_code: String,
    country: String,
    continent_code: String,
    continent: String,
    location: CityLocation
}

fn fetch_asn(asn: &str) -> Result<Asn, reqwest::Error> {
    let url = kwabang_api("asn", asn);
    let req = reqwest::blocking::get(url)?
        .json::<Asn>(); 
    return req;
}


fn fetch_country(query_ip: &str) -> Result<Country, reqwest::Error> {
    let url = kwabang_api("country", query_ip);
    let req = reqwest::blocking::get(url)?
        .json::<Country>();
    return req;
}

fn fetch_city(query_ip: &str) -> Result<City, reqwest::Error> {
    let url = kwabang_api("city", query_ip);
    let req = reqwest::blocking::get(url)?
        .json::<City>();
    return req;
}

fn fetch_geolocation(query_ip: &str) -> Result<GeoLocation, reqwest::Error> {
    let url = kwabang_api("geolocation", query_ip);
    let req = reqwest::blocking::get(url)?
        .json::<GeoLocation>();
    return req;
}

fn get_as_number(query_ip: &str){
    // ASN
    let asn_data = fetch_asn(query_ip);
    let asn_data = match asn_data {
        Ok(asn_data) => asn_data,
        Err(e) => { panic!("Error: {}", e)}
    };
    let autonomous_system_number = asn_data.autonomous_system_number;
    let autonomous_system_organization = asn_data.autonomous_system_organization;    
    println!("AS Number: AS{}", autonomous_system_number);
    println!("Organization: {}", autonomous_system_organization);
    
}

fn get_country(query_ip: &str) {
    // Continent, Country, Registered Country
    let country_data = fetch_country(query_ip);
    let country_data = match country_data {
        Ok(country_data) => country_data,
        Err(e) => { panic!("Error: {}", e)}
    };

    let country_data_continent_code = country_data.continent.code;
    let country_data_continent_geoname_id = country_data.continent.geoname_id;
    let country_data_continent_names_en = country_data.continent.names.get("en");

    let country_data_country_geoname_id = country_data.country.geoname_id;
    let country_data_country_iso_code = country_data.country.iso_code;
    let country_data_country_names_en = country_data.country.names.get("en");
    
    let country_data_registered_country_geoname_id = country_data.registered_country.geoname_id;
    let country_data_registered_country_iso_code = country_data.registered_country.iso_code;
    let country_data_registered_country_names_en = country_data.registered_country.names.get("en");

    // Continent
    println!("Continent code: {}", country_data_continent_code);
    println!("Continent GeoName ID: {}", country_data_continent_geoname_id);
    println!("Continent name: {}", country_data_continent_names_en.unwrap());
    
    // Country
    println!("Country GeoName ID: {}", country_data_country_geoname_id);
    println!("Country ISO code: {}", country_data_country_iso_code);
    println!("Country name: {}", country_data_country_names_en.unwrap());
    
    // Registered Country
    println!("Registered country GeoName ID: {}", country_data_registered_country_geoname_id);
    println!("Registered country ISO code: {}", country_data_registered_country_iso_code);
    println!("Registered country name: {}", country_data_registered_country_names_en.unwrap());
}

fn get_city(query_ip: &str) {
    // City
    let city_data = fetch_city(query_ip);
    let city_data = match city_data {
        Ok(city_data) => city_data,
        Err(e) => { panic!("Error: {}", e)}
    };

    let city_data_location_accuracy_radius = city_data.location.accuracy_radius;
    let city_data_location_latitude = city_data.location.latitude;
    let city_data_location_longitude = city_data.location.longitude;
    let city_data_location_time_zone = city_data.location.time_zone;

    println!("City accuracy radius: {}", city_data_location_accuracy_radius);
    println!("City latitude: {}", city_data_location_latitude);
    println!("City longitude: {}", city_data_location_longitude);
    println!("City time zone: {}", city_data_location_time_zone);
}

fn get_geolocation(query_ip: &str) {
    // GeoLocation
    let geolocation_data = fetch_geolocation(query_ip);
    let geolocation_data = match geolocation_data {
        Ok(geolocation_data) => geolocation_data,
        Err(e) => { panic!("Error: {}", e)}
    };

    let geolocation_data_isp = geolocation_data.ISP;
    let geolocation_data_registered_country_code = geolocation_data.registered_country_code;
    let geolocation_data_registered_country = geolocation_data.registered_country;
    let mut geolocation_data_city = geolocation_data.city;
    if geolocation_data_city.is_none() {
        geolocation_data_city = Some("None".to_string());
    }
    let geolocation_data_country_code = geolocation_data.country_code;
    let geolocation_data_country = geolocation_data.country;
    let geolocation_data_continent_code = geolocation_data.continent_code;
    let geolocation_data_continent = geolocation_data.continent;
    let geolocation_data_location_accuracy_radius = geolocation_data.location.accuracy_radius;
    let geolocation_data_location_latitude = geolocation_data.location.latitude;
    let geolocation_data_location_longitude = geolocation_data.location.longitude;
    let geolocation_data_location_time_zone = geolocation_data.location.time_zone;
    
    println!("ISP: {}", geolocation_data_isp);
    println!("Registered country code: {}", geolocation_data_registered_country_code);
    println!("Registered country: {}", geolocation_data_registered_country);
    println!("City: {}", geolocation_data_city.unwrap());
    println!("Country code: {}", geolocation_data_country_code);
    println!("Country: {}", geolocation_data_country);
    println!("Continent code: {}", geolocation_data_continent_code);
    println!("Continent: {}", geolocation_data_continent);
    println!("Location accuracy radius: {}", geolocation_data_location_accuracy_radius);
    println!("Location latitude: {}", geolocation_data_location_latitude);
    println!("Location longitude: {}", geolocation_data_location_longitude);
    println!("Location time zone: {}", geolocation_data_location_time_zone);
}


fn main() {
    let matches = clap_app!(ip_info =>
        (version: "0.1.0")
        (author: "zeroday0619 <zeroday0619_dev@outlook.com>")
        (about: "Get information about an IP address")
        (@arg IP: -i --ip +required +takes_value "IP address")
        (@arg mode: -m --mode +required +takes_value "Mode: [0: asn ,1: country, 2: city, 3: geolocation]")
    ).get_matches();

    if let Some(mode) = matches.value_of("mode") {
        if let Some(ip) = matches.value_of("IP"){
            match mode {
                "0" => get_as_number(ip),
                "1" => get_country(ip),
                "2" => get_city(ip),
                "3" => get_geolocation(ip),
                _ => println!("Mode not found")
            }
        }
    }
}
