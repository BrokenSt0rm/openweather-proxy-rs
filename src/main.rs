use std::sync::Arc;
use std::time::Duration;

use actix_ratelimit::{errors::ARError, MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv_codegen::dotenv;
use openweather_proxy::infrastructure::services::weather::WeatherServiceImpl;
use openweather_proxy::types::state::AppData;
use openweather_proxy::{
    application::weather::{self},
    types::error::OpenWeatherProxyError,
};
use r2d2_redis::{r2d2, RedisConnectionManager};

async fn not_found() -> Result<HttpResponse, OpenWeatherProxyError> {
    Err(OpenWeatherProxyError::NotFound)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let rate_limiter_store = MemoryStore::new();

    let manager = RedisConnectionManager::new(dotenv!("REDIS_URL")).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(AppData {
                weather_service: Arc::new(WeatherServiceImpl::new(pool.clone())),
            })
            .wrap(Logger::default())
            .wrap(Logger::new(
                "%a %{User-Agent}i X-Forwarded-For: %{X-Forwarded-For}i",
            ))
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(rate_limiter_store.clone()).start())
                    .with_interval(Duration::from_secs(
                        dotenv!("RATE_LIMIT_SECONDS_INTERVAL")
                            .parse::<u64>()
                            .unwrap_or(60),
                    ))
                    .with_max_requests(
                        dotenv!("RATE_LIMIT_MAX_REQUESTS_IN_INTERVAL")
                            .parse::<usize>()
                            .unwrap_or(100),
                    )
                    .with_identifier(|req| {
                        let connection_info = req.connection_info();
                        let real_ip = connection_info.realip_remote_addr();

                        match real_ip {
                            Some(real_ip) => Ok(real_ip.to_string()),
                            None => Err(ARError::IdentificationError),
                        }
                    }),
            )
            .default_service(web::route().to(not_found))
            .service(weather::get_condition_route)
    })
    .bind(format!("{}:{}", dotenv!("HOST"), dotenv!("PORT")))?
    .run()
    .await
}
