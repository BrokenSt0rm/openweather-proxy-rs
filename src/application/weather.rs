use actix_web::{get, web, HttpResponse};
use dotenv_codegen::dotenv;

use crate::{
    infrastructure::repositories::weather::{WeatherRepository, WeatherRepositoryImpl},
    types::{
        error::OpenWeatherProxyError,
        request::LatLonRequest,
        response::{OpenWeatherProxySuccessResponse, ResponseMetadata},
    },
};

#[get("/weather")]
pub async fn get_current_weather_route(
    query_params: web::Query<LatLonRequest>,
) -> Result<HttpResponse, OpenWeatherProxyError> {
    let weather_repository: WeatherRepositoryImpl =
        WeatherRepositoryImpl::new(dotenv!("OPENWEATHER_API_KEY")).await;
    let report = weather_repository
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
