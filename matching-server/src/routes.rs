use crate::SharedState;
use crate::handlers::reset_waiting::reset_waiting_handler;
use crate::handlers::send_sdp::send_sdp_handler;
use crate::handlers::start_matching::start_matching_handler;
use crate::handlers::web_rtc::send_sdp::send_sdp_handler as web_rtc_send_sdp_handler;
use crate::handlers::web_rtc::signaling_answer::signaling_answer_handler;
use crate::handlers::web_rtc::start_matching::start_matching_handler as web_rtc_start_matching_handler;
use axum::handler::Handler;
use axum::{Router, routing};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct Routes {}

impl Routes {
    pub fn add_routes(router: Router, shared_state: SharedState) -> Router {
        router
            .route(
                "/reset-waiting",
                routing::get_service(reset_waiting_handler.with_state(Arc::clone(&shared_state))),
            )
            .route(
                "/start-matching",
                routing::post_service(start_matching_handler.with_state(Arc::clone(&shared_state))),
            )
            .route(
                "/web_rtc/start-matching",
                routing::post_service(
                    web_rtc_start_matching_handler.with_state(Arc::clone(&shared_state)),
                ),
            )
            .route(
                "/web_rtc/signaling-answer",
                routing::post_service(
                    signaling_answer_handler.with_state(Arc::clone(&shared_state)),
                ),
            )
            .route(
                "/send-sdp",
                routing::post_service(send_sdp_handler.with_state(Arc::clone(&shared_state))),
            )
            .route(
                "/web_rtc/send-sdp",
                routing::post_service(
                    web_rtc_send_sdp_handler.with_state(Arc::clone(&shared_state)),
                ),
            )
            .layer(CorsLayer::permissive())
    }
}
