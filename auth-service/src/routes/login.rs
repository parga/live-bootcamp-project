use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{email::Email, error::AuthAPIError, password::Password},
    AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email.as_str());
    let password = Password::parse(request.password.as_str());

    if email.is_err() {
        println!("email is incorrect {:?}", email);
        return Err(AuthAPIError::InvalidCredentials);
    }

    if password.is_err() {
        println!("password is incorrect {:?}", password);
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user_store = state.user_store.read().await;
    if user_store.validate_user(email.unwrap(), password.unwrap()).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let response = Json(LoginResponse{
        message: "Successful login".to_string(),
    });

    Ok((StatusCode::OK,response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LoginResponse {
    pub message: String,
}
