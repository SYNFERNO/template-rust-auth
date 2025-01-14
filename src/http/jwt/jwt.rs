use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: &str, name: &str, seconds: i64) -> Self {
        let exp = (chrono::Utc::now() + chrono::Duration::seconds(seconds)).timestamp() as usize;
        Self {
            sub: user_id.to_string(),
            name: name.to_string(),
            exp,
        }
    }

    pub fn validate_jwt(jwt: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::default();

        jsonwebtoken::decode::<Claims>(jwt, &decoding_key, &validation).map(|data| data.claims)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshClaims {
    pub sub: String,
    pub jti: String,
    pub exp: usize,
}

impl RefreshClaims {
    pub fn new(user_id: &str, seconds: i64) -> Self {
        let exp = (chrono::Utc::now() + chrono::Duration::seconds(seconds)).timestamp() as usize;
        let jti = uuid::Uuid::new_v4().to_string();
        Self {
            sub: user_id.to_string(),
            jti,
            exp,
        }
    }

    pub fn validate_jwt(jwt: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::default();

        jsonwebtoken::decode::<RefreshClaims>(jwt, &decoding_key, &validation)
            .map(|data| data.claims)
    }
}
