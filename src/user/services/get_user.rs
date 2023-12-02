use actix_web::http::StatusCode;

use crate::{
    types::AppState, 
    errors::{
        AppError, 
        AppErrorMessage
    }, 
    user::model::User
};

pub async fn get_user_service(
    app_state: &AppState,
    user_id_params: u32
) -> Result<User, AppError> 
{
    /* * take db pool from handler */
    let db_pool = &app_state.db_pool;
    /* * end take db pool from handler */

    /* * sql query get one user */
    let sql_query = sqlx::query_as::<_,User>("SELECT * FROM user WHERE id_user = ?");
    let query_result = sql_query
        .bind(user_id_params)
        .fetch_one(db_pool)
        .await
        .map_err(|e| {
            let app_error_message = AppErrorMessage {
                code: StatusCode::NOT_FOUND.as_u16(),
                message: format!("user with id '{}' not found", user_id_params),
                details: Some(e.to_string())
            };
            AppError::NotFound(app_error_message.into())
        })?;
    /* * end sql query get one user */

    Ok(query_result)
}