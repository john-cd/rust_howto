#![cfg(feature = "ros2")]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Publish and receive `std_msgs/String` messages with a ROS 2 node.

use std::time::Duration;
use std::time::Instant;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use futures::FutureExt;
use futures::StreamExt;
use r2r::Context;
use r2r::Node;
use r2r::QosProfile;
use r2r::std_msgs::msg::String as StringMessage;

/// Publishes five greetings and waits for the local subscriber to receive each.
///
/// # Errors
/// Returns initialization or communication failures, a closed subscription, or
/// a timeout if a greeting is not received within ten seconds.
fn main() -> Result<()> {
    let context = Context::create().context("initialize ROS 2")?;
    let mut node = Node::create(context, "rust_howto_pubsub", "")
        .context("create the ROS 2 node")?;
    let topic = "/rust_howto/hello";
    let mut subscriber = node
        .subscribe::<StringMessage>(topic, QosProfile::default())
        .context("subscribe to greetings")?;
    let publisher = node
        .create_publisher::<StringMessage>(topic, QosProfile::default())
        .context("create the greeting publisher")?;

    for sequence in 1..=5 {
        let message = StringMessage {
            data: format!("hello from Rust #{sequence}"),
        };
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if Instant::now() >= deadline {
                bail!("timed out receiving greeting #{sequence} on {topic}");
            }
            // Retry while DDS discovers the subscriber. Repeated greetings are
            // harmless here; a production protocol may need deduplication.
            publisher.publish(&message).context("publish a greeting")?;
            // spin_once moves ROS events into the stream; polling alone cannot.
            node.spin_once(Duration::from_millis(100));
            match subscriber.next().now_or_never() {
                Some(Some(received)) if received.data == message.data => {
                    println!("received: {}", received.data);
                    break;
                }
                Some(None) => bail!("the greeting subscription closed"),
                // Ignore earlier duplicates or messages from another publisher.
                Some(Some(_)) | None => {}
            }
        }
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the ROS 2 publisher/subscriber example.
///
/// # Errors
/// Returns ROS initialization, communication, or receive timeout failures.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    /// Runs the example only when an operator supplies a ROS 2 environment.
    #[test]
    #[ignore = "needs a ROS 2 environment"]
    fn test_robotics_example_entry_point() -> anyhow::Result<()> {
        let result = super::main();
        assert!(
            result.is_ok(),
            "greetings were not received back: {result:?}"
        );
        result
    }
}
