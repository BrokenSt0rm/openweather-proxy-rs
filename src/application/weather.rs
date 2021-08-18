use actix_web::{get, web, HttpResponse};
use openweather_async::WeatherData;

use crate::types::{
    error::OpenWeatherProxyError,
    request::{ConditionRequest, LatLonRequest},
    response::{ConditionResponse, OpenWeatherProxySuccessResponse, ResponseMetadata},
    state::AppData,
};

#[get("/weather")]
pub async fn get_current_weather_route(
    query_params: web::Query<LatLonRequest>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, OpenWeatherProxyError> {
    let report = app_data
        .get_weather_service()
        .get_by_coordinates(
            query_params.lat,
            query_params.lon,
            query_params.units.clone(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(OpenWeatherProxySuccessResponse {
        metadata: ResponseMetadata {
            error: None,
            code: 200,
        },
        data: report,
    }))
}

#[get("/condition")]
pub async fn get_condition_route(
    query_params: web::Query<ConditionRequest>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, OpenWeatherProxyError> {
    let report = app_data
        .get_weather_service()
        .get_by_coordinates(
            query_params.lat,
            query_params.lon,
            query_params.units.clone(),
        )
        .await?;

    let weather_data = report
        .weather
        .unwrap_or_default()
        .into_iter()
        .next()
        .unwrap_or(WeatherData {
            id: u32::default(),
            main: String::default(),
            description: String::default(),
            icon: String::default(),
        });

    Ok(HttpResponse::Ok().json(OpenWeatherProxySuccessResponse {
        metadata: ResponseMetadata {
            error: None,
            code: 200,
        },
        data: ConditionResponse {
            name: query_params.name.clone(),
            temp: report.main.temp,
            description: weather_data.description,
            icon: weather_data.icon,
        },
    }))
}
