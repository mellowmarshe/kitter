#[macro_use]
extern crate lazy_static;

mod config;
mod constants;
mod database;
mod middlewares;
mod models;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_files::Files;
use actix_redis::RedisSession;
use actix_web::{web, App, HttpServer};
use routes::{api, auth, index};
use std::error::Error;

pub struct AppState {
    pub db: database::ConnectionPool,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let db = database::ConnectionPool::new().await?;

    db.migrations().await?;

    HttpServer::new(move || {
        let _cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .send_wildcard()
            .max_age(3600);

        App::new()
            .data(AppState { db: db.clone() })
            .wrap(RedisSession::new(
                &constants::CONFIG.database.redis,
                &[0; 32],
            ))
            .service(
                (web::scope("/api"))
                    .service(api::posts::add)
                    .service(api::posts::delete)
                    .service(api::posts::posts)
                    .service(api::posts::heart)
                    .service(api::posts::ping)
                    .service(api::users::register)
                    .service(api::users::login),
            )
            .service(
                (web::scope("/auth"))
                    .service(auth::login)
                    .service(auth::logout)
                    .service(auth::callback),
            )
            .service(index::index)
            .service(Files::new("/styles", "static/styles").show_files_listing())
            .service(Files::new("/js", "static/js").show_files_listing())
    })
    .bind(format!(
        "{}:{}",
        &constants::CONFIG.server.host,
        &constants::CONFIG.server.port
    ))?
    .run()
    .await?;

    Ok(())
}
