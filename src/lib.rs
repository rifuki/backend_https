pub mod auth;
mod db;
mod errors;

pub use db::establish_connection;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: db::DbPool
}