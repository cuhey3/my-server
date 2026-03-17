use crate::AppState;
use crate::handlers::{none_to_http_error, to_http_error};
use crate::matcher::Matcher;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use matching_if::structs::via_http::common::SignalingResponseType;
use matching_if::structs::via_http::send_sdp::{SendSdpRequest, SendSdpResponse};
use matching_if::structs::via_webrtc::receive_sdp::{ReceiveSdpOutboundData, ReceiveSdpReturnData};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn send_sdp_handler(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SendSdpRequest>,
) -> Result<(StatusCode, Json<SendSdpResponse>), (StatusCode, String)> {
    let mut state = state.lock().await;

    let matcher = Matcher::new_from_send_sdp_request(&payload);

    let opponent_id = &payload.opponent_id;
    let peer_connection_wrapper = state
        .find_wrapper_by_user_id(&matcher, opponent_id)
        .ok_or_else(|| {
            none_to_http_error(
                format!(
                    "peer connection wrapper not found by user id: {}",
                    opponent_id
                )
                .as_str(),
            )
        })?;

    peer_connection_wrapper
        .load_answer()
        .await
        .map_err(|err| to_http_error(err, "loading answer failed"))?;

    let data = ReceiveSdpOutboundData {
        matcher_id: 0,
        opponent_id: 0,
        offer: payload.offer,
    };

    peer_connection_wrapper
        .send_data(&data)
        .await
        .map_err(|err| to_http_error(err, "sending offer failed"))?;

    let message = peer_connection_wrapper
        .get_message_receiver()
        .await
        .map_err(|err| to_http_error(err, "getting message receiver failed"))?
        .recv()
        .await
        .ok_or_else(|| none_to_http_error("got message is none"))?;

    let ReceiveSdpReturnData { answer } =
        serde_json::from_slice::<ReceiveSdpReturnData>(&message.data)
            .map_err(|err| to_http_error(err, "invalid sdp answer received"))?;

    peer_connection_wrapper
        .close()
        .await
        .map_err(|err| to_http_error(err, "closing data channel failed"))?;

    state
        .remove_wrapper_by_user_id(&matcher, opponent_id)
        .map_err(|err| to_http_error(err, "wrapper remove failed"))?;

    Ok((
        StatusCode::OK,
        Json(SendSdpResponse {
            answer,
            opponent_user_id: 0,
            signaling_response_type: SignalingResponseType::NotRequired,
        }),
    ))
}
