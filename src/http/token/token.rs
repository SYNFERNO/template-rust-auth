use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_value: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewToken {
    pub user_id: Uuid,
    pub token_value: String,
    pub token_type: String,
    pub expires_at: i64,
}

impl NewToken {
    pub fn new(user_id: Uuid, token_value: String, token_type: String, seconds: i64) -> Self {
        let expires_at = (Utc::now() + chrono::Duration::seconds(seconds)).timestamp();

        Self {
            user_id,
            token_value,
            token_type,
            expires_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RequestToken {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateToken {
    pub id: Uuid,
    pub token_value: String,
    pub expires_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespondToken {
    pub token_value: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
}
