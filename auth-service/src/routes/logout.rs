use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
    AuthAPIError,
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = jar.get(JWT_COOKIE_NAME);
    if cookie.is_none() {
        return (jar, Err(AuthAPIError::MissingToken));
    }
    let cookie = cookie.unwrap();
    let token = cookie.value().to_owned();

    let validation_result = validate_token(&token);
    if validation_result.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    let jar = jar.remove(JWT_COOKIE_NAME);

    (jar, Ok(StatusCode::OK))
}
