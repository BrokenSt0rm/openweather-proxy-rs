use openweather_async::{Clouds, Coord, Main, WeatherData, Wind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub coord: Coord,
    pub weather: Option<Vec<WeatherData>>,
    pub base: Option<String>,
    pub main: Main,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub rain: Option<String>,
    pub snow: Option<String>,
    pub clouds: Clouds,
    pub dt: u32,
    pub sys: Option<Sys>,
    pub timezone: Option<i32>,
    pub id: i32,
    pub name: String,
    pub cod: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub message_type: Option<u32>,
    pub id: Option<u32>,
    pub country: Option<String>,
    pub sunrise: Option<u32>,
    pub sunset: Option<u32>,
}
