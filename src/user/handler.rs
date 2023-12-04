use actix_web::{
    web, 
    Responder, 
    HttpResponse,
    http::StatusCode
};
use serde_json::json;

use crate::{
    types::AppState,
    user::{
        services::{
            get_all_users::get_all_users_service,
            get_user::get_user_service,
            insert_user::insert_user_service,
            update_user::update_user_service,
            delete_user::delete_user_service
        },
        model::{
           In,
            UserPayload 
        }
    },
    auth::JwtAuth
};

pub async fn get_all_users(
    _: JwtAuth,
    app_state: web::Data<AppState>
) -> impl Responder 
{
    let app_state = app_state.get_ref();
    
    let get_all_users_service = get_all_users_service(app_state).await;
    match get_all_users_service {
        Ok(users) => {
            let status_code = StatusCode::OK;
            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": "successfully retrieved all users data.",
                "data": users,
                "length": users.len()
            });
            HttpResponse::build(status_code).json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}

pub async fn get_user(
    _: JwtAuth,
    app_state: web::Data<AppState>,
    path: web::Path<u32>
) -> impl Responder 
{
    let app_state = app_state.get_ref();
    let user_id_params = path.into_inner();
    
    let get_user_service = get_user_service(app_state, user_id_params).await;
    match get_user_service {
        Ok(user) => {
            let status_code = StatusCode::OK;
            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": "user data retrieved successfully.",
                "data": user
            });
            HttpResponse::build(status_code).json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}

pub async fn insert_user(
    app_state: web::Data<AppState>,
    payload: web::Json<In<UserPayload>>
) -> impl Responder {
    let app_state = app_state.get_ref();
    let payload = payload.into_inner().user;
    
    let insert_user_service = insert_user_service(app_state, payload).await;

    match insert_user_service {
        Ok(user) => { 
            let status_code = StatusCode::CREATED;
            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": format!("registration user '{}' successfully! your account has been created.", user)
            });
            HttpResponse::build(status_code).json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}

pub async fn update_user(
    _: JwtAuth,
    app_state: web::Data<AppState>,
    path: web::Path<u32>,
    payload: web::Json<In<UserPayload>>
) -> impl Responder {
    let app_state = app_state.get_ref();
    let user_id_params = path.into_inner();
    let payload = payload.into_inner().user;
    
    let update_user_service = update_user_service(app_state, user_id_params, payload).await;
    match update_user_service {
        Ok(user) => {
            let status_code = StatusCode::OK;
            let success_message = json!({
                "status": true,
                "code": status_code.as_u16(),
                "message": format!("user '{}' has been successfully updated.", user)
            });
            HttpResponse::build(status_code).json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}

pub async fn delete_user(
    _: JwtAuth,
    app_state: web::Data<AppState>,
    path: web::Path<u32>
) -> impl Responder {
    let app_state = app_state.get_ref();
    let user_id_params = path.into_inner();
    
    let delete_user_service = delete_user_service(app_state, user_id_params).await;
    match delete_user_service {
        Ok(result) => {
            let status_code = StatusCode::OK;
            let success_message = json!({
                "success": true,
                "code": status_code.as_u16(),
                "message": format!("user '{}' has been successfully deleted.", result)
            });
            HttpResponse::build(status_code).json(success_message)
        },
        Err(e) => HttpResponse::from_error(e)
    }
}