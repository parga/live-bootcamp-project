use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{email::Email, error::AuthAPIError, password::Password},
    utils::auth::generate_auth_cookie,
    AppState,
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = Email::parse(request.email.as_str());
    let password = Password::parse(request.password.as_str());

    if email.is_err() {
        println!("email is incorrect {:?}", email);
        return (jar, Err(AuthAPIError::InvalidCredentials));
    }

    if password.is_err() {
        println!("password is incorrect {:?}", password);
        return (jar, Err(AuthAPIError::InvalidCredentials));
    }
    let email = email.unwrap();
    let password = password.unwrap();

    let user_store = state.user_store.read().await;
    if user_store
        .validate_user(email.clone(), password)
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let auth_cookie = generate_auth_cookie(&email).unwrap();


    let updated_jar = jar.add(auth_cookie);
    println!("succesfull login");

    (updated_jar, Ok(StatusCode::OK.into_response()))
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


