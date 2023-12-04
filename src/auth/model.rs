use serde::Deserialize;
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::FromRow;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[0-9a-zA-Z]{2,}$").unwrap(); 
}

#[derive(FromRow)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub max_age: Option<i64>
}

#[derive(Deserialize)]
pub struct In<T> {
    pub credentials: T
}

#[derive(Deserialize, Validate)]
pub struct CredentialsPayload{
    #[validate(
        length(
            min = 2,
            max = 25,
            message = "username length must be between 2 to 25 characters.",
        ),
        regex(
            path = "RE_USERNAME",
            message = "username must consist of alphanumeric characters and be at least 2 characters long."
        )
    )]
    pub username: String,
    #[validate(
        length(
            min = 8,
            message = "password length must be at least 8 characters."
        )
    )]
    pub password: String
}