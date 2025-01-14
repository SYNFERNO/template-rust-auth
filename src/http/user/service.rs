use actix_web::{web, HttpRequest, Responder};

use crate::{middleware::auth_middleware, AppState};

use super::user::{NewUser, UpdateUser};

pub async fn create_user(
    req: HttpRequest,
    form: web::Form<NewUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let token = auth_middleware::auth_middleware(req, data.clone()).await;
    match token {
        Ok(_) => {
            let result = data
                .user_repo
                .create_user(form.into_inner(), data.clone())
                .await;

            match result {
                Ok(callback) => callback,
                Err(callback) => callback,
            }
        }
        Err(e) => e,
    }
}

pub async fn get_users(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let token = auth_middleware::auth_middleware(req, data.clone()).await;
    match token {
        Ok(token) => {
            let result = data.user_repo.get_users(&token, data.clone()).await;

            match result {
                Ok(callback) => callback,
                Err(callback) => callback,
            }
        }
        Err(e) => e,
    }
}

pub async fn update_user(
    req: HttpRequest,
    form: web::Form<UpdateUser>,
    data: web::Data<AppState>,
) -> impl Responder {
    let token = auth_middleware::auth_middleware(req, data.clone()).await;
    match token {
        Ok(token) => {
            let result = data
                .user_repo
                .update_user(&token, form.into_inner(), data.clone())
                .await;

            match result {
                Ok(callback) => callback,
                Err(callback) => callback,
            }
        }
        Err(e) => e,
    }
}
