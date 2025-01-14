use actix_web::{web, HttpResponse, Responder};

use crate::AppState;

use super::token::{NewToken, RequestToken, UpdateToken};

pub async fn create_token(form: web::Form<NewToken>, data: web::Data<AppState>) -> impl Responder {
    let result = data
        .token_repo
        .create_token(form.into_inner(), data.clone())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => e,
    }
}

pub async fn get_token_by_id(
    path: web::Query<RequestToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = data.token_repo.get_token_by_id(path.id, data.clone()).await;

    match result {
        Ok(callback) => callback,
        Err(e) => e,
    }
}

pub async fn update_token(
    form: web::Form<UpdateToken>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = data
        .token_repo
        .update_token(form.into_inner(), data.clone())
        .await;

    match result {
        Ok(callback) => callback,
        Err(e) => e,
    }
}
