use crate::AppState;
use crate::handlers::{none_to_http_error, to_http_error};
use crate::matcher::Matcher;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use matching_if::structs::via_http::common::{
    SignalingResponseType, UserIdRequestType, UserIdResponseType,
};
use matching_if::structs::via_http::start_matching::{
    StartMatchingRequest, StartMatchingResponse, StartMatchingResponseType,
};
use matching_if::types::UserId;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_matching_handler(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<StartMatchingRequest>,
) -> Result<(StatusCode, Json<StartMatchingResponse>), (StatusCode, String)> {
    let user_id_response_type = match payload.user_id_request_type {
        UserIdRequestType::Creating => {
            let created_user_id = getrandom::u32()
                .map_err(|err| to_http_error(err, "creating user id failed"))?
                as u64;
            UserIdResponseType::Created(created_user_id)
        }
        // TODO
        // UserId メンテナンスロジックを実装する
        UserIdRequestType::Updating => UserIdResponseType::Updated(222222),
        UserIdRequestType::Keep(_) => UserIdResponseType::Keep,
    };

    let own_user_id = payload
        .user_id_request_type
        .get_current_user_id(&user_id_response_type)
        .map_err(|err| to_http_error(err, "getting current user id failed"))?;

    let matcher = Matcher::new_from_start_matching_request(&payload);

    if !state.lock().await.has_waiting_user(&matcher) {
        return waiting_logic(state, user_id_response_type, own_user_id, &matcher).await;
    }

    let (user_id, _) = state
        .lock()
        .await
        // start_matching の waiting では当然、 opponent 不明のため指定なし
        .get_waiting_user_id(&matcher, None)
        .ok_or_else(|| {
            none_to_http_error(
                format!("matcher has waiting but no user id, matcher {:?}", matcher).as_str(),
            )
        })?;

    let response = StartMatchingResponse {
        user_id_response_type,
        signaling_response_type: SignalingResponseType::NotRequired,
        response_type: StartMatchingResponseType::Matched(user_id),
    };

    Ok((StatusCode::OK, Json(response)))
}

async fn waiting_logic(
    state: State<Arc<Mutex<AppState>>>,
    user_id_response_type: UserIdResponseType,
    user_id: UserId,
    matcher: &Matcher,
) -> Result<(StatusCode, Json<StartMatchingResponse>), (StatusCode, String)> {
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<(UserId, String)>(1);
    state
        .lock()
        .await
        // start_matching の waiting では当然、 opponent 不明のため指定なし
        .insert_waiting_user(matcher, &user_id, None, sender);

    let (opponent_user_id, offer) = receiver
        .recv()
        .await
        .ok_or_else(|| none_to_http_error("offer is none"))?;

    let response = StartMatchingResponse {
        user_id_response_type,
        signaling_response_type: SignalingResponseType::Offering(offer),
        response_type: StartMatchingResponseType::Matched(opponent_user_id),
    };

    Ok((StatusCode::OK, Json(response)))
}
