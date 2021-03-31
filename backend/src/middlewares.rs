use crate::models::user::User;
use crate::utils::jwt;
use actix_web::{dev, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use futures_util::future::err;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Authorization {
    pub user: User,
}

impl FromRequest for Authorization {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
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
        err(ErrorUnauthorized(
            r#"{"status_code": "401", "error": "Unauthorized. Please set the Authorization header with Bearer token."}"#,
        ))
    }
}
