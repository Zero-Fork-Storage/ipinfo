use clap::clap_app;

use kwabang::getters;

mod kwabang;
mod models;

fn main() -> Result<(), String> {
    let matches = clap_app!(ip_info =>
        (version: "0.1.0")
        (author: "zeroday0619 <zeroday0619_dev@outlook.com>")
        (about: "Get information about an IP address")
        (@arg IP: -i --ip +required +takes_value "IP address")
        (@arg mode: -m --mode +required +takes_value "Mode: [0: asn ,1: country, 2: city, 3: geolocation]")
    ).get_matches();

    let mode = matches.value_of("mode").ok_or("no mode is found")?;
    let ip = matches.value_of("IP").ok_or("no IP is found")?;

    match mode {
        "0" => getters::get_as_number(ip),
        "1" => getters::get_country(ip),
        "2" => getters::get_city(ip),
        "3" => getters::get_geolocation(ip),
        _ => Err("mode can be only {0, 1, 2, 3}".to_owned()),
    }
}
