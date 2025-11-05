use auth_service::{domain::email::Email, utils::{auth::generate_auth_cookie }};
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;
    let token_request = serde_json::json!({
        "notToken": "ups",
    });

    let response = app.post_verify_token(&token_request).await;

    assert_eq!(response.status().as_u16(), 422);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let email = Email::parse(&get_random_email()).unwrap();
    let token = generate_auth_cookie(&email).unwrap();

    let mut app = TestApp::new().await;
    let token_request = serde_json::json!({
        "token": token.value(),
    });

    let response = app.post_verify_token(&token_request).await;

    assert_eq!(response.status().as_u16(), 200);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {

    let mut app = TestApp::new().await;
    let token_request = serde_json::json!({
        "token": "malformedToken",
    });

    let response = app.post_verify_token(&token_request).await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}
