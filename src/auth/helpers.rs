use actix_web::{
    http::StatusCode,
    cookie::time::Duration as ActixDuration
};
use argon2::{
    Argon2,
    PasswordHash, 
    PasswordVerifier,
};
use chrono::Utc;

use crate::{
    db::DbPool, 
    errors::{
        AppError, 
        AppErrorMessage
    },
    auth::model::Credentials
};

/* * get user credentials returns Result Query */
pub async fn get_user_credentials(
    db_pool: &DbPool,
    username: &str
) -> Result<Credentials, sqlx::Error>
{
    let sql_query = sqlx::query_as::<_, Credentials>("SELECT * FROM credentials WHERE username = ?");
    let query_result = sql_query
        .bind(username)
        .fetch_one(db_pool)
        .await;
    query_result
}
/* * get user credentials returns Result Query */

/* * verifying stored user password */
pub fn verify_password(
    user_payload_password: &str,
    stored_hashed_password: &str
) -> Result<bool, AppError>
{
    let parsed_stored_hashed_password = PasswordHash::new(stored_hashed_password).map_err(|e| {
        log::error!("error parsing hashed password {}", e);
        let app_error_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("failed to parse hashed password."),
            details: Some(e.to_string())
        };
        AppError::InternalServerError(app_error_message.into())
    })?;

    let argon2 = Argon2::default();
    let verified_password = argon2.verify_password(
        user_payload_password.as_bytes(), 
        &parsed_stored_hashed_password
    ).is_ok();

    Ok(verified_password)
}
/* * end verifying stored user password */

/* * convert timestamp to actix duration */
pub fn convert_timestamp_to_actix_duration(timestamp: i64) -> ActixDuration {
    let current_time = Utc::now().naive_utc().timestamp();
    let duration = timestamp - current_time;

    ActixDuration::seconds(duration)
}
/* * end convert timestamp to actix duration */