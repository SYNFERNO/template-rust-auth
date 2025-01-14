use actix_web::{middleware::Logger, web, App, HttpServer};
use config::connect_database;
use sqlx::{Pool, Postgres};

mod config;
mod http;
mod utils;
mod middleware;

pub struct AppState {
    pub db: Pool<Postgres>,
    user_repo: http::user::repository::UserRepository,
    token_repo: http::token::repository::TokenRepository,
    session_repo: http::session::repository::SessionRepository,
    auth_repo: http::auth::repository::AuthRepository,
    jwt_repo: http::jwt::repository::JWTRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");

    let database_url = "postgresql://postgres:postgres@my-psql.orb.local:5432/rust_rest_auth";
    let port = 9191;
    let pool = connect_database(Some(database_url.to_string())).await;

    println!("Server started at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                user_repo: http::user::repository::UserRepository::new(),
                token_repo: http::token::repository::TokenRepository::new(),
                session_repo: http::session::repository::SessionRepository::new(),
                auth_repo: http::auth::repository::AuthRepository::new(),
                jwt_repo: http::jwt::repository::JWTRepository::new("apollo".to_string(), 3600),
            }))
            .wrap(Logger::default())
            .configure(http::user::config_user)
            // .configure(http::token::config_token)
            // .configure(http::session::config_session)
            .configure(http::auth::config_auth)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
