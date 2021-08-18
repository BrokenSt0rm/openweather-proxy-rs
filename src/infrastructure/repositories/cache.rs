use async_trait::async_trait;
use openweather_async::Weather;
use r2d2_redis::{
    r2d2::{Pool, PooledConnection},
    redis::Commands,
    RedisConnectionManager,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::types::error::OpenWeatherProxyError;

#[async_trait]
pub trait CacheRepository<T> {
    async fn get(&self, key: String) -> Result<Option<T>, OpenWeatherProxyError>;

    async fn set(
        &self,
        key: String,
        exp: Option<usize>,
        value: &Weather,
    ) -> Result<(), OpenWeatherProxyError>;
}

pub struct CacheRepositoryImpl {
    pool: Pool<RedisConnectionManager>,
}

impl CacheRepositoryImpl {
    pub fn new(pool: Pool<RedisConnectionManager>) -> CacheRepositoryImpl {
        CacheRepositoryImpl { pool }
    }
    fn get_connection(&self) -> PooledConnection<RedisConnectionManager> {
        self.pool.get().unwrap()
    }
}

#[async_trait]
impl<'a, T: Sized + Send + Sync + Serialize + DeserializeOwned> CacheRepository<T>
    for CacheRepositoryImpl
where
    T: 'static,
{
    async fn get(&self, key: String) -> Result<Option<T>, OpenWeatherProxyError> {
        let mut connection = self.get_connection();
        let redis_stored: Option<String> = connection.get(key)?;

        if let Some(serialized_weather) = redis_stored {
            let deserialized: T = serde_json::from_str(&serialized_weather)?;

            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    async fn set(
        &self,
        key: String,
        exp: Option<usize>,
        value: &Weather,
    ) -> Result<(), OpenWeatherProxyError> {
        let mut connection = self.get_connection();

        if let Some(expiration) = exp {
            connection.set_ex::<String, String, String>(
                key,
                serde_json::to_string(&value)?,
                expiration,
            )?;
        } else {
            connection.set::<String, String, String>(key, serde_json::to_string(&value)?)?;
        }

        Ok(())
    }
}
