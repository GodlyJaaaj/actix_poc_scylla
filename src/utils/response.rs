use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

pub fn success<T>(status: StatusCode, data: Option<T>) -> Response<T> {
    Response {
        status: status.as_u16(),
        message: None,
        data,
    }
}

pub fn error(status: StatusCode, message: String) -> Response<()> {
    Response {
        status: status.as_u16(),
        message: Some(message),
        data: None,
    }
}
