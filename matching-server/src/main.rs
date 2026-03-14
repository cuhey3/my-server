mod app_state;
mod handlers;
mod matcher;
mod routes;

use crate::app_state::AppState;
use crate::routes::Routes;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{Level, event};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = SharedState::default();

    let app = Routes::add_routes(Router::new(), shared_state);

    let result = tokio::net::TcpListener::bind("127.0.0.1:3000").await;

    let Ok(listener) = result else {
        event!(Level::ERROR, "listening failed: {:?}", result.err());
        panic!();
    };

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    if let Err(err) = axum::serve(listener, app).await {
        event!(Level::ERROR, "starting server failed: {}", err);
    };
}
