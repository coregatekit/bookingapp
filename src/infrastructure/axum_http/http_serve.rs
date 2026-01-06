use std::net::SocketAddr;

use anyhow::Result;
use axum::{Router, http::Method, routing::get};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use crate::infrastructure::axum_http::default_routers;

pub async fn start() -> Result<()> {
    let app = Router::new()
        .fallback(default_routers::not_found)
        .route("/health", get(default_routers::health_check))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await?;

    info!("🚀 Server is running on {}", addr);

    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;

    Ok(())
}

async fn shutdown_signal() {
  let ctrl_c = async {
    tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
  };

  let terminate = std::future::pending::<()>();

  tokio::select! {
    _ = ctrl_c => {
      info!("🛑 Received Ctrl+C signal, shutting down...");
    },
    _ = terminate => {
      info!("🛑 Received terminate signal, shutting down...");
    },
  }
}
