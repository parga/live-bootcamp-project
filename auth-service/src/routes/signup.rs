use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::error::AuthAPIError;
use crate::domain::user::User;
use crate::services::hashmap_user_store::UserStoreError;
use crate::AppState;

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    if email.is_empty() || !email.contains("@") || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    match user_store.add_user(user) {
        Ok(_) => {},
        Err(err) => {
            return match err {
                UserStoreError::UserAlreadyExists => Err(AuthAPIError::UserAlreadyExists),
                _ => Err(AuthAPIError::UnexpectedError)
            };
        } ,
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
