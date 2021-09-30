use crate::models::asn::Asn;
use crate::models::city::City;
use crate::models::country::Country;
use crate::models::geolocation::GeoLocation;
use crate::models::KwabangType;

pub fn get_as_number(asn: &str) -> Result<(), String> {
    get_info_internal::<Asn>(asn)
}

pub fn get_country(country: &str) -> Result<(), String> {
    get_info_internal::<Country>(country)
}

pub fn get_city(city: &str) -> Result<(), String> {
    get_info_internal::<City>(city)
}

pub fn get_geolocation(geolocation: &str) -> Result<(), String> {
    get_info_internal::<GeoLocation>(geolocation)
}

fn get_info_internal<T: KwabangType>(arg: &str) -> Result<(), String> {
    let ret = T::request(arg).map_err(|x| x.to_string())?;
    println!("{}", ret);
    Ok(())
}
