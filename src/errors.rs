use serde_json::{
    Value as JsonValue,
    json,
};
use thiserror::Error as ThisError;
use actix_web::{
    ResponseError,
    HttpResponse,
    body::BoxBody,
    http::StatusCode
};
use serde::Serialize;
use validator::ValidationErrors;
use sqlx::Error as SqlxError;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),
    #[error("Internal Server Error: {0}")]
    InternalServerError(JsonValue),

    #[error("Conflict: {0}")]
    Conflict(JsonValue),
    #[error("Not Found: {0}")]
    NotFound(JsonValue),
    #[error("Unauthorized: {0}")]
    Unauthorized(JsonValue),
    #[error("Forbidden: {0}")]
    Forbidden(JsonValue),
    #[error("See Other: {0}")]
    SeeOther(JsonValue),
}

/* * convert AppError to HttpResponse */
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match *self {
            Self::UnprocessableEntity(ref message) => HttpResponse::UnprocessableEntity().json(message),
            Self::InternalServerError(ref message) => HttpResponse::InternalServerError().json(message),
            Self::Conflict(ref message) => HttpResponse::Conflict().json(message),
            Self::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Self::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            Self::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Self::SeeOther(ref message) => HttpResponse::SeeOther().json(message)
        }
    }
}
/* * end convert AppError to HttpResponse */

/* * custom app json error message */
pub struct AppErrorMessage<T> {
    pub code: u16,
    pub message: String,
    pub details: Option<T>
}
impl<T> From<AppErrorMessage<T>> for JsonValue
where T: Serialize
{
    fn from(value: AppErrorMessage<T>) -> Self {
        match value.details {
            Some(details) => json!({
                "success": false,
                "error": json!({
                    "code": value.code,
                    "message": value.message,
                    "details": details
                })
            }),
            None => json!({
                "success": false,
                "error": json!({
                    "code": value.code,
                    "message": value.message,
                })
            })
        }
    }
}
/* * end custom app json error message */

/* * convert validate error to AppError */
impl From<ValidationErrors> for AppError {
    fn from(value: ValidationErrors) -> Self {
        let mut cleaned_errors = Vec::new();
        for (field_name, field_errors) in value.field_errors().iter() {
            let errors_msg = field_errors
                .iter()
                .map(|e| json!(e.message) )
                .collect::<Vec<JsonValue>>();
            cleaned_errors.push(
                json!({
                    field_name.to_string(): JsonValue::Array(errors_msg)
                })
            )
        }
        let error_status = StatusCode::UNPROCESSABLE_ENTITY;
        let app_error_message = AppErrorMessage {
            code: error_status.into(),
            message: String::from("validation failed. there are errors in the input data."),
            details: Some(cleaned_errors)
        };
        AppError::UnprocessableEntity(app_error_message.into())
    }
}
/* * convert validate error to AppError */

/* * convert database error to AppError */
impl From<SqlxError> for AppError {
    fn from(value: SqlxError) -> Self {
        log::error!("error: {}", value);
        let app_error_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("an database error occurred while processing your request."),
            details: Some(value.to_string())
        };
        Self::InternalServerError(app_error_message.into())
    }
}
/* * end convert database error to AppError */