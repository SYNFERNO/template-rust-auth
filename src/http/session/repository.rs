use actix_web::{web, HttpResponse};
use sqlx::query;

use crate::{
    http::token,
    utils::convert::{InetToString, StringToInet},
    AppState,
};

use super::session::NewSession;

pub struct SessionRepository;

impl SessionRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_sessions(
        &self,
        session: NewSession,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!(
            "INSERT INTO sessions (user_id, token_id, device_info, ip_address) VALUES ($1, $2, $3, $4)",
            &session.user_id,
            &session.token_id,
            session.device_info,
            session.ip_address.to_inet()
        )
        .execute(&data.db)
        .await;
        match result {
            Ok(_) => Ok(HttpResponse::Created().finish()),
            Err(err) => {
                let error = format!("Failed to create sessions: {}", err);
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

    pub async fn get_sessions(
        &self,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!("SELECT * FROM sessions").fetch_all(&data.db).await;
        match result {
            Ok(r) => {
                let sessions: Vec<NewSession> = r
                    .iter()
                    .map(|row| NewSession {
                        device_info: row.device_info.clone(),
                        ip_address: Some(row.ip_address.to_string()),
                        token_id: row.token_id,
                        user_id: row.user_id,
                    })
                    .collect();
                Ok(HttpResponse::Ok().json(sessions))
            }
            Err(_) => Err(HttpResponse::InternalServerError().finish()),
        }
    }

    pub async fn remove_sessions(
        &self,
        token: &str,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!("DELETE FROM sessions").execute(&data.db).await;
        match result {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Err(HttpResponse::InternalServerError().finish()),
        }
    }
}
