use actix_web::web::{self};

pub(crate) mod repository;
mod service;
mod auth;

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .route("/login", web::post().to(service::login))
            .route("/register", web::post().to(service::register))
            .route("/logout", web::post().to(service::logout)),
    );
}
