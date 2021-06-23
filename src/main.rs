use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv_codegen::dotenv;
use openweather_proxy::{
    application::weather::{self},
    types::error::OpenWeatherProxyError,
};

async fn not_found() -> Result<HttpResponse, OpenWeatherProxyError> {
    Err(OpenWeatherProxyError::NotFound)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(not_found))
            .service(weather::get_current_weather_route)
    })
    .bind(format!("{}:{}", dotenv!("HOST"), dotenv!("PORT")))?
    .run()
    .await
}
