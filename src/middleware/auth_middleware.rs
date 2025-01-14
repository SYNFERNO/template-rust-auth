use actix_web::{web, HttpRequest, HttpResponse};

use crate::AppState;

pub async fn auth_middleware(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<String, HttpResponse> {
    let token = req.headers().get("Authorization");
    match token {
        Some(token) => {
            let token = token.to_str().unwrap().to_string();
            let result = data.jwt_repo.verify_token(&token);
            match result {
                Ok(_) => Ok(token),
                Err(e) => Err(HttpResponse::Unauthorized().json("Invalid Token")),
            }
        }
        None => Err(HttpResponse::Unauthorized().finish()),
    }
}
