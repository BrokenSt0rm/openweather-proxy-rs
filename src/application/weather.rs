use actix_web::{get, web, HttpResponse};
use dotenv_codegen::dotenv;
use openweather_async::Units;
use serde::Deserialize;

use crate::{
    infrastructure::repositories::weather::{WeatherRepository, WeatherRepositoryImpl},
    types::{
        error::OpenWeatherProxyError,
        response::{OpenWeatherProxySuccessResponse, ResponseMetadata},
    },
};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum UnitsDef {
    Metric,
    Fahrenheit,
    Kelvin,
}
#[derive(Deserialize)]
pub struct LatLonRequest {
    lat: f32,
    lon: f32,
    units: UnitsDef,
}

#[get("/weather")]
pub async fn get_current_weather_route(
    query_params: web::Query<LatLonRequest>,
) -> Result<HttpResponse, OpenWeatherProxyError> {
    let weather_repository: WeatherRepositoryImpl = WeatherRepositoryImpl::new(
        dotenv!("OPENWEATHER_API_KEY"),
        query_params.units.clone().into(),
    )
    .await;
    let report = weather_repository
        .get_by_coordinates(query_params.lat, query_params.lon)
        .await?;

    Ok(HttpResponse::Ok().json(OpenWeatherProxySuccessResponse {
        metadata: ResponseMetadata {
            error: None,
            code: 200,
        },
        data: report,
    }))
}

impl From<UnitsDef> for Units {
    fn from(unit: UnitsDef) -> Self {
        match unit {
            UnitsDef::Metric => Units::Metric,
            UnitsDef::Fahrenheit => Units::Fahrenheit,
            UnitsDef::Kelvin => Units::Kelvin,
        }
    }
}
