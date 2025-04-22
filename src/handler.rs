use axum::{extract::State, http::StatusCode, Json};
use crate::models::Message;

use crate::kafka_client::DaprKafkaClient;


pub async fn send_message_handler(
    State(mut client): State<DaprKafkaClient>,
    Json(payload): Json<Message>,
) -> Result<StatusCode, StatusCode> {

    client.publish(payload).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}
