use crate::models::{general, github::User, post};
use crate::utils;
use crate::AppState;
use actix_session::Session;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/post/add")]
async fn add(
    data: web::Data<AppState>,
    session: Session,
    mut post: web::Json<post::NewPost>,
) -> impl Responder {
    if !post::verify(post.content.clone()) {
        return HttpResponse::BadRequest().json(general::Error {
            status_code: "400".to_string(),
            error: "Invalid request, make sure you dont have contents longer than 512 characters."
                .to_string(),
        });
    }

    let user = session.get::<User>("user").unwrap().expect("");

    post.content = Some(utils::escape_html(post.content.clone().unwrap()));
    post.author = Some(user.login);
    post.author_id = Some(user.id);
    post.hearts = Some(0);
    post.hearted_users = Some(Vec::<i64>::new());

    let res = &data.db.add_post(&post).await;

    match res {
        Err(_) => {
            return HttpResponse::InternalServerError().json(general::Error {
                status_code: "500".to_string(),
                error: "Error when adding to database.".to_string(),
            });
        }
        Ok(e) => HttpResponse::Ok().json(e),
    }
}

#[post("/post/posts")]
async fn posts(
    data: web::Data<AppState>,
    paged: Option<web::Json<post::PagedPosts>>,
) -> impl Responder {
    let res;

    if let Some(p) = paged {
        res = data.db.get_paged_posts(&p).await;
    } else {
        res = data.db.get_all_posts().await;
    }

    match res {
        Err(_) => {
            return HttpResponse::InternalServerError().json(general::Error {
                status_code: "500".to_string(),
                error: "Error contacting database.".to_string(),
            })
        }
        Ok(e) => HttpResponse::Ok().json(e),
    }
}

#[delete("/post/delete")]
async fn delete(
    data: web::Data<AppState>,
    session: Session,
    post: web::Json<post::IdOnlyPost>,
) -> impl Responder {
    let user = session.get::<User>("user").unwrap().expect("");
    let res = &data.db.delete_post(&post, &user).await;

    match res {
        Err(_) => {
            return HttpResponse::InternalServerError().json(general::Error {
                status_code: "500".to_string(),
                error: "Error when removing to database, maybe that Id doesnt exist..".to_string(),
            })
        }
        Ok(e) => HttpResponse::Ok().json(e),
    }
}

#[post("/post/heart")]
async fn heart(
    data: web::Data<AppState>,
    session: Session,
    post: web::Json<post::IdOnlyPost>,
) -> impl Responder {
    let user = session.get::<User>("user").unwrap().expect("");
    let res = &data.db.toggle_heart_post(&post, &user).await;

    match res {
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().json(general::Error {
                status_code: "500".to_string(),
                error: "Error when updating hearts...".to_string(),
            });
        }
        Ok(e) => HttpResponse::Ok().json(e),
    }
}
