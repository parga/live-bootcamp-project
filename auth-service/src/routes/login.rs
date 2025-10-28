use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        data_stores::{LoginAttemptId, TwoFACode},
        email::Email,
        error::AuthAPIError,
        password::Password,
    },
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

    let user = match user_store.get_user(email.clone()).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    match user.requires_2fa {
        true => handle_2fa(&user.email, &state.clone(), jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let mut store = state.two_fa_code_store.write().await;
    let two_fa_code = TwoFACode::default();
    let login_atempt_id = LoginAttemptId::default();

    let result = store
        .add_code(email.clone(), login_atempt_id.clone(), two_fa_code)
        .await;

    match result {
        Ok(_) => {
            let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                message: "2FA required".to_owned(),
                login_attempt_id: login_atempt_id.as_ref().to_owned(), 
            }));

            (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
        }
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = generate_auth_cookie(email).unwrap();
    let updated_jar = jar.add(auth_cookie);

    (
        updated_jar,
        Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
    )
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
