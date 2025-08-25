#![allow(dead_code)]
// ANCHOR: example
use futures::stream::StreamExt; /* or: use futures_lite::stream::StreamExt; */
use lapin::BasicProperties;
use lapin::Channel;
use lapin::Connection;
use lapin::ConnectionProperties;
use lapin::Queue;
use lapin::options::*;
use lapin::types::FieldTable;

/// # AMQP (Advanced Message Queuing Protocol) Client Example with `lapin`
///
/// This example demonstrates basic usage of the `lapin` crate, an AMQP client
/// library, to interact with a message broker like RabbitMQ.
///
/// ## Features
///
/// - Connects to a RabbitMQ server.
/// - Declares a queue.
/// - Publishes a message to the queue.
/// - Consumes a message from the queue.
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

    // Declare a queue named "my_queue".
    //
    // `QueueDeclareOptions` allows specifying properties like whether the queue
    // is passive, durable, exclusive, auto_delete, etc.
    // `FieldTable` is a map of string keys to AMQP values for additional
    // arguments.
    let _queue: Queue = channel
        .queue_declare(
            "my_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(), // a Map<String, AMQPValue>
        )
        .await?;

    // Publish a message to the queue.
    //
    // - The first argument is the exchange name (empty string for the default
    //   exchange).
    // - The second argument is the routing key (queue name in this case).
    // - `BasicPublishOptions` allows specifying properties like mandatory,
    //   immediate, etc.
    let message = "Hello from Rust!";
    channel
        .basic_publish(
            "",         // exchange
            "my_queue", // routing key
            BasicPublishOptions::default(),
            message.as_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?; // Wait for confirmation

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
            "my_queue",
            "consumer_tag",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Process messages:
    println!("Waiting for messages...");
    if let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        println!(
            "Received message: {:?}",
            String::from_utf8_lossy(&delivery.data)
        );

        // Acknowledge the message.
        //
        // This tells the broker that the message has been processed and can be
        // removed from the queue.
        delivery.ack(BasicAckOptions::default()).await?;
        // In this example, we quit after the first and only message. In real
        // life, use `while let`.
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    unsafe {
        // Refer to the `compose*.yaml` files:
        std::env::set_var(
            "AMQP_ADDR",
            "amqp://guest:guest@rust_howto_dev-amqp-1:5672",
        );
    }
    main()?;
    Ok(())
}
