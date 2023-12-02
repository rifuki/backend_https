use actix_web::{
    web,
    Responder, 
    HttpResponse,
    http::StatusCode,
    cookie::{
        Cookie,
        time::OffsetDateTime
    },
    HttpRequest
};
use serde_json::json;

use crate::{
    types::AppState,
    auth::{
        services::{
            signin::signin_service,
            refresh::refresh_token_service,
            logout::logout_service
        },
        model::{
            In,
            CredentialsPayload
        },
        types::{
            ResponseSignin,
            ResponseRefreshToken 
        },
        constants::ACTIX_REFRESH_TOKEN_EXPIRED
    }
};

pub async fn signin(
    app_state: web::Data<AppState>, 
    payload: web::Json<In<CredentialsPayload>>
) -> impl Responder 
{
    let app_state = app_state.get_ref();
    let payload = payload.into_inner().credentials;

    let signin_service = signin_service(app_state, payload).await;
    match signin_service {
        Ok(user) => {

            let status_code = StatusCode::OK;

            let create_cookie = Cookie::build("refresh_token", user.encoded_refresh_token)
                .http_only(true)
                .max_age(ACTIX_REFRESH_TOKEN_EXPIRED)
                .path("/")
                .finish();

            let response_data = ResponseSignin {
                access_token: user.encoded_access_token
            };
            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": format!("user '{}' has successfully logged in.", user.username),
                "data": response_data
            });

            HttpResponse::build(status_code)
                .cookie(create_cookie)
                .json(success_message)
        },

        Err(e) => HttpResponse::from_error(e)
    }
}

pub async fn refresh_token(
    request: HttpRequest,
    app_state: web::Data<AppState>
) -> impl Responder {
    let refresh_token_cookie = request.cookie("refresh_token");
    let app_state = &app_state.get_ref();

    let refresh_token_service = refresh_token_service(app_state, refresh_token_cookie).await;
    match refresh_token_service {
        Ok(token) => {
            let status_code = StatusCode::OK;

            let update_cookie = Cookie::build("refresh_token", token.refresh_token.clone())
                .http_only(true)
                .max_age(token.refresh_token_previous_max_age)
                .path("/")
                .finish();

            let response_data = ResponseRefreshToken {
                access_token: token.access_token,
                refresh_token: token.refresh_token
            };

            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "data": response_data 
            });
            HttpResponse::build(status_code)
                .cookie(update_cookie)
                .json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}

pub async fn logout(
    app_state: web::Data<AppState>,
    request: HttpRequest,
) -> impl Responder {
    let app_state = app_state.get_ref();
    let refresh_token_cookie = request.cookie("refresh_token");

    let logout_service = logout_service(app_state, refresh_token_cookie).await;
    match logout_service {
        Ok(user) => {
            let status_code = StatusCode::OK;

            let delete_cookie = Cookie::build("refresh_token", "")
                .path("/")
                .expires(OffsetDateTime::now_utc())
                .finish();

            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": format!("user '{}' successfully logout.", user)
            });
            HttpResponse::build(status_code)
                .cookie(delete_cookie)
                .json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}