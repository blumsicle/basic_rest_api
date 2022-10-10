use std::fmt::Display;

use actix_web::{
    body::BoxBody, http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorNoId {
    pub id: u32,
    pub err: String,
}

impl Display for ErrorNoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrorNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code()).json(self)
    }
}
