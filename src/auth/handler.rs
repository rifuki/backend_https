use actix_web::{
    web, 
    Responder, 
    HttpResponse,
    http::StatusCode
};
use serde_json::json;

use crate::{
    AppState,
    auth::{
        services::{
            signin::signin_service,
            signup::signup_service
        },
        model::{
            In,
            CredentialsSignin,
            CredentialsSignup
        }
    }
};

pub async fn signin(
    app_state_data: web::Data<AppState>, 
    payload: web::Json<In<CredentialsSignin>>
) -> impl Responder 
{
    let app_state = app_state_data.get_ref();
    let payload = payload.into_inner().credentials;

    let signin_service = signin_service(app_state, payload).await;
    match signin_service {
        Ok(result) => {
            let status_code = StatusCode::OK;
            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": format!("user '{}' has successfully logged in.", result)
            });
            HttpResponse::build(status_code)
                .json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}

pub async fn signup(
    app_state_data: web::Data<AppState>,
    payload: web::Json<In<CredentialsSignup>>
) -> impl Responder 
{
    let app_state = app_state_data.get_ref();
    let payload = payload.into_inner().credentials;

    let signup_service = signup_service(app_state, payload).await;
    match signup_service {
        Ok(result) => { 
            let status_code = StatusCode::CREATED;
            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": format!("registration user '{}' successfully! your account has been created.", result)
            });
            HttpResponse::build(status_code)
                .json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}