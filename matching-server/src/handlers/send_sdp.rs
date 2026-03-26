use crate::AppState;
use crate::handlers::{none_to_http_error, to_http_error};
use crate::matcher::Matcher;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use matching_if::structs::via_http::common::{SdpType, SignalingResponseType};
use matching_if::structs::via_http::send_sdp::{SendSdpRequest, SendSdpResponse};
use matching_if::types::UserId;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::Sender;

pub async fn send_sdp_handler(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<SendSdpRequest>,
) -> Result<(StatusCode, Json<SendSdpResponse>), (StatusCode, String)> {
    let matcher = Matcher::new_from_send_sdp_request(&payload);

    let user_id = &payload.user_id;

    let opponent_id = &payload.opponent_id;

    // offerer なら opponent_id がないものを検索する(待っている相手は opponent_id を知らないため)
    // answerer なら opponent_id を指定して検索する
    let opponent_id_for_search = if matches!(payload.sdp_type, SdpType::Offer(_)) {
        None
    } else {
        Some(payload.opponent_id)
    };

    let (_, sender) = state
        .lock()
        .await
        .find_waiting_user_by_id(&matcher, &payload.opponent_id, opponent_id_for_search)
        .ok_or_else(|| {
            none_to_http_error(
                format!(
                    "peer connection wrapper not found by user id: {}",
                    opponent_id
                )
                .as_str(),
            )
        })?;

    match payload.sdp_type {
        SdpType::Offer(offer) => {
            sender
                .send((payload.user_id, offer))
                .await
                .map_err(|err| to_http_error(err, "send offer by sender failed"))?;

            let (sender, mut receiver) = tokio::sync::mpsc::channel(1);

            state
                .lock()
                .await
                // 自分が offerer なら、相手から自分の user_id を opponent_id として検索してほしいのでキーにセットする
                .insert_waiting_user(&matcher, user_id, Some(*user_id), sender);

            let (opponent_user_id, answer) = receiver
                .recv()
                .await
                .ok_or(none_to_http_error("receive answer by sender failed"))?;

            Ok((
                StatusCode::OK,
                Json(SendSdpResponse {
                    answer,
                    opponent_user_id,
                    signaling_response_type: SignalingResponseType::NotRequired,
                }),
            ))
        }
        SdpType::Answer(answer) => {
            Ok(send_answer(&payload.user_id, opponent_id, answer, sender).await?)
        }
    }
}

async fn send_answer(
    own_user_id: &UserId,
    opponent_user_id: &UserId,
    answer: String,
    sender: Sender<(UserId, String)>,
) -> Result<(StatusCode, Json<SendSdpResponse>), (StatusCode, String)> {
    sender
        .send((*own_user_id, answer))
        .await
        .map_err(|err| to_http_error(err, "send answer by sender failed"))?;

    Ok((
        StatusCode::OK,
        Json(SendSdpResponse {
            answer: "".to_owned(),
            opponent_user_id: *opponent_user_id,
            signaling_response_type: SignalingResponseType::NotRequired,
        }),
    ))
}
