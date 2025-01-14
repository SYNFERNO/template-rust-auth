use actix_web::{web, HttpResponse};
use sqlx::query;
use uuid::Uuid;

use crate::{
    utils::convert::{OffsetDateTimeToDateTime, TimeMilisToDateTime},
    AppState,
};

use super::token::{NewToken, RespondToken, UpdateToken};

pub struct TokenRepository;

impl TokenRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_token(
        &self,
        token: NewToken,
        data: web::Data<AppState>,
    ) -> Result<Uuid, HttpResponse> {
        if token.token_type != "refresh_token" && token.token_type != "access_token" {
            return Err(HttpResponse::BadRequest().json("Invalid token type"));
        }
        let result = query!(
            "INSERT INTO tokens (user_id, token_value, token_type, expires_at) VALUES ($1, $2, $3, $4) RETURNING id",
            &token.user_id,
            &token.token_value,
            &token.token_type,
            token.expires_at.to_offset_datetime(),
        )
        .fetch_one(&data.db)
        .await;
        match result {
            Ok(r) => Ok(r.id),
            Err(err) => {
                let error = format!("Failed to create token: {}", err);

                if err
                    .to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    if err.to_string().contains("token_value") {
                        return Err(HttpResponse::Conflict().json("Token value already exists"));
                    }
                }

                Err(HttpResponse::InternalServerError().json(error))
            }
        }
    }

    pub async fn get_token_by_id(
        &self,
        id: Uuid,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!("SELECT * FROM tokens WHERE id = $1", id)
            .fetch_one(&data.db)
            .await;
        match result {
            Ok(res) => {
                let token = RespondToken {
                    token_value: res.token_value,
                    token_type: res.token_type,
                    expires_at: res.expires_at.to_datetime_utc(),
                };
                Ok(HttpResponse::Ok().json(token))
            }
            Err(err) => Err(HttpResponse::InternalServerError().json(err.to_string())),
        }
    }


    pub async fn update_token(
        &self,
        token: UpdateToken,
        data: web::Data<AppState>,
    ) -> Result<HttpResponse, HttpResponse> {
        let result = query!(
            "UPDATE tokens SET token_value = $1, expires_at = $2 WHERE id = $3 RETURNING *",
            &token.token_value,
            token.expires_at.to_offset_datetime(),
            &token.id,
        )
        .fetch_one(&data.db)
        .await;
        match result {
            Ok(res) => {
                let user = RespondToken {
                    token_value: res.token_value,
                    token_type: res.token_type,
                    expires_at: res.expires_at.to_datetime_utc(),
                };
                Ok(HttpResponse::Ok().json(user))
            }
            Err(err) => Err(HttpResponse::InternalServerError().json(err.to_string())),
        }
    }
}
