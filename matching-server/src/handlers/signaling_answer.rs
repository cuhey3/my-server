use crate::AppState;
use crate::handlers::{none_to_http_error, to_http_error};
use crate::matcher::Matcher;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use matching_if::structs::via_http::common::SignalingResponseType;
use matching_if::structs::via_http::signaling_answer::{
    SignalingAnswerRequest, SignalingAnswerResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn signaling_answer_handler(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SignalingAnswerRequest>,
) -> Result<(StatusCode, Json<SignalingAnswerResponse>), (StatusCode, String)> {
    let mut state = state.lock().await;

    let matcher = Matcher::new_from_signaling_answer_request(&payload);

    let user_id = &payload.user_id;

    let peer_connection_wrapper = state
        .find_wrapper_by_user_id(&matcher, user_id)
        .ok_or_else(|| {
            none_to_http_error(
                format!("peer connection wrapper not found by user id: {}", user_id).as_str(),
            )
        })?;

    peer_connection_wrapper
        .set_answer(&payload.answer)
        .map_err(|err| to_http_error(err, "setting answer failed"))?;

    let response = SignalingAnswerResponse {
        signaling_response_type: SignalingResponseType::AnswerAccepted,
    };

    Ok((StatusCode::OK, Json(response)))
}
