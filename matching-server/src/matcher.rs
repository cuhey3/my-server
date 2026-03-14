use matching_if::structs::via_http::send_sdp::SendSdpRequest;
use matching_if::structs::via_http::signaling_answer::SignalingAnswerRequest;
use matching_if::structs::via_http::start_matching::StartMatchingRequest;
use matching_if::types::{AppId, MatcherId};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Matcher {
    app_id: AppId,
    matcher_id: MatcherId,
}

impl Matcher {
    pub fn new_from_start_matching_request(payload: &StartMatchingRequest) -> Self {
        Self {
            app_id: payload.app_id,
            matcher_id: payload.matcher_id,
        }
    }

    pub fn new_from_signaling_answer_request(payload: &SignalingAnswerRequest) -> Self {
        Self {
            app_id: payload.app_id,
            matcher_id: payload.matcher_id,
        }
    }

    pub fn new_from_send_sdp_request(payload: &SendSdpRequest) -> Self {
        Self {
            app_id: payload.app_id,
            matcher_id: payload.matcher_id,
        }
    }
}
