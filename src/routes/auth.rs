use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use reqwest::header;
use std::error;

use crate::models::{auth, github};
use crate::AppState;
use crate::{config, constants};

#[get("/login")]
async fn login(_: web::Data<AppState>, _: Session) -> impl Responder {
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&state={}&scope=user&response_type=code",
        constants::CONFIG.github.client_id, constants::CONFIG.github.state
    );

    HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, url)
        .finish()
}

#[get("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();

    HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, "/")
        .finish()
}

#[get("/callback")]
async fn callback(
    _: web::Data<AppState>,
    callback: web::Query<auth::Callback>,
    session: Session,
) -> impl Responder {
    let res = authenticate(&constants::CONFIG, callback).await;

    match res {
        Err(_) => {
            return HttpResponse::TemporaryRedirect()
                .header(header::LOCATION, "/")
                .finish();
        }
        Ok(e) => {
            /*
            let token = jwt_encode(&e);



            match token {
                Ok(t) => return HttpResponse::Ok().json(auth::AuthToken { token: t }),
                Err(e) => {
                    println!("{:?}", e);
                    return HttpResponse::Ok().json(auth::AuthToken {
                        token: "owo".to_string(),
                    });
                }
            };
            */

            session.set("user", e).expect("Err in session");

            return HttpResponse::TemporaryRedirect()
                .header(header::LOCATION, "/")
                .finish();
        }
    }
}

async fn authenticate(
    config: &config::Config,
    cb: web::Query<auth::Callback>,
) -> Result<github::User, Box<dyn error::Error>> {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent("kitter/1.0.0")
        .use_rustls_tls()
        .build()?;

    let url = format!(
        "https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={}&state={}",
        config.github.client_id,
        config.github.client_secret,
        &cb.code,
        config.github.state,
    );

    let access_token = client
        .post(&url)
        .send()
        .await?
        .json::<auth::AccessToken>()
        .await?;

    let res = client
        .get("https://api.github.com/user")
        .header(
            "Authorization",
            format!("token {}", access_token.access_token),
        )
        .send()
        .await?
        .json::<github::User>()
        .await?;

    Ok(res)
}
