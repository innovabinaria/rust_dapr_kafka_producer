use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,
    pub pubsub_component: String,
    pub topic: String,
    pub dapr_addr: String,
    pub dapr_grpc_port: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let builder = config::Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .set_default("SERVER_PORT", 3000)?
            .set_default("PUBSUB_COMPONENT", "kafka-pubsub")?
            .set_default("TOPIC", "my-topic")?
            .set_default("DAPR_ADDR", "localhost")?
            .set_default("DAPR_GRPC_PORT", "50001")?;
            
        builder.build()?.try_deserialize()
    }
}
