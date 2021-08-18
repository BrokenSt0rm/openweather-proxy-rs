use std::sync::Arc;

use crate::infrastructure::services::weather::WeatherService;

pub struct AppData {
    pub weather_service: Arc<dyn WeatherService>,
}

impl AppData {
    pub fn get_weather_service(&self) -> &dyn WeatherService {
        &*self.weather_service
    }
}
