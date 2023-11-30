use actix_web::http::StatusCode;
use argon2::{
    password_hash::{
        SaltString,  
        rand_core::OsRng
    },
    PasswordHasher,
    Argon2,
};

use crate::{
    db::DbPool, 
    errors::{
        AppError, 
        AppErrorMessage
    }
};

/* * check is user info already taken / exists */
pub async fn check_user_insert_availability(
    db_pool: &DbPool,
    username: &str,
    email: &Option<String>,
    phone_number: &Option<String>,
) -> Result<(bool, bool, bool), AppError> {
    let query_result = sqlx::query!(
        r#"
        SELECT
            EXISTS (SELECT 1 FROM user WHERE username = ?) AS username_exists,
            EXISTS (SELECT 1 FROM user WHERE email = ?) AS email_exists, 
            EXISTS (SELECT 1 FROM user WHERE phone_number = ?) AS phone_number_exists
        "#,
        username,
        email,
        phone_number
    )
        .fetch_one(db_pool)
        .await?;

    Ok(
        (
            query_result.username_exists != 0,
            query_result.email_exists != 0,
            query_result.phone_number_exists != 0
        )
    )
}
/* * end check is user info already taken / exists */

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

/* * check user with custom query and return bool */
pub async fn get_stored_user(
    db_pool: &DbPool,
    query: &str,
    message: Option<String>,
    code: Option<u16>
) -> Result<sqlx::mysql::MySqlRow, AppError>
{
    let status_code = code.unwrap_or(StatusCode::NOT_FOUND.as_u16());
    let message = message.unwrap_or(String::from("Empty String"));

    let query_result = sqlx::query(query)
        .fetch_one(db_pool)
        .await
        .map_err(|e| {
            let app_err_message = AppErrorMessage {
                code: status_code,
                message: message,
                details: Some(e.to_string())
            };

            match code {
                Some(_) => AppError::InternalServerError(app_err_message.into()),
                None => AppError::NotFound(app_err_message.into())
            }
            
        })?;

    Ok (query_result)
}
/* * end check user with custom query and return bool */