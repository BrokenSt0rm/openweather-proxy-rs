use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    Celsius,
    Fahrenheit,
    Kelvin,
}
#[derive(Deserialize)]
pub struct LatLonRequest {
    pub lat: f32,
    pub lon: f32,
    pub units: Units,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Units::Celsius => write!(f, "{:?}", "metric"),
            Units::Fahrenheit => write!(f, "{:?}", "imperial"),
            Units::Kelvin => write!(f, "{:?}", "standard"),
        }
    }
}
