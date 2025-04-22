
# 🦀 Rust Dapr Kafka Producer

This project is a sample application written in **Rust** demonstrating how to **publish messages to Kafka using Dapr** and the asynchronous runtime **Tokio**, along with the **Axum** framework to expose an HTTP endpoint.

---

## 🚀 Technologies Used

- **Rust 2021 Edition**
- **Axum**: Asynchronous web framework.
- **Tokio**: Asynchronous runtime.
- **Serde / Serde JSON**: Data serialization and deserialization.
- **Dapr**: Microservices runtime.
- **Kafka**: Message broker for distributed systems.

---

## ⚙️ Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Dapr CLI](https://docs.dapr.io/get-dapr/cli/)
- [Docker](https://www.docker.com/)
- Kafka running on `localhost:9092` (can be via container)
- Dapr component file for Kafka (see below)

---

## 📦 Build

```bash
cargo build
```

---

## ▶️ Run with Dapr

First, make sure you have a Kafka component file named for example `components/kafka-pubsub.yaml` with the following content:

```yaml
apiVersion: dapr.io/v1alpha1
kind: Component
metadata:
  name: kafka-pubsub
  namespace: default
spec:
  type: pubsub.kafka
  version: v1
  metadata:
    - name: brokers
      value: "localhost:9092"
    - name: consumerGroup
      value: "rust-group"
    - name: authRequired
      value: "false"
    - name: publishTopic
      value: "my-topic"
```

Then run the app with Dapr:

```bash
dapr run   --app-id rust-kafka-producer   --app-port 3000   --dapr-http-port 3500   --dapr-grpc-port 50001   --resources-path ./components   cargo run
```

---

## 📤 Send a Message (with `curl` in bash)

You can send a message through the HTTP endpoint exposed by Axum:

```bash
curl -X POST http://localhost:3000/send   -H "Content-Type: application/json"   -d '{"id":"1", "value":"Hello from Rust + Kafka + Dapr!"}'
```

---

## 📋 Environment Variables Used

You can create a `.env` file with the following variables:

```env
APP_SERVER_PORT=3000
APP_PUBSUB_COMPONENT=kafka-pubsub
APP_TOPIC=my-topic
DAPR_GRPC_PORT=50001
```

These are automatically loaded thanks to the [`dotenv`](https://crates.io/crates/dotenv) dependency.

---

## 🧪 Viewing Messages in Kafka

If you're using Docker, you can connect to the Kafka container and run the consumer:

```bash
docker exec -it <KAFKA_CONTAINER_NAME> bash
kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic my-topic --from-beginning
```

*Replace `<KAFKA_CONTAINER_NAME>` with the actual name, e.g., `azurite-kafka-1`.*

---

