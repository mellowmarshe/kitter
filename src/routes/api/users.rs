use crate::models::{auth, general, user};
use crate::utils;
use crate::AppState;
use actix_web::{post, web, HttpResponse, Responder};
use utils::jwt;

#[post("/user/register")]
async fn register(
    data: web::Data<AppState>,
    mut user: web::Json<user::Register>,
) -> impl Responder {
    if !user::verify(user.clone()) {
        return HttpResponse::BadRequest().json(general::Error {
            status_code: "400".to_string(),
            error: "Invalid request. Make sure the fields meet our requirements.".to_string(),
        });
    }

    let hashed = bcrypt::hash(user.password.clone(), 10);

    if let Err(_) = hashed {
        return HttpResponse::InternalServerError().json(general::Error {
            status_code: "500".to_string(),
            error: "Error with hashing, please try again.".to_string(),
        });
    };
    let plain = user.password.clone();
    user.password = hashed.unwrap();

    let res = &data.db.add_user(&user).await;

    match res {
        Err(_) => {
            return HttpResponse::InternalServerError().json(general::Error {
                status_code: "500".to_string(),
                error: "Error adding user.".to_string(),
            });
        }
        Ok(e) => {
            let mut u = e.clone();
            u.password = plain;
            HttpResponse::Ok().json(u)
        }
    }
}

#[post("/user/login")]
async fn login(data: web::Data<AppState>, user: web::Json<user::UserPassword>) -> impl Responder {
    let res = &data.db.get_user_by_name(&user).await;

    match res {
        Err(e) => {
            println!("{}", e);
            return HttpResponse::InternalServerError().json(general::Error {
                status_code: "500".to_string(),
                error: "Error getting user.".to_string(),
            });
        }

        Ok(e) => {
            let mut u = e.clone();
            u.password = user.password.clone();
            match bcrypt::verify(user.password.clone(), &e.password) {
                Ok(res) => {
                    if res == true {
                        match jwt::encode(&u) {
                            Ok(t) => {
                                return HttpResponse::Ok().json(auth::AuthToken {
                                    token: t,
                                    token_type: "bearer".to_string(),
                                })
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                return HttpResponse::InternalServerError().json(general::Error {
                                    status_code: "500".to_string(),
                                    error: "Error with JWT, please try again.".to_string(),
                                });
                            }
                        };
                    }
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().json(general::Error {
                        status_code: "500".to_string(),
                        error: "Error with hashing, please try again.".to_string(),
                    });
                }
            }
            HttpResponse::Ok().json(u)
        }
    }
}
