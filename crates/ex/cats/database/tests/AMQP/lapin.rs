// ANCHOR: example
use futures::stream::StreamExt; /* or: use futures_lite::stream::StreamExt; */
use lapin::BasicProperties;
use lapin::Channel;
use lapin::Connection;
use lapin::ConnectionProperties;
use lapin::Queue;
use lapin::options::*;
use lapin::types::FieldTable;

// AMQP client library for e.g. RabbitMQ

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to RabbitMQ server
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into());
    let conn = Connection::connect(
        &addr,
        ConnectionProperties::default(),
    )
    .await?;
    // Main entry point for most AMQP operations.
    // Channel serves as a "lightweight connection"
    // and can be obtained from a Connection
    let channel: Channel = conn.create_channel().await?;

    // Declare a queue
    let _queue: Queue = channel
        .queue_declare(
            "my_queue",
            QueueDeclareOptions::default(), // Whether the queue is passive, durable, exclusive, auto_delete...
            FieldTable::default(), // a Map<String, AMQPValue>
        )
        .await?;

    // Publish a message to the queue
    let message = "Hello from Rust!";
    channel
        .basic_publish(
            "", // exchange
            "my_queue", // routing key
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
        // In this exmample, we quit after the first and only message.
        break;
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    unsafe {
        // Refer to the compose*.yaml files
        std::env::set_var(
            "AMQP_ADDR",
            "amqp://guest:guest@rust_howto_dev-amqp-1:5672",
        );
    }
    main()?;
    Ok(())
}
// [ P2 write; https://lib.rs/crates/lapin](https://github.com/john-cd/rust_howto/issues/1015)
