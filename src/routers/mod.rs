use axum::{
    routing::{
        get, post
    },
    Router,
};

use crate::handlers::{
    healthz_handler,
    sensitive_detect_handler,
};

/**
 * @desc 总路由
 */
pub fn routers(
) -> Router {
    Router::new()
        .nest("/healthz", healthz_routers())
        .nest("/", sensitive_routers())
}

/**
 * @desc Healthz路由
 */
pub fn healthz_routers() -> Router {
    Router::new()
        .route("/", get(healthz_handler::healthz))
}

/**
 * @desc Sensitive路由
 */
pub fn sensitive_routers() -> Router {
    Router::new()
        .route("/sensitive", post(sensitive_detect_handler::sensitive_check))
}