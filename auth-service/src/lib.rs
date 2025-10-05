use axum::{response::Html, routing::get, serve::Serve, Router};
use std::error::Error;
use tower_http::services::ServeDir;
// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
        // This is needed for Docker to work, which we will add later on.
        // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"));
            // .route("/hello", get(Self::hello_handler)); // Removed for now, but left here for reference

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
