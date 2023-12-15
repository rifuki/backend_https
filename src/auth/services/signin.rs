use actix_web::http::StatusCode;
use sqlx::Row;
use validator::Validate;
use chrono::Utc;
use jsonwebtoken::{
    encode,
    EncodingKey,
    Header
};

use crate::{
    types::AppState,
    auth::{
        model::CredentialsPayload, 
        helpers::{
            get_user_credentials,
            verify_password
        },
        types::{
            ServiceOkSignin,
            Claims
        },
        constants::{
            CHRONO_ACCESS_TOKEN_EXPIRED,
            CHRONO_REFRESH_TOKEN_EXPIRED
        }
    },
    errors::{
        AppError, 
        AppErrorMessage
    },
};

pub async fn signin_service(
    app_state: &AppState,
    payload: CredentialsPayload
) -> Result<ServiceOkSignin, AppError> 
{
    /* * validating user input */
    payload.validate()?;
    /* * end validating user input */

    /* * take db_pool from handler */
    let db_pool = &app_state.db_pool;
    /* * end take db_pool from handler */

    /* * get stored user credentials from database */
    let user_credentials = get_user_credentials(db_pool, &payload.username)
        .await
        .map_err(|e| {
            let app_error_message = AppErrorMessage {
                code: StatusCode::NOT_FOUND.as_u16(),
                message: format!("the provided user '{}' was not found in the database. please check the username or register first.", &payload.username),
                details: Some(e.to_string())
            };
            AppError::NotFound(app_error_message.into())
        })?;
    /* * get stored user credentials from database */


    /* * verifying user payload password with stored user password */
    let is_verified_password = verify_password(
        &payload.password,
        &user_credentials.password
    )?;
    if !is_verified_password {
        let app_error_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::UNAUTHORIZED.as_u16(),
            message: String::from("Oops! Authentication failed. Your credentials are incorrect. Please double-check your username and password."),
            details: None
        };
        return Err(AppError::Unauthorized(app_error_message.into()));
    }
    /* * end verifying user payload password with stored user password */

    let time_now = Utc::now().naive_utc().timestamp();

    // /* * check is user already logged in */
    // if !user_credentials.refresh_token.is_none() && (user_credentials.max_age.unwrap_or_default() > time_now) {
    //     let app_error_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
    //         code: StatusCode::SEE_OTHER.as_u16(),
    //         message: format!("you '{}'! are already authenticated.", &user_credentials.username),
    //         details: None
    //     };
    //     return Err(AppError::SeeOther(app_error_message.into()));
    // }
    // /* * end check is user already logged in */

    /* * generate jwt_encoded_access_token */
    let secret_access_token = &app_state.secret_access_token;
    let access_token_exp = (Utc::now().naive_utc() + *CHRONO_ACCESS_TOKEN_EXPIRED).timestamp();
    let claims_access_token = Claims {
        username: payload.username.clone(),
        iat: time_now,
        exp: access_token_exp
    };
    let encoded_access_token = encode(
        &Header::default(),
        &claims_access_token,
        &EncodingKey::from_secret(secret_access_token.as_ref())
    )
    .map_err(|e| {
        let app_err_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("failed to encode access token: {:#?}", claims_access_token),
            details: Some(e.to_string())
        };
        AppError::InternalServerError(app_err_message.into())
    })?;
    /* * end generate jwt_encoded_access_token */
    /* * generate jwt_encoded_refresh_token */
    let secret_refresh_token = &app_state.secret_refresh_token;
    let refresh_token_exp = (Utc::now().naive_utc() + *CHRONO_REFRESH_TOKEN_EXPIRED).timestamp();
    let claims_refresh_token = Claims {
        username: payload.username.clone(),
        iat: time_now,
        exp: refresh_token_exp
    };
    let encoded_refresh_token = encode(
        &Header::default(),
        &claims_refresh_token,
        &EncodingKey::from_secret(secret_refresh_token.as_ref())
    )
    .map_err(|e| {
        let app_err_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("failed to encode access token: {:#?}", claims_refresh_token),
            details: Some(e.to_string())
        };
        AppError::InternalServerError(app_err_message.into())
    })?;
    /* * end generate jwt_encoded_refresh_token */

    /* * stored jwt_encoded_refresh_token to credentials user */
    let sql_query = sqlx::query("UPDATE credentials SET refresh_token = ?, max_age = ? WHERE username = ?");
    let _ = sql_query
        .bind(&encoded_refresh_token)
        .bind(refresh_token_exp as i64)
        .bind(&payload.username)
        .execute(db_pool)
        .await?;
    /* * end stored jwt_encoded_refresh_token to credentials user */
    /* * check is jwt_encoded_refresh_token stored in credentials user */
    let sql_query = sqlx::query("SELECT refresh_token FROM credentials WHERE username = ?");
    let query_result = sql_query
        .bind(&payload.username)
        .fetch_one(db_pool)
        .await?;
    let stored_refresh_token = query_result.get::<&str, _>("refresh_token");
    if stored_refresh_token != encoded_refresh_token {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("refresh token is mismatch or not found."),
            details: None
        };
        return Err(AppError::InternalServerError(app_err_message.into()));
    }
    /* * end check is jwt_encoded_refresh_token stored in credentials user */

    Ok(
        ServiceOkSignin {
            username: payload.username,
            encoded_access_token,
            encoded_refresh_token
        }
    )
}
