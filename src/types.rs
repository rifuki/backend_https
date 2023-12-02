use crate::db::DbPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: DbPool,
    pub secret_access_token: String,
    pub secret_refresh_token: String
}

