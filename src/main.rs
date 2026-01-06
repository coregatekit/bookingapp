use bookingapp::{config::config_loader, infrastructure::axum_http::http_serve::start};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_config = match config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            error!("❌ Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    info!("✅ Configuration loaded successfully: {:?}", dotenvy_config);

    start().await.expect("❌ Failed to start server!");
}
