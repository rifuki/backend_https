use actix_web::http::StatusCode;
use validator::Validate;

use crate::{
    AppState, 
    auth::{
        model::CredentialsSignup, 
        helpers::{
            check_username, 
            hashing_password
        }
    },
    errors::{
        AppError, 
        AppErrorMessage
    }
};

pub async fn signup_service(
    app_state: &AppState,
    payload: CredentialsSignup
) -> Result<String, AppError> 
{
    /* * validating user input */
    payload.validate()?;
    /* * end validating user input */

    /* * take db pool from handler */
    let db_pool = &app_state.db_pool;
    /* * end take db pool from handler */

    /* * checking username availability */
    let is_username_exists = check_username(db_pool, &payload.username).await;
    if is_username_exists {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::CONFLICT.as_u16(),
            message: format!("the username '{}' is already taken. please choose a different username.", &payload.username),
            details: None
        };
        return Err(AppError::Conflict(app_err_message.into()));
    }
    /* * end checking username availability */

    /* * hashing user password */
    let hashed_password = hashing_password(&payload.password)?;
    /* * end hashing user password */

    /* * saving user credentials */
    let sql_query = sqlx::query("INSERT INTO credentials (username, password) VALUES (?, ?)");
    let _query_result = sql_query
        .bind(&payload.username)
        .bind(hashed_password)
        .execute(db_pool)
        .await?;
    /* * end  saving user credentials */

    /* * checking user credentials stored in database */
    let is_credentials_saved = check_username(db_pool, &payload.username).await;
    if !is_credentials_saved {
        let app_error_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!("an unknown error occurred while processing your request. please try again later."),
            details: None 

        };
        return Err(AppError::InternalServerError(app_error_message.into()));
    }

    Ok(payload.username)
    /* * end checking user credentials stored in database */
}