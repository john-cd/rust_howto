// ANCHOR: example

// AMQP client library

use futures::stream::StreamExt; /* or: use futures_lite::stream::StreamExt; */
use lapin::BasicProperties;
use lapin::Channel;
use lapin::Connection;
use lapin::ConnectionProperties;
use lapin::Queue;
use lapin::options::*;
use lapin::types::FieldTable;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to RabbitMQ server
    // let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_|
    // "amqp://127.0.0.1:5672/%2f".into());
    let conn = Connection::connect(
        "amqp://localhost/",
        ConnectionProperties::default(),
    )
    .await?;
    // Main entry point for most AMQP operations.
    // Channel serves as a "lightweight connection" and can be obtained from a
    // Connection
    let channel: Channel = conn.create_channel().await?;

    // Declare a queue
    let _queue: Queue = channel
        .queue_declare(
            "my_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(), // a Map<String, AMQPValue>
        )
        .await?;

    // Publish a message to the queue
    let message = "Hello from Rust!";
    channel
        .basic_publish(
            "",
            "my_queue",
            BasicPublishOptions::default(),
            message.as_bytes(),
            BasicProperties::default(),
        )
        .await?
        .await?; // Wait for confirmation

    println!("Sent message: {}", message);

    // Consume messages from the queue
    let mut consumer = channel
        .basic_consume(
            "my_queue",
            "consumer_tag",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Process messages
    println!("Waiting for messages...");
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        println!(
            "Received message: {:?}",
            String::from_utf8_lossy(&delivery.data)
        );

        // Acknowledge the message (i.e., the message is removed from the queue)
        delivery.ack(BasicAckOptions::default()).await?;
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [ P2 write; https://lib.rs/crates/lapin](https://github.com/john-cd/rust_howto/issues/1015)
