use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn login_returns_signup_ui() {
    let app = TestApp::new().await;

    let response = app.get_signup().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn login_returns_login_ui() {
    let app = TestApp::new().await;

    let response = app.get_login().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn login_returns_logout_ui() {
    let app = TestApp::new().await;

    let response = app.get_logout().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn login_returns_verify_2fa_success() {
    let app = TestApp::new().await;

    let response = app.get_verify_2fa().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn login_returns_verify_token_success() {
    let app = TestApp::new().await;

    let response = app.get_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.
