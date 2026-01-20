use std::sync::Arc;

use bookingapp::{config::config_loader, infrastructure::{axum_http::http_serve::start, postgres::postgres_connection}};
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

    let postgres_pool = match postgres_connection::establish_connection(&dotenvy_config.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("❌ Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    };

    info!("✅ Database connection established successfully");

    start(Arc::new(dotenvy_config), Arc::new(postgres_pool)).await.expect("❌ Failed to start server!");
}
