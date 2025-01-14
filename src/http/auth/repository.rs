use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::query;

use crate::{
    http::{
        jwt::jwt::{Claims, RefreshClaims},
        session::session::NewSession,
        token::token::NewToken,
    },
    utils::bcrypt::hash_password,
    AppState,
};

use super::auth::{Login, Register};

pub struct AuthRepository;

impl AuthRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn login(
        &self,
        login: Login,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let encrypt_password = hash_password(&login.password).unwrap();
        let result = query!(
            "SELECT * FROM users WHERE username = $1 OR email = $1 AND password_hash = $2",
            login.email,
            encrypt_password
        )
        .fetch_one(&data.db)
        .await;

        let trx = data.db.begin().await;

        match trx {
            Ok(tx) => match result {
                Ok(r) => {
                    let user = r;
                    let duration = 3600;

                    let new_access_token = data.jwt_repo.generate_token::<Claims>(&Claims::new(
                        &user.id.to_string(),
                        &user.username.to_string(),
                        duration,
                    ));

                    match new_access_token {
                        Ok(access_token) => {
                            let new_refresh_token =
                                data.jwt_repo
                                    .generate_token::<RefreshClaims>(&RefreshClaims::new(
                                        &user.id.to_string(),
                                        duration,
                                    ));

                            match new_refresh_token {
                                Ok(refresh_token) => {
                                    let db_token = NewToken::new(
                                        user.id,
                                        refresh_token.clone(),
                                        "refresh_token".to_string(),
                                        duration,
                                    );

                                    let result_token =
                                        data.token_repo.create_token(db_token, data.clone()).await;
                                    match result_token {
                                        Ok(token) => {
                                            let session = NewSession {
                                                user_id: user.id,
                                                token_id: token,
                                                device_info: None,
                                                ip_address: None,
                                            };

                                            let result_session = data
                                                .session_repo
                                                .create_sessions(session, data.clone())
                                                .await;

                                            match result_session {
                                                Ok(_) => {
                                                    let _ = tx.commit().await;
                                                    return Ok(HttpResponse::Ok().json({
                                                        json!({
                                                            "access_token": access_token,
                                                            "refresh_token": refresh_token,
                                                            "expires_in": duration,
                                                            "user": json!({
                                                                "id": user.id,
                                                                "username": user.username,
                                                                "email": user.email})
                                                        })
                                                    }));
                                                }
                                                Err(err) => {
                                                    let _ = tx.rollback().await;
                                                    return Err(err);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            let _ = tx.rollback().await;
                                            return Err(err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    let _ = tx.rollback().await;
                                    return Err(
                                        HttpResponse::InternalServerError().json(err.to_string())
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            let _ = tx.rollback().await;
                            return Err(HttpResponse::InternalServerError().json(err.to_string()));
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.rollback().await;
                    Err(HttpResponse::NotFound().json("Credentials not found or incorrect"))
                }
            },
            Err(err) => {
                return Err(HttpResponse::InternalServerError()
                    .json(format!("Failed to start transaction: {}", err)))
            }
        }
    }

    pub async fn register(
        &self,
        register: Register,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let encrypted_password = hash_password(&register.password).unwrap();
        let result = query!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
            register.username,
            register.email,
            encrypted_password
        )
        .execute(&data.db)
        .await;

        match result {
            Ok(_) => Ok(HttpResponse::Created().finish()),
            Err(err) => {
                let error = format!("Failed to create user: {}", err);
                if err
                    .to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    if err.to_string().contains("username") {
                        return Ok(HttpResponse::Conflict().json("Username already exists"));
                    } else if err.to_string().contains("email") {
                        return Ok(HttpResponse::Conflict().json("Email already exists"));
                    }
                }
                Err(HttpResponse::InternalServerError().json(error))
            }
        }
    }

    pub async fn logout(
        &self,
        token: &str,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!("DELETE FROM sessions ").execute(&data.db).await;

        match result {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => Err(HttpResponse::InternalServerError().json(err.to_string())),
        }
    }
}
