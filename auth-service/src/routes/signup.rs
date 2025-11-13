use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use crate::domain::user::User;
use crate::domain::data_stores::UserStoreError;
use crate::AppState;

#[tracing::instrument(name = "Signup", skip_all)] // New!
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    Email::parse(request.email.as_str()).map_err(|_|AuthAPIError::InvalidCredentials)?;
    Password::parse(request.password.as_str()).map_err(|_|AuthAPIError::InvalidCredentials)?;

    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if let Err(err) = user_store.add_user(user).await {
        return match err {
                UserStoreError::UserAlreadyExists => Err(AuthAPIError::UserAlreadyExists),
                e => Err(AuthAPIError::UnexpectedError(e.into()))
            };
    }
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}
