use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Result;

use crate::{
    app_state::AppState,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
    AuthAPIError,
};

#[tracing::instrument(name = "Logging out", skip_all)]
pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = jar.get(JWT_COOKIE_NAME);
    if cookie.is_none() {
        return (jar, Err(AuthAPIError::MissingToken));
    }
    let cookie = cookie.unwrap();
    let token = cookie.value().to_owned();

    let validation_result = validate_token(&token, state.banned_token_store.clone()).await;
    if validation_result.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    if let Err(e) = state
        .banned_token_store
        .write()
        .await
        .add_token(token.to_owned())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    let jar = jar.remove(JWT_COOKIE_NAME);

    (jar, Ok(StatusCode::OK))
}
