use crate::SharedState;
use crate::handlers::send_sdp::send_sdp_handler;
use crate::handlers::signaling_answer::signaling_answer_handler;
use crate::handlers::start_matching::start_matching_handler;
use axum::handler::Handler;
use axum::{Router, routing};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct Routes {}

impl Routes {
    pub fn add_routes(router: Router, shared_state: SharedState) -> Router {
        router
            .route(
                "/start-matching",
                routing::post_service(start_matching_handler.with_state(Arc::clone(&shared_state))),
            )
            .route(
                "/signaling-answer",
                routing::post_service(
                    signaling_answer_handler.with_state(Arc::clone(&shared_state)),
                ),
            )
            .route(
                "/send-sdp",
                routing::post_service(send_sdp_handler.with_state(Arc::clone(&shared_state))),
            )
            .layer(CorsLayer::permissive())
    }
}
