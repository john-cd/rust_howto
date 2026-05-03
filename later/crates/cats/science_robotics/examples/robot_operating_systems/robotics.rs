#![cfg(all(feature = "rosrust", not(windows)))] // TODO review Windows support
#![allow(dead_code)]
// ANCHOR: example
//! Example: publish and subscribe to a simple ROS topic using `rosrust`.
//!
//! This minimal ROS node publishes a `std_msgs/String` message and also
//! subscribes to the same topic, printing any received messages.

use rosrust::api::raii::Publisher;
use rosrust_msg::std_msgs::String;

fn main() {
    rosrust::init("rust_howto_robotics_example");

    let _subscriber = rosrust::subscribe("/hello", 10, |msg: String| {
        println!("received: {}", msg.data);
    })
    .expect("failed to subscribe");

    let publisher: Publisher<String> =
        rosrust::publish("/hello", 10).expect("failed to create publisher");

    let mut count = 0;
    while rosrust::is_ok() && count < 5 {
        let mut msg = String::default();
        msg.data = format!("hello from Rust #{}", count + 1);
        publisher.send(msg).expect("failed to publish");
        rosrust::spin_once();
        std::thread::sleep(std::time::Duration::from_secs(1));
        count += 1;
    }

    println!("ROS example finished.");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
// [review](https://github.com/john-cd/rust_howto/issues/844)
