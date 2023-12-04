use actix_web::{web, Responder, HttpResponse, http::StatusCode};
use serde_json::json;

pub fn scoped_ping(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ping")
            .to(ping)
    );
}

async fn ping() -> impl Responder {
    let status_code = StatusCode::OK;

    let success_message = json!({
        "success": true,
        "code": status_code.as_u16(),
        "message": "PONG" 
    });

    HttpResponse::build(status_code)
        .json(success_message)
}
