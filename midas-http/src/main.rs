const DEFAULT_BIND_ADDR: &str = "0.0.0.0:8000";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化Rayon线程池
    midas_core::init_rayon_pool();
    
    // 初始化tracing
    tracing_subscriber::fmt::init();

    // cors
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // init route
    let app = axum::Router::new()
        .route(
            "/indexCode/list",
            axum::routing::get(midas_http::controller::index_code::list),
        )
        .route(
            "/indexData/list/{code}",
            axum::routing::get(midas_http::controller::index_data::list_by_code),
        )
        .route(
            "/simulate",
            axum::routing::post(midas_http::controller::simulate::simulate),
        )
        .route(
            "/simulate/multiple",
            axum::routing::post(midas_http::controller::simulate::simulate_multiple),
        )
        .layer(cors);

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or(DEFAULT_BIND_ADDR.to_string());
    tracing::info!("bind_addr: {:?}", bind_addr);

    let tcp_listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    // run app
    let app = app.into_make_service_with_connect_info::<std::net::SocketAddr>();
    axum::serve(tcp_listener, app).await?;

    Ok(())
}
