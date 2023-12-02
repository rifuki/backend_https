use actix_web::{
    http::StatusCode, 
    cookie::Cookie
};
use sqlx::Row;

use crate::{
    errors::{
        AppError, 
        AppErrorMessage
    }, 
    AppState
};

pub async fn logout_service(
    app_state: &AppState,
    refresh_token_cookie: Option<Cookie<'_>>
) -> Result<String, AppError> 
{
    let refresh_token_cookie = refresh_token_cookie
        .map(|cookie| cookie.value().to_string() )
        .ok_or_else(|| {
            let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                code: StatusCode::UNAUTHORIZED.as_u16(),
                message: format!("you have been already logged out."),
                details: None
            };
            AppError::Unauthorized(app_err_message.into())
        })?;

    let db_pool = &app_state.db_pool;
    /* * check is refresh_token exist in database */
    let sql_query = sqlx::query("SELECT username, refresh_token FROM credentials WHERE refresh_token = ?"); 
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
    /* * check is refresh_token exist in database */
    
    /* * set stored_refresh_token to null */
    let sql_query = sqlx::query("UPDATE credentials SET refresh_token = null, max_age = null WHERE refresh_token = ?");
    let _ = sql_query
        .bind(stored_refresh_token)
        .execute(db_pool)
        .await?;
    /* * end set stored_refresh_token to null */
    /* * check is stored_refresh_token is null */
    let sql_query = sqlx::query("SELECT 1 FROM credentials WHERE refresh_token = ?"); 
    let query_result = sql_query
        .bind(&refresh_token_cookie)
        .fetch_one(db_pool)
        .await;
    if let Ok(_) = query_result {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!(
                "an unexpected error occurred while processing your request. refresh_token from user '{}' still exists in the database. please try again.",
                &stored_username
            ),
            details: None
        };
        return Err(AppError::InternalServerError(app_err_message.into()))
    }
    /* * end check is stored_refresh_token is null */

    Ok(stored_username)
}