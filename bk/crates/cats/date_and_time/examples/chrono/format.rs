#![allow(dead_code)]
// ANCHOR: example
use chrono::DateTime;
use chrono::Utc;

fn main() {
    // Get the current time in UTC.
    let now: DateTime<Utc> = Utc::now();

    // Print the current time in various formats.
    println!("UTC now is: {now}");
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
