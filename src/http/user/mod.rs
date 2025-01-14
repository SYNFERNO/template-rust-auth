use actix_web::web::{self};

pub(crate) mod repository;
mod service;
mod user;


pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .route("", web::post().to(service::create_user))
            .route("", web::get().to(service::get_users))
            .route("", web::put().to(service::update_user)),
    );
}
