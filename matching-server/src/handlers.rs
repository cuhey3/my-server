use axum::http::StatusCode;
use std::fmt::Display;

pub mod send_sdp;
pub mod signaling_answer;
pub mod start_matching;

pub fn to_http_error<T: Display>(err: T, message: &str) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("{}: {}", message, err),
    )
}

pub fn none_to_http_error(message: &str) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_owned())
}
