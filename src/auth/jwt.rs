use actix_web::{
    FromRequest,
    HttpRequest,
    dev::Payload,
    http::{
        header::AUTHORIZATION, 
        StatusCode
    }, 
    web
};
use chrono::Utc;
use std::future::{
    Ready,
    ready
};
use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation,
    Algorithm,
    errors::ErrorKind as JwtErrorKind
};

use crate::{
    errors::{
        AppError, 
        AppErrorMessage
    }, 
    auth::types::Claims,
    AppState
};

#[derive(Debug)]
pub struct JwtAuth;
impl FromRequest for JwtAuth {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        /* * checking Authorization header and get value */
        let auth_header = req.headers().get(AUTHORIZATION);
        let auth_value = auth_header.map_or("", |hv| hv.to_str().unwrap_or(""));
        let token  = auth_value.split_whitespace().nth(1);
        if token.is_none() {
            let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                code: StatusCode::UNAUTHORIZED.as_u16(),
                message: String::from("Unauthorized: missing or invalid authorization token."),
                details: None
            };
            return ready(Err(AppError::Unauthorized(app_err_message.into())));
        }
        /* * end checking Authorization header and get value */

        /* * get secret_access_token from app_state */
        let secret_access_token = req.app_data::<web::Data<AppState>>()
            .map(|app_state| &app_state.get_ref().secret_access_token)
            /* * * convert option type to result type */
            .ok_or_else(|| {
                let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: String::from("failed to get app_state."),
                    details: None
                };
                AppError::InternalServerError(app_err_message.into())
            });
            /* * * end convert option type to result type */
        /* * get secret_access_token from app_state */
        
        /* * streaming result secret_access_token for return Ok and Err */
        let future_result = secret_access_token.and_then(|secret_access_token| {
            /* * * checking user token is valid */
            let is_token_valid = decode::<Claims>(
                &token.unwrap(),
                &DecodingKey::from_secret(secret_access_token.as_ref()),
                &Validation::new(Algorithm::HS256)
            ).map_err(|e| {
                match e.kind() {
                    JwtErrorKind::ExpiredSignature => {
                        let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                            code: StatusCode::UNAUTHORIZED.as_u16(),
                            message: String::from("Access token signature has expired. Please regenerate a new signature for the access token."),
                            details: None
                        };
                        AppError::Unauthorized(app_err_message.into())
                    },
                    _ => {
                        let app_err_message = AppErrorMessage {
                            code: StatusCode::UNAUTHORIZED.as_u16(),
                            message: format!("Access token validation failed."),
                            details: Some(e.to_string()) 
                        };
                        AppError::Unauthorized(app_err_message.into())
                    }
               } 
            })?;
            /* * * end checking user token is valid */

            /* * * checking token expired  */
            let token_exp = is_token_valid.claims.exp;
            let time_now = Utc::now().naive_utc().timestamp();
            if time_now >= token_exp {
                let app_err_message: AppErrorMessage<Option<bool>> = AppErrorMessage {
                    code: StatusCode::UNAUTHORIZED.as_u16(),
                    message: String::from("Sorry, the access token expired token. Please obtain a new access token using the refresh token."),
                    details: None
                };
                return Err(AppError::Unauthorized(app_err_message.into()));
            }
            /* * * end checking token expired  */

            Ok( Self )
        });
        /* * end streaming result secret_access_token for return Ok and Err */

        /* * return future */
        ready(future_result)
        /* * end return future */
    }
}