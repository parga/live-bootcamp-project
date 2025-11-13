use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::data_stores::{LoginAttemptId, TwoFACode};
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::utils::auth::generate_auth_cookie;
use color_eyre::eyre::{eyre, Result};

#[tracing::instrument(name = "Sending email", skip_all)]
pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = Email::parse(request.email.as_str());
    let two_fa_code = TwoFACode::parse(request.two_fa_code);
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id);

    if email.is_err() || two_fa_code.is_err() || login_attempt_id.is_err() {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    }

    let email = email.unwrap();
    let two_fa_code = two_fa_code.unwrap();
    let login_attempt_id = login_attempt_id.unwrap();

    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let code_info = two_fa_code_store.get_code(&email).await;

    let (store_login_attempt_id, store_two_fa_code) = match code_info {
        Ok(val) => val,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    if two_fa_code != store_two_fa_code || login_attempt_id != store_login_attempt_id {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let _ = two_fa_code_store
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await;

    let result = two_fa_code_store.remove_code(&email).await;

    match result {
        Ok(_) => {
            let cookie = match generate_auth_cookie(&email) {
                Ok(cookie) => cookie,
                Err(_) => {
                    return (
                        jar,
                        Err(AuthAPIError::UnexpectedError(eyre!(
                            "Error generating auth cookie"
                        ))),
                    )
                }
            };

            let updated_jar = jar.add(cookie);
            (updated_jar, Ok(()))
        }
        Err(e) => (
            jar,
            Err(AuthAPIError::UnexpectedError(eyre!(
                e.to_string()
            ))),
        ),
    }
}

#[derive(Deserialize, Serialize)]
pub struct Verify2FARequest {
    email: String,

    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,

    #[serde(rename = "2FACode")]
    two_fa_code: String,
}
