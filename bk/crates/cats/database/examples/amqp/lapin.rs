#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Publishing and consuming a message with the `lapin` AMQP client.

// This example demonstrates how to publish and consume messages with the
// `lapin` AMQP client.
use futures::stream::StreamExt as _; /* or: use futures_lite::stream::StreamExt; */
use lapin::BasicProperties;
use lapin::Channel;
use lapin::Connection;
use lapin::ConnectionProperties;
use lapin::Queue;
use lapin::options::BasicAckOptions;
use lapin::options::BasicConsumeOptions;
use lapin::options::BasicPublishOptions;
use lapin::options::ConfirmSelectOptions;
use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;

/// # AMQP (Advanced Message Queuing Protocol) Client Example with `lapin`
///
/// This example demonstrates basic usage of the `lapin` crate, an AMQP client
/// library, to interact with a message broker like `RabbitMQ`.
///
/// ## Features
///
/// - Connects to a `RabbitMQ` server.
/// - Declares a queue.
/// - Publishes a message to the queue.
/// - Consumes a message from the queue.
/// Connects to RabbitMQ, publishes a message, consumes it, and acknowledges it.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to a RabbitMQ server:
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://127.0.0.1:5672".into());
    let conn =
        Connection::connect(&addr, ConnectionProperties::default()).await?;

    // Main entry point for most AMQP operations. A `Channel` serves as a
    // "lightweight connection" and can be obtained from a `Connection`.
    let channel: Channel = conn.create_channel().await?;

    // Declare a temporary, server-named queue for this run.
    //
    // `QueueDeclareOptions` allows specifying properties like whether the queue
    // is passive, durable, exclusive, auto_delete, etc.
    // `FieldTable` is a map of string keys to AMQP values for additional
    // arguments.
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

    // Request broker acknowledgements for published messages on this channel.
    channel
        .confirm_select(ConfirmSelectOptions::default())
        .await?;

    // Publish a message to the queue.
    //
    // - The first argument is the exchange name (empty string for the default
    //   exchange).
    // - The second argument is the routing key (the generated queue name).
    // - `BasicPublishOptions` controls routing behavior such as whether the
    //   broker returns an unroutable message.
    let message = "Hello from Rust!";
    let confirmation = channel
        .basic_publish(
            "".into(),          // exchange
            queue_name.clone(), // routing key
            BasicPublishOptions::default(),
            message.as_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?; // Wait for confirmation

    anyhow::ensure!(
        confirmation.is_ack(),
        "broker did not acknowledge publish"
    );

    println!("Sent message: {message}");

    // Consume messages from the queue.
    //
    // - The first argument is the queue name.
    // - The second argument is the consumer tag (a client-generated
    //   identifier).
    // - `BasicConsumeOptions` allows specifying properties like no_local,
    //   no_ack, exclusive, etc.
    // - `FieldTable` is for additional arguments.
    let mut consumer = channel
        .basic_consume(
            queue_name,
            "consumer_tag".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Process messages:
    println!("Waiting for messages...");
    let delivery = consumer.next().await.ok_or_else(|| {
        anyhow::anyhow!("consumer stopped before receiving a message")
    })??;
    let received = String::from_utf8_lossy(&delivery.data);
    anyhow::ensure!(received == message, "received an unexpected message");
    println!("Received message: {received}");

    // Acknowledge the delivery so the broker can remove it from the queue.
    anyhow::ensure!(
        delivery.ack(BasicAckOptions::default()).await?,
        "delivery was not acknowledged"
    );

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
    ///
    /// Start the broker with the Docker command shown in the chapter first.
    #[test]
    #[ignore = "needs RabbitMQ"]
    fn rabbitmq_round_trip() -> anyhow::Result<()> {
        // `.devcontainer/compose-heavy-tests.yaml` configures AMQP_ADDR for the
        // book service that runs this external-service test.
        main()
    }
}
