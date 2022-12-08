use actix_web::{HttpResponse as Response, ResponseError};
use derive_new::new;
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use sqlx::Error as SqlxError;
use validator::ValidationErrors;

#[derive(new, Debug, thiserror::Error)]
pub enum Error {
    #[error("400 Bad Request: {0}")]
    BadRequest(#[new(default)] JsonValue),
    #[error("401 Unauthorized: {0}")]
    Unauthorized(#[new(default)] JsonValue),
    #[error("403 Forbidden: {0}")]
    Forbidden(#[new(default)] JsonValue),
    #[error("404 Not Found: {0}")]
    NotFound(#[new(default)] JsonValue),
    #[error("409 Conflict: {0}")]
    Conflict(#[new(default)] JsonValue),
    #[error("422 Unprocessable Entity: {0}")]
    UnprocessableEntity(#[new(default)] JsonValue),
    #[error("500 Internal Server Error")]
    InternalServerError(#[new(default)] JsonValue),
}

impl ResponseError for Error {
    fn error_response(&self) -> Response {
        let to = |v: &JsonValue| json!({ "error": v });
        match self {
            Error::BadRequest(v) => Response::BadRequest().json(to(v)),
            Error::Unauthorized(v) => Response::Unauthorized().json(to(v)),
            Error::Forbidden(v) => Response::Forbidden().json(to(v)),
            Error::NotFound(v) => Response::NotFound().json(to(v)),
            Error::Conflict(v) => Response::Conflict().json(to(v)),
            Error::UnprocessableEntity(v) => Response::UnprocessableEntity().json(to(v)),
            Error::InternalServerError(v) => Response::InternalServerError().json(to(v)),
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();
        for (field, field_errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = field_errors
                .iter()
                .map(|error| json!(error.message))
                .collect();
            err_map.insert(field.to_string(), json!(errors));
        }
        Error::UnprocessableEntity(err_map.into())
    }
}

impl From<SqlxError> for Error {
    fn from(error: SqlxError) -> Self {
        match error {
            _ => Error::new_internal_server_error(),
        }
    }
}
