use crate::models::{general, user::User};
use crate::{constants, utils::jwt};
use actix_service::{Service, Transform};
use actix_web::{
    dev::ServiceRequest,
    dev::{self, ServiceResponse},
    error::ErrorUnauthorized,
    Error, FromRequest, HttpRequest, HttpResponse,
};
use futures::future::{ok, Ready};
use futures::Future;
use futures_util::future::err;
use serde::Deserialize;
use std::pin::Pin;
use std::task::{Context, Poll};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if constants::CONFIG
            .server
            .secured
            .contains(&req.path().to_string())
        {
            if let Some(i) = req.headers().get("Authorization") {
                let i = i.to_str().unwrap();
                if i.to_lowercase().starts_with("bearer") {
                    let token = i[6..i.len()].trim();

                    match jwt::decode(token.to_string()) {
                        Ok(c) => {
                            println!("good: {:?}", c);
                        }
                        Err(_) => {
                            return Box::pin(async move {
                                Ok(req.into_response(
                            HttpResponse::Unauthorized()
                                .json(general::Error {
                                    status_code: "401".to_string(),
                                    error: "Unauthorized. Please set the Authorization header with Bearer token.".to_string(),
                                }).into_body(),))
                            });
                        }
                    }
                }
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Authorization {
    pub user: User,
}

impl FromRequest for Authorization {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        if let Some(i) = req.headers().get("Authorization") {
            let i = i.to_str().unwrap();
            if i.to_lowercase().starts_with("bearer") {
                let token = i[6..i.len()].trim();

                match jwt::decode(token.to_string()) {
                    Ok(c) => return ok(Authorization { user: c.user }),
                    Err(_) => {
                        return err(ErrorUnauthorized(
                            r#"{"status_code": "401", "error": "Unauthorized. Please set the Authorization header with Bearer token."}"#,
                        ))
                    }
                }
            }
        }
        return err(ErrorUnauthorized(
            r#"{"status_code": "401", "error": "Unauthorized. Please set the Authorization header with Bearer token."}"#,
        ));
    }
}
