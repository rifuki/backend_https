use actix_web::http::StatusCode;
use validator::Validate;
use sqlx::Row;

use crate::{
    AppState, 
    user::{
        model::UserPayload,
        helpers::{
            get_stored_user, 
            hashing_password
        }
    },
    errors::{
        AppError, 
        AppErrorMessage
    }, 
};

pub async fn update_user_service(
    app_state: &AppState,
    user_id_params: u32,
    payload: UserPayload
) -> Result<String, AppError> 
{
    /* * validating user input */
    payload.validate()?;
    /* * end validating user input */

    /* * take db pool from handler */
    let db_pool = &app_state.db_pool;
    /* * end take db pool from handler */

    /* * checking and get stored user data */
    let sql_query = format!("SELECT * FROM user where id_user = {}", user_id_params);
    let message = Some(
        format!("the user with id '{}' was not found in the database.", user_id_params)
    );
    let stored_user = get_stored_user(db_pool, &sql_query, message, None).await?;
    let stored_username = stored_user.get::<String,_>("username");
    let stored_email= stored_user.get::<Option<&str>,_>("email");
    let stored_phone_number= stored_user.get::<Option<&str>,_>("phone_number");
    /* * end checking and get stored user data */

    /* * checking availability username */
    let raw_sql_query = format!("SELECT username FROM user WHERE username = '{}'", &payload.username);
    let check_username = get_stored_user(db_pool, &raw_sql_query, None, None).await;
    if let Ok(query_result) = check_username {
        if query_result.get::<&str, _>("username") != stored_username {
            let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                code: StatusCode::CONFLICT.as_u16(),
                message: format!(
                    "the username '{}' is already taken. please choose a different username.", 
                    &payload.username
                ),
                details: None
            };
            return Err(AppError::Conflict(app_err_message.into()));
        }
    }
    /* * checking availability username */
    /* * checking availability email */
    if let Some(payload_email) = &payload.email {
        let raw_sql_query = format!("SELECT email FROM user WHERE email = '{}'", payload_email);
        let check_email = get_stored_user(db_pool, &raw_sql_query, None, None).await;
        if let Ok(query_result) = check_email {
            if query_result.get::<&str, _>("email") != stored_email.as_deref().unwrap_or_default(){
                let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                    code: StatusCode::CONFLICT.as_u16(),
                    message: format!(
                        "the email '{}' is already associated with an existing account. please use a different email", 
                        &payload.email.clone().unwrap()
                    ),
                    details: None
                };
                return Err(AppError::Conflict(app_err_message.into()));
            }
        }
    }
    /* * checking availability email */
    /* * checking availability phone_number */
    if let Some(payload_phone_number) = &payload.phone_number {
        let raw_sql_query = format!("SELECT phone_number FROM user WHERE phone_number = '{}'", payload_phone_number);
        let check_phone_number = get_stored_user(db_pool, &raw_sql_query, None, None).await;
        if let Ok(query_result) = check_phone_number {
            if query_result.get::<&str, _>("phone_number") != stored_phone_number.as_deref().unwrap_or_default() {
                let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                    code: StatusCode::CONFLICT.as_u16(),
                    message: format!(
                        "the phone number '{}' is already registered. please use a different phone number.", 
                        &payload.phone_number.clone().unwrap()
                    ),
                    details: None
                };
                return Err(AppError::Conflict(app_err_message.into()));
            }
        }
    }
    /* * checking availability phone number */

    /* * hashing password */
    let hashed_password = hashing_password(&payload.password)?;
    /* * end hashing password */

    /* * update stored user data  */
    let sql_query = sqlx::query("UPDATE user SET 
        username = ?, 
        password = ?,
        email = ?,
        phone_number = ?,
        full_name = ?
        WHERE id_user = ?
   ");
   let _ = sql_query
        .bind(&payload.username)
        .bind(&hashed_password)
        .bind(&payload.email)
        .bind(&payload.phone_number)
        .bind(&payload.full_name)
        .bind(user_id_params)
        .execute(db_pool)
        .await?;
    /* * end update stored user data  */
    /* * update stored credentials data */
    let sql_query = sqlx::query("UPDATE credentials SET
        username = ?,
        password = ?
        WHERE username = ?
    ");
    let _ = sql_query
        .bind(&payload.username)
        .bind(&hashed_password)
        .bind(&payload.username)
        .execute(db_pool)
        .await?;
    /* * end update stored credentials data */
    
    /* * checking is user update stored  */
    let raw_sql_query = format!("SELECT password FROM user WHERE id_user = {}", user_id_params);
    let message = Some(
        format!(
            "an unknown error occurred while processing your request. the user with id '{}' has not been found in the database. please try again.",
            user_id_params
        )
    );
    let is_user_update_stored = get_stored_user(db_pool, &raw_sql_query, message, None).await?;
    let stored_user_update_password = is_user_update_stored.get::<&str,_>("password");
    /* * end checking is user update stored  */
    /* * checking is credentials update stored  */
    let raw_sql_query = format!("SELECT password FROM credentials WHERE username = '{}'", &payload.username);
    let message = Some(
        format!(
            "an unknown error occurred while processing your request. the user '{}' has not been found in the database. please try again.",
            &payload.username
        )
    );
    let is_credentials_update_stored = get_stored_user(db_pool, &raw_sql_query, message, None).await?;
    let stored_credentials_update_password = is_credentials_update_stored.get::<&str,_>("password");
    /* * end checking is credentials update stored  */

    /* * final validate authentication user credentials  */
    if stored_user_update_password != stored_credentials_update_password {
        let app_err_message = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: String::from("oops! update user failed. unable to update user."),
            details: Some("the provided password does not match the stored credentials. please contact support for assistance.")
        };
        return Err(AppError::InternalServerError(app_err_message.into()));
    }
    /* * end final validate authentication user credentials  */

    /* * */
    Ok( stored_username )
}