#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A typed message bus: publish Protobuf messages through a trait, with an
//! in-memory implementation for tests and AWS SQS / SNS implementations for
//! production.
//!
//! The SQS and SNS implementations are compiled but never called by the tests:
//! they need AWS credentials and a queue.

use std::sync::Mutex;

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use async_trait::async_trait;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use prost::Message;

/// SQS rejects message bodies larger than 256 KiB.
const MAX_BODY_BYTES: usize = 256 * 1024;

/// A message published when an order is placed.
#[derive(Clone, PartialEq, Eq, Message)]
pub struct OrderPlaced {
    /// The order identifier.
    #[prost(string, tag = "1")]
    pub order_id: String,
    /// The number of items ordered.
    #[prost(uint32, tag = "2")]
    pub quantity: u32,
}

/// A message received from a queue, with the handle needed to acknowledge it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Received {
    /// The base64-encoded Protobuf payload.
    pub body: String,
    /// The handle to pass to [`MessageSubscriber::ack`].
    pub receipt: String,
}

/// Publishes encoded messages to a queue or topic.
#[async_trait]
pub trait MessagePublisher: Send + Sync {
    /// Sends one text body.
    async fn publish_body(&self, body: String) -> Result<()>;
}

/// Receives messages from a queue.
#[async_trait]
pub trait MessageSubscriber: Send + Sync {
    /// Returns the messages that are currently visible, if any.
    async fn receive(&self) -> Result<Vec<Received>>;

    /// Removes a processed message so that it is not delivered again.
    async fn ack(&self, receipt: &str) -> Result<()>;
}

/// Encodes a Protobuf message as base64 text, which is safe in an SQS body.
///
/// # Errors
///
/// Fails when the encoded body exceeds the SQS limit.
pub fn encode_body<M: Message>(message: &M) -> Result<String> {
    let body = STANDARD.encode(message.encode_to_vec());
    ensure!(
        body.len() <= MAX_BODY_BYTES,
        "message body is {} bytes; the SQS limit is {MAX_BODY_BYTES}",
        body.len()
    );
    Ok(body)
}

/// Decodes a base64 body produced by [`encode_body`].
///
/// # Errors
///
/// Fails when the text is not valid base64 or not a valid `M`.
pub fn decode_body<M: Message + Default>(body: &str) -> Result<M> {
    let bytes = STANDARD.decode(body).context("body is not base64")?;
    M::decode(bytes.as_slice()).context("body is not the expected message")
}

/// Encodes `message` and publishes it.
pub async fn publish<M: Message>(
    publisher: &dyn MessagePublisher,
    message: &M,
) -> Result<()> {
    publisher.publish_body(encode_body(message)?).await
}

/// Receives messages, runs `handler` on each, and acknowledges only the ones
/// the handler accepted. Failed messages become visible again after the
/// visibility timeout, so handlers must be idempotent (at-least-once delivery).
pub async fn consume<M, F>(
    subscriber: &dyn MessageSubscriber,
    mut handler: F,
) -> Result<usize>
where
    M: Message + Default,
    F: FnMut(M) -> Result<()>,
{
    let mut handled = 0;
    for received in subscriber.receive().await? {
        let outcome = decode_body::<M>(&received.body).and_then(&mut handler);
        if outcome.is_ok() {
            subscriber.ack(&received.receipt).await?;
            handled += 1;
        }
    }
    Ok(handled)
}

/// An in-memory queue used by tests instead of AWS.
#[derive(Default)]
pub struct InMemoryBus {
    /// Messages that have not been acknowledged.
    messages: Mutex<Vec<Received>>,
}

#[async_trait]
impl MessagePublisher for InMemoryBus {
    async fn publish_body(&self, body: String) -> Result<()> {
        let mut messages = self.messages.lock().expect("queue lock poisoned");
        let receipt = format!("receipt-{}", messages.len());
        messages.push(Received { body, receipt });
        Ok(())
    }
}

#[async_trait]
impl MessageSubscriber for InMemoryBus {
    async fn receive(&self) -> Result<Vec<Received>> {
        Ok(self.messages.lock().expect("queue lock poisoned").clone())
    }

    async fn ack(&self, receipt: &str) -> Result<()> {
        self.messages
            .lock()
            .expect("queue lock poisoned")
            .retain(|m| m.receipt != receipt);
        Ok(())
    }
}

/// An SQS queue. Requires AWS credentials and an existing queue.
pub struct SqsBus {
    /// The shared SQS client.
    client: aws_sdk_sqs::Client,
    /// The queue URL, e.g. `https://sqs.us-east-1.amazonaws.com/123/orders`.
    queue_url: String,
}

impl SqsBus {
    /// Creates a bus from a client built from a shared `SdkConfig`.
    pub fn new(client: aws_sdk_sqs::Client, queue_url: String) -> Self {
        Self { client, queue_url }
    }
}

#[async_trait]
impl MessagePublisher for SqsBus {
    async fn publish_body(&self, body: String) -> Result<()> {
        self.client
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(body)
            .send()
            .await
            .context("SQS SendMessage failed")?;
        Ok(())
    }
}

#[async_trait]
impl MessageSubscriber for SqsBus {
    async fn receive(&self) -> Result<Vec<Received>> {
        let output = self
            .client
            .receive_message()
            .queue_url(&self.queue_url)
            .max_number_of_messages(10)
            // Long polling avoids empty responses and reduces cost.
            .wait_time_seconds(20)
            // How long a message stays hidden while it is being handled.
            .visibility_timeout(30)
            .send()
            .await
            .context("SQS ReceiveMessage failed")?;
        Ok(output
            .messages()
            .iter()
            .filter_map(|m| {
                Some(Received {
                    body: m.body()?.to_owned(),
                    receipt: m.receipt_handle()?.to_owned(),
                })
            })
            .collect())
    }

    async fn ack(&self, receipt: &str) -> Result<()> {
        self.client
            .delete_message()
            .queue_url(&self.queue_url)
            .receipt_handle(receipt)
            .send()
            .await
            .context("SQS DeleteMessage failed")?;
        Ok(())
    }
}

/// An SNS topic, which fans a message out to every subscribed queue.
pub struct SnsTopic {
    /// The shared SNS client.
    client: aws_sdk_sns::Client,
    /// The topic ARN.
    topic_arn: String,
}

impl SnsTopic {
    /// Creates a publisher for the given topic.
    pub fn new(client: aws_sdk_sns::Client, topic_arn: String) -> Self {
        Self { client, topic_arn }
    }
}

#[async_trait]
impl MessagePublisher for SnsTopic {
    async fn publish_body(&self, body: String) -> Result<()> {
        self.client
            .publish()
            .topic_arn(&self.topic_arn)
            .message(body)
            .send()
            .await
            .context("SNS Publish failed")?;
        Ok(())
    }
}

/// Publishes one order to the in-memory bus and consumes it.
pub async fn demo() -> Result<()> {
    let bus = InMemoryBus::default();
    publish(
        &bus,
        &OrderPlaced {
            order_id: "A-1".to_owned(),
            quantity: 3,
        },
    )
    .await?;

    let handled = consume::<OrderPlaced, _>(&bus, |order| {
        println!("order {} x{}", order.order_id, order.quantity);
        Ok(())
    })
    .await?;
    assert_eq!(handled, 1);
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .build()?
        .block_on(demo())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds the sample message.
    fn order() -> OrderPlaced {
        OrderPlaced {
            order_id: "A-1".to_owned(),
            quantity: 3,
        }
    }

    /// Verifies that a message survives the base64 Protobuf round trip.
    #[test]
    fn body_round_trips() -> Result<()> {
        let body = encode_body(&order())?;
        assert_eq!(decode_body::<OrderPlaced>(&body)?, order());
        Ok(())
    }

    /// Verifies that oversized bodies are rejected before sending.
    #[test]
    fn oversized_body_is_rejected() {
        let big = OrderPlaced {
            order_id: "x".repeat(MAX_BODY_BYTES),
            quantity: 1,
        };
        assert!(encode_body(&big).is_err());
    }

    /// Verifies that a handled message is acknowledged exactly once.
    #[tokio::test]
    async fn handled_messages_are_acknowledged() -> Result<()> {
        let bus = InMemoryBus::default();
        publish(&bus, &order()).await?;
        assert_eq!(consume::<OrderPlaced, _>(&bus, |_| Ok(())).await?, 1);
        assert_eq!(consume::<OrderPlaced, _>(&bus, |_| Ok(())).await?, 0);
        Ok(())
    }

    /// Verifies that a failing handler leaves the message for redelivery.
    #[tokio::test]
    async fn failed_messages_are_redelivered() -> Result<()> {
        let bus = InMemoryBus::default();
        publish(&bus, &order()).await?;
        let handled = consume::<OrderPlaced, _>(&bus, |_| {
            anyhow::bail!("transient failure")
        })
        .await?;
        assert_eq!(handled, 0);
        assert_eq!(bus.receive().await?.len(), 1);
        Ok(())
    }

    /// Verifies that the demonstration runs.
    #[test]
    fn demo_runs() -> Result<()> {
        run()
    }
}
