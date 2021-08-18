use rust_decimal::Decimal;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Units {
    Celsius,
    Fahrenheit,
    Kelvin,
}
#[derive(Deserialize)]
pub struct LatLonRequest {
    pub lat: Decimal,
    pub lon: Decimal,
    pub units: Units,
}

#[derive(Deserialize)]
pub struct ConditionRequest {
    pub lat: Decimal,
    pub lon: Decimal,
    pub units: Units,
    pub name: String,
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Units::Celsius => write!(f, "metric"),
            Units::Fahrenheit => write!(f, "imperial"),
            Units::Kelvin => write!(f, "standard"),
        }
    }
}
