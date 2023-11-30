use actix_web::http::StatusCode;
use sqlx::Row;
use validator::Validate;

use crate::{
    AppState, 
    user::{
        model::UserPayload,
        helpers::{
            check_user_insert_availability,
            hashing_password, get_stored_user, 
        }
    },
    errors::{
        AppError, 
        AppErrorMessage
    }, 
};

pub async fn insert_user_service(
    app_state: &AppState,
    payload: UserPayload 
) -> Result<String, AppError> 
{
    /* * validating user input */
    payload.validate()?;
    /* * end validating user input */

    /* * take db pool from handler */
    let db_pool = &app_state.db_pool;
    /* * end take db pool from handler */

    /* * checking username & email address & phone_number availability */
    let ( username_exists, email_exists, phone_number_exists ) = check_user_insert_availability (
        db_pool, 
        &payload.username, 
        &payload.email,
        &payload.phone_number
    ).await?;
    if username_exists {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::CONFLICT.as_u16(),
            message: format!(
                "the username '{}' is already taken. please choose a different username.", 
                &payload.username
            ),
            details: None
        };
        return Err(AppError::Conflict(app_err_message.into()));
    } else if email_exists {
        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::CONFLICT.as_u16(),
            message: format!(
                "the email '{}' is already associated with an existing account. please use a different email", 
                &payload.email.clone().unwrap()
            ),
            details: None
        };
        return Err(AppError::Conflict(app_err_message.into()));
    } else if phone_number_exists {
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
    /* * end checking username & email address & phone_number availability */

    /* * hashing user password */
    let hashed_password = hashing_password(&payload.password)?;
    /* * end hashing user password */

    /* * storing user to user table */
    let sql_query = sqlx::query("INSERT INTO user (
        username, 
        password, 
        full_name, 
        email, 
        phone_number
    ) VALUES (?, ?, ?, ?, ?);");
    let _ = sql_query
        .bind(&payload.username)
        .bind(&hashed_password)
        .bind(&payload.full_name)
        .bind(&payload.email)
        .bind(&payload.phone_number)
        .execute(db_pool)
        .await?;
    /* * end storing user to user table */
    /* * storing user to credential table */
    let sql_query = sqlx::query("INSERT INTO credentials (
        username, 
        password
    ) VALUES (?, ?);");
    let _ = sql_query
        .bind(&payload.username)
        .bind(&hashed_password)
        .execute(db_pool)
        .await?;
    /* * end storing user to credential table */

    /* * checking user stored in user table */
    let raw_sql_query = format!("SELECT password FROM user WHERE username = '{}'", &payload.username);
    let message = Some(
        format!(
            "an unknown error occurred while processing your request. the user '{}' has not been found in the database. please try again.",
            &payload.username
        )
    );
    let is_user_stored = get_stored_user(db_pool, &raw_sql_query, message, None).await?;
    let stored_user_password = is_user_stored.get::<&str,_>("password");
    /* * end checking user stored user table */

    /* * checking credentials stored in credentials table */
    let raw_sql_query = format!("SELECT password FROM credentials WHERE username = '{}'", &payload.username);
    let message = Some(
        format!(
            "an unknown error occurred while processing your request. the user '{}' has not been found in the database. please try again.",
            &payload.username
        )
    );
    let is_credentials_stored = get_stored_user(db_pool, &raw_sql_query, message, None).await?;
    let stored_credentials_password = is_credentials_stored.get::<&str,_>("password");
    /* * end checking credentials stored in credentials table */
    
    /* * final validate authentication user credentials */
    if stored_user_password != stored_credentials_password {
        let app_err_message = AppErrorMessage {
            code: StatusCode::CONFLICT.as_u16(),
            message: String::from("oops! registration failed. unable to create user."),
            details: Some("the password provided does not match the stored credentials. please contact support for assistance.") 
        };
        return Err(AppError::Conflict(app_err_message.into()));
    }
    /* * end final validate authentication user credentials */

    Ok( payload.username )
}