use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

use super::response::{OpenWeatherProxyErrorResponse, ResponseMetadata};

#[derive(Error, Debug)]
pub enum OpenWeatherProxyError {
    #[error("Requested file was not found")]
    NotFound,
    #[error("You are forbidden to access requested file.")]
    Forbidden,
    #[error("Unknown Internal Error")]
    Unknown,
    #[error("OpenWeather is not available")]
    RepositoryError,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    ActixError(#[from] actix_web::error::Error),
}

impl ResponseError for OpenWeatherProxyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RepositoryError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ReqwestError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ActixError(..) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = OpenWeatherProxyErrorResponse {
            metadata: ResponseMetadata {
                error: Some(self.to_string()),
                code: status_code.as_u16(),
            },
            data: None,
        };
        HttpResponse::build(status_code).json(error_response)
    }
}
