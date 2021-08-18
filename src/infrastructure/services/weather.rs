use std::sync::Arc;

use async_trait::async_trait;
use dotenv_codegen::dotenv;
use r2d2_redis::{r2d2::Pool, RedisConnectionManager};
use rust_decimal::Decimal;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    infrastructure::repositories::{
        cache::{CacheRepository, CacheRepositoryImpl},
        weather::{WeatherRepository, WeatherRepositoryImpl},
    },
    types::{error::OpenWeatherProxyError, open_weather::Weather, request::Units},
};

#[async_trait(?Send)]
pub trait WeatherService {
    async fn get_by_coordinates(
        &self,
        lat: Decimal,
        lon: Decimal,
        unit: Units,
    ) -> Result<Weather, OpenWeatherProxyError>;
}

pub struct WeatherServiceImpl<T: Send> {
    weather_repository: Arc<dyn WeatherRepository>,
    cache_repository: Arc<dyn CacheRepository<T>>,
}

impl<'a, T> WeatherServiceImpl<T>
where
    T: 'static + Send + Sync + Serialize + DeserializeOwned,
{
    pub fn new(redis_pool: Pool<RedisConnectionManager>) -> WeatherServiceImpl<T> {
        WeatherServiceImpl {
            weather_repository: Arc::new(WeatherRepositoryImpl::new(dotenv!(
                "OPENWEATHER_API_KEY"
            ))),
            cache_repository: Arc::new(CacheRepositoryImpl::new(redis_pool)),
        }
    }
}

#[async_trait(?Send)]
impl<T: Sized + Send + Sync> WeatherService for WeatherServiceImpl<T>
where
    T: 'static,
    Weather: From<T>,
{
    async fn get_by_coordinates(
        &self,
        lat: Decimal,
        lon: Decimal,
        unit: Units,
    ) -> Result<Weather, OpenWeatherProxyError> {
        #[cfg(feature = "truncate_coordinates_decimal")]
        let lat = lat.trunc();
        #[cfg(feature = "truncate_coordinates_decimal")]
        let lon = lon.trunc();

        let cached_result: Option<Weather> = {
            let this = &self;
            &*this.cache_repository
        }
        .get(format!("{}:{}:{}", lat, lon, unit.clone()))
        .await?
        .map(Into::into);

        if let Some(weather) = cached_result {
            return Ok(weather);
        }

        let weather = {
            let this = &self;
            &*this.weather_repository
        }
        .get_by_coordinates(lat, lon, unit.clone())
        .await?;

        let expiration_from_env: usize = dotenv!("COORDINATES_CACHE_SECONDS").parse().unwrap();
        let expiration;

        if expiration_from_env > 0 {
            expiration = Some(expiration_from_env);
        } else {
            expiration = None;
        }

        {
            let this = &self;
            &*this.cache_repository
        }
        .set(
            format!("{}:{}:{}", lat, lon, unit.clone()),
            expiration,
            &weather,
        )
        .await?;

        Ok(weather)
    }
}
