use jsonwebtoken::{errors::Error, EncodingKey, Header};
use serde::Serialize;

use super::jwt::{Claims, RefreshClaims};
pub struct JWTRepository {
    pub secret_key: String,
    pub token_duration: i64,
}

impl JWTRepository {
    pub fn new(secret: String, token_duration: i64) -> Self {
        Self {
            secret_key: secret,
            token_duration,
        }
    }

    pub fn generate_token<T: Serialize>(
        &self,
        claims: &T,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(&self.secret_key.as_bytes());
        jsonwebtoken::encode(&header, claims, &encoding_key)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, Error> {
        Claims::validate_jwt(token, &self.secret_key)
    }

    pub fn refresh_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<(String, String), Error> {
        let refresh_claims = RefreshClaims::validate_jwt(refresh_token, &self.secret_key);

        match refresh_claims {
            Ok(refresh_claims) => {
                let access_claims = Claims::validate_jwt(access_token, &self.secret_key)?;
                let user_id = refresh_claims.sub.clone();
                let name = access_claims.name.clone();
                let new_access_claims = Claims::new(&user_id, &name, self.token_duration);
                let new_refresh_claims = RefreshClaims::new(&user_id, self.token_duration);

                let access_token = self.generate_token(&new_access_claims)?;

                let new_refresh_token = self.generate_token(&new_refresh_claims)?;
                Ok((access_token, new_refresh_token))
            }
            Err(e) => Err(e),
        }
    }
}
