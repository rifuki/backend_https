use actix_web::{
    http::StatusCode, 
    cookie::Cookie,
};
use sqlx::Row;
use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation,
    Algorithm::HS256,
    errors::ErrorKind as JwtErrorKind,
    encode,
    EncodingKey,
    Header
};
use chrono::Utc;

use crate::{
    errors::{
        AppError, 
        AppErrorMessage
    }, 
    auth::{
        types::{
            Claims, 
            ServiceOkRefreshToken
        }, 
        constants::CHRONO_ACCESS_TOKEN_EXPIRED,
        helpers::convert_timestamp_to_actix_duration
    },
    AppState
};

pub async fn refresh_token_service(
    app_state: &AppState,
    refresh_token_cookie: Option<Cookie<'_>>
) -> Result<ServiceOkRefreshToken, AppError>
{
    /* * check and get refresh_token cookie value */
    let refresh_token_cookie = refresh_token_cookie.map(|cookie| cookie.value().to_string() )
    .ok_or_else(|| {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::UNAUTHORIZED.as_u16(),
            message: format!("missing or invalid refresh_token cookie"),
            details: None
        };
        AppError::Unauthorized(app_err_message.into())
    })?;
    /* * end check and get refresh_token cookie value */
    
    let db_pool = &app_state.db_pool;
    /* * check refresh_token_cookie is exists from db */
    let sql_query = sqlx::query("SELECT username, refresh_token, max_age FROM credentials WHERE refresh_token = ?");
    let query_result = sql_query
        .bind(&refresh_token_cookie)
        .fetch_one(db_pool)
        .await
        .map_err(|e| {
            let app_err_message = AppErrorMessage {
                code: StatusCode::NOT_FOUND.as_u16(),
                message: String::from("refresh_token not found in the database."),
                details: Some(e.to_string())
            };
            AppError::NotFound(app_err_message.into())
        })?;
    let stored_username = query_result.get::<String, _>("username");
    let stored_refresh_token = query_result.get::<&str, _>("refresh_token");
    let stored_max_age = query_result.get::<i64, _>("max_age");
    /* * end check refresh_token_cookie is exists from db */

    let secret_refresh_token = &app_state.secret_refresh_token;
    /* * decode refresh_token from db */
    let decoded_refresh_token = decode::<Claims>(
        &stored_refresh_token,
        &DecodingKey::from_secret(secret_refresh_token.as_ref()),
        &Validation::new(HS256)
    )
    .map_err(|e| {
        match e.kind() {
            JwtErrorKind::ExpiredSignature => {
                let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                    code: StatusCode::FORBIDDEN.as_u16(),
                    message: String::from("signature refresh_token is expired."),
                    details: None
                };
                AppError::Forbidden(app_err_message.into())
            },
            _ => {
                let app_err_message = AppErrorMessage {
                    code: StatusCode::FORBIDDEN.as_u16(),
                    message: String::from("failed to decoded refresh_token"),
                    details: Some(e.to_string())
                };
                AppError::Forbidden(app_err_message.into())
            }
        }
    })?;
    /* * end decode refresh_token from db */
    
    /* * check refresh_token is expired */
    let time_now = Utc::now().naive_utc().timestamp();
    if time_now >= decoded_refresh_token.claims.exp {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::FORBIDDEN.as_u16(),
            message: String::from("refresh token has been expired. please log in again."),
            details: None
        };
        return Err(AppError::Forbidden(app_err_message.into()))
    }
    /* * end check refresh_token is expired */

    /* * matching stored_username with decoded_refresh_token_username */
    if !stored_username.eq(&decoded_refresh_token.claims.username) {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::FORBIDDEN.as_u16(),
            message: String::from("stored_username is mismatch with decoded_refresh_token_username"),
            details: None
        };
        return Err(AppError::Forbidden(app_err_message.into()));
    }
    /* * end matching stored_username with decoded_refresh_token_username */

    /* * generate new encoded_access_token */
    let secret_access_token = &app_state.secret_access_token;
    let access_token_exp = (Utc::now().naive_utc() + *CHRONO_ACCESS_TOKEN_EXPIRED).timestamp();
    let claims = Claims {
        username: stored_username.to_string(),
        iat: time_now,
        exp: access_token_exp 
    };
    let new_encoded_access_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_access_token.as_ref())
    )
    .map_err(|e| {
        let app_err_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("failed to encode access token: {:#?}", claims),
            details: Some(e.to_string())
        };
        AppError::InternalServerError(app_err_message.into())
    })?;
    /* * end generate new encoded_access_token */

    /* * generate new encoded_refresh_token with previous exp date */
    let secret_refresh_token = &app_state.secret_refresh_token;
    let claims = Claims {
        username: stored_username.to_string(),
        iat: time_now,
        exp: decoded_refresh_token.claims.exp
    };
    let new_encoded_refresh_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_refresh_token.as_ref())
    )
    .map_err(|e| {
        let app_err_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("failed to encode refresh token: {:#?}", claims),
            details: Some(e.to_string())
        };
        AppError::InternalServerError(app_err_message.into())
    })?;
    /* * end generate new encoded_refresh_token with previous exp date */

    /* * stored new encoded refresh token to db.credentials */
    let sql_query = sqlx::query("UPDATE credentials SET refresh_token = ? WHERE refresh_token = ?");
    let _ = sql_query
        .bind(&new_encoded_refresh_token)
        .bind(&refresh_token_cookie)
        .execute(db_pool)
        .await?;
    /* * end stored new encoded refresh token to db.credentials */
    /* * check is new encoded refresh token stored in db.credentials */
    let sql_query = sqlx::query("SELECT refresh_token FROM credentials WHERE username = ?");
    let query_result = sql_query
        .bind(&stored_username)
        .fetch_one(db_pool)
        .await?;
    let stored_new_refresh_token = query_result.get::<&str, _>("refresh_token");
    if stored_new_refresh_token != new_encoded_refresh_token {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("an unexpected error occurred while processing your request. refresh_token is mismatch. please try again."),
            details: None
        };
        return Err(AppError::InternalServerError(app_err_message.into()));
    }
    /* * end check is new encoded refresh token stored in db.credentials */

    let refresh_token_previous_max_age = convert_timestamp_to_actix_duration(stored_max_age);
    Ok( 
        ServiceOkRefreshToken {
            access_token: new_encoded_access_token,
            refresh_token: new_encoded_refresh_token,
            refresh_token_previous_max_age
        }
    )
}