use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_id: Uuid,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSession {
    pub user_id: Uuid,
    pub token_id: Uuid,
    pub device_info: Option<String>,
    pub ip_address: Option<String>,
}