use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct CityLocation {
    pub(crate) accuracy_radius: u64,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) time_zone: String,
}

impl Display for CityLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "City accuracy radius: {}
City latitude: {}
City longitude: {}
City time zone: {}",
            self.accuracy_radius, self.latitude, self.longitude, self.time_zone
        )
    }
}
