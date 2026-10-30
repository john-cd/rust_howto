// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Publishes orders to a NATS JetStream stream and processes them with a
//! durable pull consumer.
//!
//! The recipe shows the parts that matter for reliability:
//!
//! - the publisher waits for the server's acknowledgement and sets a
//!   `Nats-Msg-Id` header, so JetStream drops duplicate publishes;
//! - the consumer is durable, pulls bounded batches, and acknowledges each
//!   message only after it was handled;
//! - delivery is at least once, so handling is idempotent: a redelivered order
//!   is acknowledged without being applied twice;
//! - malformed messages are terminated instead of being redelivered forever.

use std::collections::HashSet;
use std::time::Duration;

use async_nats::jetstream;
use async_nats::jetstream::AckKind;
use async_nats::jetstream::consumer::AckPolicy;
use async_nats::jetstream::consumer::pull;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use tokio_util::sync::CancellationToken;

/// Maximum accepted payload size, in bytes.
const MAX_PAYLOAD: usize = 64 * 1024;
/// Maximum number of messages requested per pull.
const BATCH_SIZE: usize = 10;
/// How long a pull waits for messages before returning.
const BATCH_WAIT: Duration = Duration::from_secs(1);
/// Redeliveries stop after this many attempts.
const MAX_DELIVER: i64 = 5;

/// An order published on the `orders.new` subject.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Order {
    /// Business identifier, used to make processing idempotent.
    id: u64,
    /// Amount in cents.
    cents: u64,
}

/// What happened to a message.
#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    /// The order was applied.
    Applied,
    /// The order was seen before; nothing was done.
    Duplicate,
    /// The payload cannot be processed, now or later.
    Invalid,
}

/// Applies orders at most once per identifier.
#[derive(Default)]
struct Processor {
    /// Identifiers of the orders applied so far. A real service stores this
    /// in the same transaction as the order's effects.
    seen: HashSet<u64>,
    /// Total applied, in cents.
    total_cents: u64,
}

impl Processor {
    /// Decodes and applies one payload.
    fn process(&mut self, payload: &[u8]) -> Outcome {
        if payload.len() > MAX_PAYLOAD {
            return Outcome::Invalid;
        }
        let Ok(order) = serde_json::from_slice::<Order>(payload) else {
            return Outcome::Invalid;
        };
        if !self.seen.insert(order.id) {
            return Outcome::Duplicate;
        }
        self.total_cents = self.total_cents.saturating_add(order.cents);
        Outcome::Applied
    }
}

/// Publishes `order` and waits until JetStream has stored it.
async fn publish_order(
    js: &jetstream::Context,
    order: &Order,
) -> anyhow::Result<()> {
    let mut headers = async_nats::HeaderMap::new();
    // JetStream ignores a second publish with the same id within the
    // stream's duplicate window.
    headers.insert("Nats-Msg-Id", order.id.to_string().as_str());
    let payload = serde_json::to_vec(order)?;
    let ack = js
        .publish_with_headers("orders.new", headers, payload.into())
        .await?
        .await?;
    println!("stored order {} at sequence {}", order.id, ack.sequence);
    Ok(())
}

/// Pulls batches until `shutdown` is cancelled or a batch comes back empty.
async fn consume(
    stream: &jetstream::stream::Stream,
    processor: &mut Processor,
    shutdown: &CancellationToken,
) -> anyhow::Result<()> {
    let consumer = stream
        .get_or_create_consumer(
            "order-worker",
            pull::Config {
                durable_name: Some("order-worker".to_owned()),
                ack_policy: AckPolicy::Explicit,
                ack_wait: Duration::from_secs(30),
                max_deliver: MAX_DELIVER,
                ..Default::default()
            },
        )
        .await?;

    while !shutdown.is_cancelled() {
        let mut batch = consumer
            .fetch()
            .max_messages(BATCH_SIZE)
            .expires(BATCH_WAIT)
            .messages()
            .await?;
        let mut received = 0_usize;
        while let Some(message) = batch.next().await {
            let message = message.map_err(|error| anyhow::anyhow!(error))?;
            received += 1;
            match processor.process(&message.payload) {
                // Acknowledge only after the work is done, so a crash makes
                // the server redeliver the message.
                Outcome::Applied | Outcome::Duplicate => {
                    message.ack().await.map_err(|e| anyhow::anyhow!(e))?;
                }
                // `Term` tells the server never to redeliver this message.
                Outcome::Invalid => {
                    message
                        .ack_with(AckKind::Term)
                        .await
                        .map_err(|e| anyhow::anyhow!(e))?;
                }
            }
        }
        if received == 0 {
            break;
        }
    }
    Ok(())
}

/// Runs the recipe against the server in `NATS_URL`, if any.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Ok(url) = std::env::var("NATS_URL") else {
        println!(
            "Set NATS_URL (for example nats://127.0.0.1:4222) to a server \
             started with `nats-server -js` to run this example."
        );
        return Ok(());
    };
    let client = async_nats::connect(url).await?;
    let js = jetstream::new(client);
    let stream = js
        .get_or_create_stream(jetstream::stream::Config {
            name: "ORDERS".to_owned(),
            subjects: vec!["orders.>".to_owned()],
            duplicate_window: Duration::from_secs(120),
            ..Default::default()
        })
        .await?;

    for id in 1..=3 {
        publish_order(
            &js,
            &Order {
                id,
                cents: id * 100,
            },
        )
        .await?;
    }
    // Publishing the same id again is dropped by the server.
    publish_order(&js, &Order { id: 3, cents: 300 }).await?;

    let mut processor = Processor::default();
    let shutdown = CancellationToken::new();
    tokio::select! {
        result = consume(&stream, &mut processor, &shutdown) => result?,
        _ = tokio::signal::ctrl_c() => shutdown.cancel(),
    }
    println!("applied {} cents", processor.total_cents);
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Encodes an order the way the publisher does.
    fn encode(id: u64, cents: u64) -> Vec<u8> {
        serde_json::to_vec(&Order { id, cents }).expect("orders serialize")
    }

    /// A valid order is applied once.
    #[test]
    fn valid_order_is_applied() {
        let mut processor = Processor::default();
        assert_eq!(processor.process(&encode(1, 250)), Outcome::Applied);
        assert_eq!(processor.total_cents, 250);
    }

    /// A redelivered order is recognised and not applied twice.
    #[test]
    fn redelivery_is_idempotent() {
        let mut processor = Processor::default();
        assert_eq!(processor.process(&encode(7, 100)), Outcome::Applied);
        assert_eq!(processor.process(&encode(7, 100)), Outcome::Duplicate);
        assert_eq!(processor.total_cents, 100);
    }

    /// Malformed and oversized payloads are rejected without side effects.
    #[test]
    fn invalid_payloads_are_rejected() {
        let mut processor = Processor::default();
        assert_eq!(processor.process(b"not json"), Outcome::Invalid);
        assert_eq!(
            processor.process(&vec![b' '; MAX_PAYLOAD + 1]),
            Outcome::Invalid
        );
        assert_eq!(processor.total_cents, 0);
    }

    /// The amount cannot overflow.
    #[test]
    fn totals_saturate() {
        let mut processor = Processor::default();
        processor.process(&encode(1, u64::MAX));
        processor.process(&encode(2, 5));
        assert_eq!(processor.total_cents, u64::MAX);
    }

    /// Without `NATS_URL` the recipe explains how to run it and succeeds.
    #[test]
    fn main_without_server_is_a_no_op() {
        if std::env::var("NATS_URL").is_err() {
            assert!(super::main().is_ok());
        }
    }

    /// Publishes, consumes with a durable consumer and checks redelivery
    /// handling against a real server.
    ///
    /// Start one with `nats-server -js` (or Docker image `nats:2` with
    /// `-js`) and set `NATS_URL=nats://127.0.0.1:4222`.
    #[test]
    #[ignore = "needs a NATS server with JetStream enabled (NATS_URL)"]
    fn jetstream_round_trip() -> anyhow::Result<()> {
        main()
    }
}
