/* * check user with custom query and return bool */
pub async fn check_user<'a, T>(
    db_pool: &DbPool,
    query: &'a str,
    value: &'a T,
) -> bool
where
    T: sqlx::Type<sqlx::MySql> + sqlx::Encode<'a, sqlx::MySql> + Send + Sync,
{
    let query_result = sqlx::query(query)
        .bind(value)
        .fetch_one(db_pool)
        .await
        .is_ok();

    query_result
}
/* * end check user with custom query and return bool */