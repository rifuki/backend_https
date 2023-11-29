use actix_web::http::StatusCode;
use argon2::{
    password_hash::{
        SaltString,  
        rand_core::OsRng
    },
    PasswordHasher,
    Argon2,
    PasswordHash, 
    PasswordVerifier,
};

use crate::{
    db::DbPool, 
    errors::{
        AppError, 
        AppErrorMessage
    },
    auth::model::Credentials
};

/* * check username return bool */
pub async fn check_username(
    db_pool: &DbPool, 
    username: &str
) -> bool 
{
    let sql_query = sqlx::query("SELECT username FROM credentials WHERE username = ?");
    sql_query
        .bind(username)
        .fetch_one(db_pool)
        .await
        .is_ok()
}
/* * end check username return bool */

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

/* * hashing user password */
pub fn hashing_password(
    user_payload_password: &str
) -> Result<String, AppError>
{
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    
    let hashed_password = argon2.hash_password(
        user_payload_password.as_bytes(),
        &salt
    )
    .map_err(|e| {
        log::error!("error hashing user password {}", e);
        let app_err_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("failed to hash password: {}", user_payload_password),
            details: Some(e.to_string())
        };
        AppError::InternalServerError(app_err_message.into())
    })?;

    Ok(hashed_password.to_string())
}
/* * end hashing user password */

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