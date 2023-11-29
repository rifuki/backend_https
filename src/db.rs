use sqlx::mysql::{
    MySqlPool,
    MySqlPoolOptions
};
use std::time::Duration;

pub type DbPool = MySqlPool;

pub async fn establish_connection(db_url: &str) -> DbPool {
    MySqlPoolOptions::new()
        .acquire_timeout(Duration::from_secs(1))
        .idle_timeout(Duration::from_secs(1))
        .max_connections(10)
        .connect(db_url)
        .await
        .unwrap_or_else(|e| {
            let error_message = "FAILED TO CREATE DATABASE POOL.";
            eprintln!("{} [{}]", error_message, e);
            std::process::exit(1);
        })

}