use actix_web::web::{self};

pub(crate) mod repository;
mod service;
pub(crate)  mod token;

pub fn config_token(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/tokens")
            .route("", web::post().to(service::create_token))
            .route("", web::get().to(service::get_token_by_id))
            .route("", web::put().to(service::update_token)),
    );
}
