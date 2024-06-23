use anyhow::Result;
use notify_server::{get_router, setup_pg_listener, AppConfig, AppState};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let config = AppConfig::load().expect("Failed to load config");
    let state = AppState::new(config);
    let app = get_router(state.clone());
    let addr = format!("0.0.0.0:{}", state.config.server.port);

    setup_pg_listener(state).await?;

    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
