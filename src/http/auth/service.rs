use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::AppState;

use super::auth::{Login, Register};

pub async fn login(form: web::Form<Login>, data: web::Data<AppState>) -> impl Responder {
    let result = data.auth_repo.login(form.into_inner(), data.clone()).await;

    match result {
        Ok(callback) => callback,
        Err(callback) => callback,
    }
}

pub async fn register(form: web::Form<Register>, data: web::Data<AppState>) -> impl Responder {
    let result = data
        .auth_repo
        .register(form.into_inner(), data.clone())
        .await;

    match result {
        Ok(callback) => callback,
        Err(callback) => callback,
    }
}

pub async fn logout(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let token = req.headers().get("Authorization");
    match token {
        Some(token) => {
            let token = token.to_str().unwrap().to_string();
            let result = data.auth_repo.logout(&token, data.clone()).await;
            match result {
                Ok(callback) => callback,
                Err(callback) => callback,
            }
        }
        None => {
            let callback = HttpResponse::Unauthorized().json("Unauthorized");
            return callback;
        }
    }
}
