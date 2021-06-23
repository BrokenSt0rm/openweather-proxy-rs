use async_trait::async_trait;
use openweather_async::{OpenWeather, Units, Weather};

use crate::types::error::OpenWeatherProxyError;

#[async_trait]
pub trait WeatherRepository {
    async fn get_by_coordinates(
        &self,
        lat: f32,
        lon: f32,
    ) -> Result<Weather, OpenWeatherProxyError>;
}

pub struct WeatherRepositoryImpl {
    openweather_service: OpenWeather,
}

impl WeatherRepositoryImpl {
    pub async fn new(api_key: &str, unit: Units) -> WeatherRepositoryImpl {
        WeatherRepositoryImpl {
            openweather_service: OpenWeather::new(api_key, unit).await.unwrap(),
        }
    }
}

#[async_trait]
impl WeatherRepository for WeatherRepositoryImpl {
    async fn get_by_coordinates(
        &self,
        lat: f32,
        lon: f32,
    ) -> Result<Weather, OpenWeatherProxyError> {
        match self.openweather_service.get_by_coordinates(lat, lon).await {
            Ok(weather) => Ok(weather),
            Err(_) => Err(OpenWeatherProxyError::RepositoryError),
        }
    }
}
