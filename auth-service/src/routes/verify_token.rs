use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{domain::error::AuthAPIError, routes::VerifyTokenRequest, utils::auth::validate_token};

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
    let token = request.token;
    let response = validate_token(&token);
    if response.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }
    Ok(StatusCode::OK)
}

