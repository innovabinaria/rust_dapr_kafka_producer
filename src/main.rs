mod config;
mod handler;
mod kafka_client;
mod models;

use axum::{routing::post, Router};
use config::Config;
use handler::send_message_handler;
use kafka_client::DaprKafkaClient;
use tokio::net::TcpListener;
use tracing_subscriber;
use std::net::SocketAddr;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let cfg = Config::from_env()?;

    tracing::info!("Loaded config: topic={}, port={}", cfg.topic, cfg.server_port);

    let dapr_addr_grpc_port = format!("https://{}:{}", cfg.dapr_addr,cfg.dapr_grpc_port);
    let kafka_client = DaprKafkaClient::new(&cfg.pubsub_component, &cfg.topic, &dapr_addr_grpc_port).await?;

    let app = Router::new()
        .route("/send", post(send_message_handler))
        .with_state(kafka_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.server_port));
    tracing::info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
