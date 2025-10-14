use auth_service::{services::hashmap_user_store::HashmapUserStore, Application};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = auth_service::AppState::new(user_store);
    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build the application.");
    app.run().await.expect("Failed to run the application.");
}
