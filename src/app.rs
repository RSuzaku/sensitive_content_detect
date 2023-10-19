use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

use tracing::info;

use axum::{
    http::{
        Method,
        header
    },
    Router,
};

use sensitive_detect_server::{
    base::error::AppError,
    init::{
        app_init,
        server_init,
        log_init,
        map_init,
    },
    routers,
};


#[tokio::main]
async fn main() -> Result<(),AppError> {
    // 初始化日志
    let _guard = log_init::setup_global_subscriber();

    // 读取配置
    let _ = app_init::init_config().await?;

    // 初始化字典
    let _ = map_init::init_sensitive_word_map().await?;

    // 初始化停用词
    let _ = map_init::init_stop_word().await?;

    // 解决跨域问题
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
        // allow requests from any origin
        .allow_origin(Any);

    let app: Router = Router::new()
        .nest("/", routers::routers())
        .layer(cors);

    let (ip_addr, port) = server_init::server_init().await?;

    let addr = SocketAddr::from((ip_addr, port));

    info!("Server running at http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}