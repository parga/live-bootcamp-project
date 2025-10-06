use crate::helpers::TestApp;

#[tokio::test]
async fn login_returns_verify_2fa_success() {
    let app = TestApp::new().await;

    let response = app.get_verify_2fa().await;

    assert_eq!(response.status().as_u16(), 200);
}
