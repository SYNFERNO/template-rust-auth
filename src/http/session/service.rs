use actix_web::{web, Responder};

use crate::AppState;

use super::session::NewSession;

pub async fn create_session(
    form: web::Form<NewSession>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = data
        .session_repo
        .create_sessions(form.into_inner(), data.clone())
        .await;

    match result {
        Ok(callback) => callback,
        Err(callback) => callback,
    }
}

pub async fn get_sessions(data: web::Data<AppState>) -> impl Responder {
    let result = data.session_repo.get_sessions(data.clone()).await;

    match result {
        Ok(callback) => callback,
        Err(callback) => callback,
    }
}
