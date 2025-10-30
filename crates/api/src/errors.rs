use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct ApiErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

impl ResponseError for ApiErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(self)
    }
}

impl ApiErrorResponse {
    pub fn bad_request(message: impl Into<String>) -> Self {
        ApiErrorResponse {
            error: "BAD_REQUEST".into(),
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        ApiErrorResponse {
            error: "INTERNAL_ERROR".into(),
            message: message.into(),
        }
    }
}