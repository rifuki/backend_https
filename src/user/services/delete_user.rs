use actix_web::http::StatusCode;
use sqlx::Row;

use crate::{
    types::AppState, 
    errors::{
        AppError, 
        AppErrorMessage
    }, 
    user::helpers::get_stored_user
};

pub async fn delete_user_service(
    app_state: &AppState,
    user_id_params: u32
) -> Result<String, AppError> 
{
    /* * take db pool from handler */
    let db_pool = &app_state.db_pool;
    /* * end take db pool from handler */

    /* * check & get user from database */
    let raw_sql_query = format!("SELECT * FROM user WHERE id_user = {}", user_id_params);
    let message = Some(
        format!("user with id '{}' not found.", user_id_params)
    );
    let get_existing_user = get_stored_user(db_pool, &raw_sql_query, message, None).await?;
    /* * check & get user from database */

    /* * destroy user from database */
    let sql_query = sqlx::query("DELETE FROM user WHERE id_user = ?");
    let _ = sql_query
        .bind(user_id_params)
        .execute(db_pool)
        .await;
    /* * end destroy user from database */
    
    /* * check is user successfully destroyed */
    let raw_sql_query = format!("SELECT username FROM user WHERE id_user = {}", user_id_params);
    let code = Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16());
    let is_user_still_stored = get_stored_user(db_pool, &raw_sql_query, None, code).await;
    if let Ok(res) = is_user_still_stored {
        let app_error_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: format!(
                "an unexpected error occurred while processing your request. user '{}' still exists in the database. please try again.",
                 res.get::<&str, _>("username")
            ),
            details: None 
        };
        return Err(AppError::InternalServerError(app_error_message.into()));
    }
    // 
    /* * end check is user successfully destroyed */

    let user_username = get_existing_user.get::<String, _>("username");
    Ok( user_username )
}