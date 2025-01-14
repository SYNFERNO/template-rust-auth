use actix_web::web::{self};

pub(crate) mod repository;
mod service;
pub(crate)  mod session;

pub fn config_session(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/sessions")
            .route("", web::post().to(service::create_session))
            .route("", web::get().to(service::get_sessions)),
    );
}
