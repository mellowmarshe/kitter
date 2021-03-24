use actix_session::Session;
use actix_web::{get, HttpResponse, Responder};

use crate::models::{general, github::User};
use crate::AppState;
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
async fn index(data: web::Data<AppState>, session: Session) -> impl Responder {
    let res = &data.db.get_all_posts().await;

    if let Err(e) = res {
        println!("{}", e);
        return HttpResponse::InternalServerError().json(general::Error {
            status_code: "500".to_string(),
            error: "Error retrieving posts.".to_string(),
        });
    }

    let mut context = Context::new();

    let user = session.get::<User>("user");

    if let Ok(_u) = user {
        if let Some(u) = _u {
            context.insert("user", &u.clone());

            let _ = &data.db.update_changed_usernames(&u).await;
        }
    }

    context.insert("posts", &res.as_ref().unwrap());

    match TEMPLATES.render("index.html", &context) {
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
