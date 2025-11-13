use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::error::AuthAPIError, utils::auth::validate_token};

#[tracing::instrument(name = "Verify token", skip_all)]
pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
    let token = request.token;
    let response = validate_token(&token, state.banned_token_store.clone()).await;
    if response.is_err() {
        return Err(AuthAPIError::InvalidToken);
    }

    tracing::debug!("token is verified and valid: {:?}", response);
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
