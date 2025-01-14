use actix_web::{web, HttpResponse};
use sqlx::query;

use crate::{utils::convert::OffsetDateTimeToDateTime, AppState};

use super::user::{NewUser, RespondUser, UpdateUser};

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_user(
        &self,
        user: NewUser,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
            &user.username,
            &user.email,
            &user.password
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

    pub async fn get_users(
        &self,
        token: &str,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!("SELECT * FROM users").fetch_all(&data.db).await;
        match result {
            Ok(r) => {
                let users: Vec<RespondUser> = r
                    .iter()
                    .map(|row| RespondUser {
                        id: row.id,
                        username: row.username.clone(),
                        email: row.email.clone(),
                        created_at: row.created_at.to_datetime_utc(),
                        updated_at: row.updated_at.to_datetime_utc(),
                        is_active: row.is_active.unwrap_or_default(),
                    })
                    .collect();
                Ok(HttpResponse::Ok().json(users))
            }
            Err(_) => Err(HttpResponse::InternalServerError().finish()),
        }
    }

    pub async fn update_user(
        &self,
        token: &str,
        user: UpdateUser,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!(
            "UPDATE users SET password_hash = COALESCE($1, password_hash), is_active = COALESCE($2, is_active) WHERE id = $3 RETURNING *",
            match user.password {
            Some(p) => Some(p),
            None => None,
            },
            match user.is_active {
            Some(a) => Some(a),
            None => None,
            },
            &user.id
        )
        .fetch_one(&data.db)
        .await;
        match result {
            Ok(res) => {
                let user = RespondUser {
                    id: res.id,
                    username: res.username,
                    email: res.email,
                    created_at: res.created_at.to_datetime_utc(),
                    updated_at: res.updated_at.to_datetime_utc(),
                    is_active: res.is_active.unwrap_or_default(),
                };
                Ok(HttpResponse::Ok().json(user))
            }
            Err(_) => Err(HttpResponse::InternalServerError().finish()),
        }
    }
}
