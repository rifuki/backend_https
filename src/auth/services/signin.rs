use actix_web::http::StatusCode;
use validator::Validate;

use crate::{
    AppState, 
    auth::{
        model::CredentialsPayload, 
        helpers::{
            get_user_credentials,
            verify_password
        }
    },
    errors::{
        AppError, 
        AppErrorMessage
    }
};

pub async fn signin_service(
    app_state: &AppState,
    payload: CredentialsPayload
) -> Result<String, AppError> 
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
                message: format!("the provided user '{}' was not found in the database. please check the username and try again.", &payload.username),
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

    Ok(payload.username)
}