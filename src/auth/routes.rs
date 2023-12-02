use actix_web::web;

use crate::{
    auth::handler::{
        signin,
        refresh_token,
        logout
    },
    user::insert_user
};

pub fn scoped_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/logout", web::get().to(logout))
            .route("/signup", web::post().to(insert_user))
            .route("/signin", web::post().to(signin))
            .route("/refresh", web::get().to(refresh_token))
    );
}