use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use thiserror::Error;

use crate::domain::merchant::PasswordError;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("A database error occured")]
    Database(#[from] sqlx::Error),

    #[error("Error occured while processing a password")]
    Password(#[from] PasswordError),
}

impl ResponseError for Errors {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Errors::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::Password(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({
            "status": "error",
            "message": self.to_string()
        }))
    }
}
