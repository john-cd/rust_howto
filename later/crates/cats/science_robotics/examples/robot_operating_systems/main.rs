//! Runs the robot operating systems example.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(feature = "ros2")]
mod ros2_pubsub;

/// Publishes and receives ROS 2 messages in a sourced ROS 2 environment.
///
/// # Errors
/// Returns ROS initialization, communication, or receive timeout failures.
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "ros2")]
    ros2_pubsub::run()?;
    Ok(())
}
