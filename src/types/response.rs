use serde::Serialize;

#[derive(Serialize)]
pub struct OpenWeatherProxyErrorResponse {
    pub metadata: ResponseMetadata,
    pub data: Option<u8>,
}

#[derive(Serialize)]
pub struct ResponseMetadata {
    pub error: Option<String>,
    pub code: u16,
}

#[derive(Serialize)]
pub struct OpenWeatherProxySuccessResponse<T> {
    pub metadata: ResponseMetadata,
    pub data: T,
}
