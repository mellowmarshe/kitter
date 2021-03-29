use actix_web::{get, HttpResponse, Responder};

use crate::AppState;
use crate::{middlewares::Authorization, models::general};
use actix_web::web;

use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Error parsing templates {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html"]);
        return tera;
    };
}

#[get("/")]
async fn index(authorization: Option<Authorization>, data: web::Data<AppState>) -> impl Responder {
    let mut context = Context::new();

    let mut template = "notloggedin.html";

    if let Some(u) = authorization {
        let user = u.user;
        context.insert("user", &user.clone());

        let _ = &data.db.update_changed_usernames(&user).await;

        template = "index.html";
    }

    match TEMPLATES.render(template, &context) {
        Ok(t) => HttpResponse::Ok()
            .header("Content-Type", "text/html; charset=utf-8")
            .body(t),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().json(general::Error {
                status_code: "500".to_string(),
                error: "Unknown error.".to_string(),
            })
        }
    }
}
