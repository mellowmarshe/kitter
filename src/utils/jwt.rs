use crate::constants;
use crate::models::{auth, user};
use auth::Claims;
use chrono::Utc;
use jsonwebtoken::{decode as dc, encode as ec, DecodingKey, EncodingKey, Header, Validation};

pub fn encode(user: &user::User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000;

    let claims = Claims {
        company: "Kitter".to_string(),
        exp: now + (60 * 60 * 24 * 7),
        iss: user.id.to_string(),
        user: user.clone(),
    };

    let token = ec(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&constants::CONFIG.server.secret.as_bytes()),
    );

    if let Err(e) = token {
        return Err(e);
    }

    Ok(token.unwrap())
}

pub fn decode(token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation {
        ..Validation::default()
    };
    let token_data = dc::<Claims>(
        &token,
        &DecodingKey::from_secret(&constants::CONFIG.server.secret.as_bytes()),
        &validation,
    );

    if let Err(e) = token_data {
        return Err(e);
    }

    Ok(token_data.unwrap().claims)
}
