#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Publishing and consuming a message with the `lapin` AMQP client.

use futures::stream::StreamExt as _;
use lapin::BasicProperties;
use lapin::Channel;
use lapin::Connection;
use lapin::ConnectionProperties;
use lapin::Consumer;
use lapin::Queue;
use lapin::options::BasicAckOptions;
use lapin::options::BasicConsumeOptions;
use lapin::options::BasicPublishOptions;
use lapin::options::ConfirmSelectOptions;
use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use lapin::types::ShortString;

/// Publishes a message to a queue and waits for confirmation.
async fn publish_and_confirm(
    channel: &Channel,
    queue_name: &ShortString,
    message: &str,
) -> anyhow::Result<()> {
    let confirmation = channel
        .basic_publish(
            "".into(),
            queue_name.clone(),
            BasicPublishOptions::default(),
            message.as_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?;

    anyhow::ensure!(
        confirmation.is_ack(),
        "broker did not acknowledge publish"
    );
    Ok(())
}

/// Consumes a message from a consumer stream and acknowledges it.
async fn consume_and_ack(
    consumer: &mut Consumer,
    expected_message: &str,
) -> anyhow::Result<()> {
    let delivery = consumer.next().await.ok_or_else(|| {
        anyhow::anyhow!("consumer stopped before receiving a message")
    })??;
    let received = String::from_utf8_lossy(&delivery.data);
    anyhow::ensure!(received == expected_message, "received an unexpected message");

    anyhow::ensure!(
        delivery.ack(BasicAckOptions::default()).await?,
        "delivery was not acknowledged"
    );
    Ok(())
}

/// Connects to RabbitMQ, publishes a message, consumes it, and acknowledges it.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://127.0.0.1:5672".into());
    let conn =
        Connection::connect(&addr, ConnectionProperties::default()).await?;

    let channel: Channel = conn.create_channel().await?;

    let queue: Queue = channel
        .queue_declare(
            "".into(),
            QueueDeclareOptions {
                exclusive: true,
                auto_delete: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;
    let queue_name = queue.name().clone();

    channel
        .confirm_select(ConfirmSelectOptions::default())
        .await?;

    let message = "Hello from Rust!";
    publish_and_confirm(&channel, &queue_name, message).await?;
    println!("Sent message: {message}");

    let mut consumer = channel
        .basic_consume(
            queue_name,
            "consumer_tag".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Waiting for messages...");
    consume_and_ack(&mut consumer, message).await?;
    println!("Received and acknowledged message.");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the publish, consume, and acknowledgement round trip with RabbitMQ.
    #[test]
    #[ignore = "needs RabbitMQ"]
    fn rabbitmq_round_trip() -> anyhow::Result<()> {
        main()
    }
}
