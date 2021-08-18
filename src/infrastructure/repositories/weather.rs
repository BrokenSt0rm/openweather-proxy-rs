use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::Decimal;

use crate::types::{error::OpenWeatherProxyError, open_weather::Weather, request::Units};

#[async_trait]
pub trait WeatherRepository {
    async fn get_by_coordinates(
        &self,
        lat: Decimal,
        lon: Decimal,
        unit: Units,
    ) -> Result<Weather, OpenWeatherProxyError>;

    fn get_url(&self, operation: String, unit: Units) -> String;
}

pub struct WeatherRepositoryImpl {
    internal_client: Client,
    api_key: String,
}

impl WeatherRepositoryImpl {
    pub fn new(api_key: &str) -> WeatherRepositoryImpl {
        WeatherRepositoryImpl {
            internal_client: Client::new(),
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl WeatherRepository for WeatherRepositoryImpl {
    async fn get_by_coordinates(
        &self,
        lat: Decimal,
        lon: Decimal,
        unit: Units,
    ) -> Result<Weather, OpenWeatherProxyError> {
        let response: Weather = self
            .internal_client
            .get(&self.get_url(format!("weather?lat={}&lon={}", lat, lon), unit))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    fn get_url(&self, operation: String, unit: Units) -> String {
        let base_http = "https://api.openweathermap.org/data/2.5/";
        format!(
            "{}{}&appid={}&units={}",
            &base_http, &operation, self.api_key, unit
        )
    }
}
