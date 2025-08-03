//jwt 생성 및 검증 
use super::app_error::AppError;
use axum::{
    http::{HeaderMap,Request,StatusCode},
    middleware::Next,
    response::Response,body::Body,
};
use chrono::Duration;
use jsonwebtoken::{decode,encode,DecodingKey, EncodingKey,Header,Validation};
use serde::{Deserialize,Serialize};
use std::env;
use tracing::{debug,error};
use lazy_static::lazy_static;

lazy_static!{
    static ref SECRET_KEY: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
}

#[derive(Serialize,Deserialize)]
pub struct Claims{
    exp:usize,
    username:String,
}

pub fn create_token(username:String)-> Result<String,AppError>{
    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims=Claims(exp,username);
    let token_header = Header::default();
    let key =EncodingKey::from_secret(SECRET_KEY.as_bytes());
    encode(&token_header,&claims,&key).map_err(|err|{
        error!("Error creating token: {:?}",err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "there was an error, please try again later",
        )
    })
}

pub fn validate_token(token:&str)-> Result<Claims,AppError>{
    let binding - token.replace("Bearer ","");
    let key=DecodingKey::from_secret(SECRET_KEY.as_bytes());
    let validation=Validation::new(jsonwebtoken::Algorithm::HS256);

    decode::<Claims>(&binding,&key,&validate_token)
        .map_err(|err| match err.kind(){
            jsonwebtoken::errors:ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpriedSignature => {
                AppError::new(StatusCode::UNAUTHORIZED,"not authenticated!")
            }
            _ => {
                error!("Error validating token: {:?}",err);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"error validating token")
            }
        })
        .and_then(|decode|{
            if chrono::Utc::now().timestamp() > decoded.claims.exp as i64{
                Err(AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "not authenticated!",
                ))
            } else {
                Ok(decoded.claims)
            }
        })
}

pub async fn authenticate(
    headers:HeaderMap,
    request: Request<Body>,
    next:Next,
) -> Result<Response,AppError> {
    if let Some(value)= headers.get("Authorization"){
        let token = value.to_str().map_err(|err|{
            error!("Error extractinv token from headers: {:?}",err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Error reading token")
        })?;

        let claim=validate_token(token)?;

        debug!("Authenticated user: {}", claim.username);

        if claim.exp < (chrono::Utc::now().timestamp() as usize){
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "Token has expired"));
        }
        Ok(next.run(request).await)
    }else{
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ))
    }
}