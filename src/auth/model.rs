use serde::Deserialize;
use validator::Validate;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[0-9a-zA-Z]{2,}$").unwrap(); 
}

#[derive(Deserialize)]
pub struct In<T> {
    pub credentials: T
}

#[derive(Deserialize, Validate)]
pub struct CredentialsSignin {
    #[validate(
        length(
            min = 2,
            max = 25,
            message = "username length must be between 2 and 25 characters.",
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

#[derive(Deserialize, Validate)]
pub struct CredentialsSignup {
    #[validate(
        length(
            min = 2,
            max = 25,
            message = "username length must be between 2 and 25 characters."
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
    pub password: String,
    #[validate(must_match(
        other = "password",
        message = "password do not match."
    ))]
    pub confirm_password: String
}