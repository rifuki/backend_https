use crate::{
    types::AppState, 
    errors::AppError, 
    user::model::User, 
    auth::JwtAuth,
};

pub async fn get_all_users_service(
    _: JwtAuth,
    app_state: &AppState
) -> Result<Vec<User>, AppError> 
{
    /* * take db pool from handler */
    let db_pool = &app_state.db_pool;
    /* * end take db pool from handler */

    /* * sql query retrieved all users */
    let sql_query = sqlx::query_as::<_, User>("SELECT * FROM user");
    let query_result = sql_query.fetch_all(db_pool).await?;
    /* * end sql query retrieved all users */

    Ok(query_result)
}