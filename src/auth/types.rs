use serde::{
    Serialize,
    Deserialize
};
use actix_web::cookie::time::Duration as ActixDuration;

#[derive(Serialize, Debug, Deserialize)]
pub struct Claims {
    pub username: String,
    pub iat: i64,
    pub exp: i64 
}

pub struct ServiceOkSignin {
    pub username: String,
    pub encoded_access_token: String,
    pub encoded_refresh_token: String
}

#[derive(Serialize)]
pub struct ResponseSignin {
    pub access_token: String
}

pub struct ServiceOkRefreshToken {
    pub access_token: String,
    pub refresh_token: String,
    pub refresh_token_previous_max_age: ActixDuration
}

#[derive(Serialize)]
pub struct ResponseRefreshToken {
    pub access_token: String,
    pub refresh_token: String
}