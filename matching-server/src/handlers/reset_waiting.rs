use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn reset_waiting_handler(
    state: State<Arc<Mutex<AppState>>>,
) -> Result<StatusCode, (StatusCode, String)> {
    state.lock().await.clear_matcher_to_user_id();
    Ok(StatusCode::OK)
}
