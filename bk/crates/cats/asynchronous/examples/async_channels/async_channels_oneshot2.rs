#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the use of `tokio::sync::oneshot` channels to
//! communicate between asynchronous tasks. It simulates downloading a file
//! and then processing it in separate tasks.

use std::time::Duration;

use tokio::sync::oneshot;

/// Simulates downloading a file and returns its name.
async fn download_file() -> Result<String, std::io::Error> {
    // Simulate downloading a file:
    let filename = "data.txt";
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Downloaded file: {filename}");
    Ok(filename.to_owned())
}

/// Simulates processing the downloaded file named `filename`.
async fn process_file(filename: String) {
    // Simulate processing the downloaded file:
    println!("Processing file: {filename}");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finished processing file.");
}

/// Downloads a file in a spawned task, sends its name through a `oneshot`
/// channel, then processes it.
async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = oneshot::channel();

    // Spawn the download task:
    tokio::spawn(async move {
        let filename = download_file().await?;
        sender.send(filename).expect("Failed to send filename");
        Ok::<(), std::io::Error>(())
    });

    // Wait for the downloaded filename from the receiver:
    let filename = receiver.await?;

    // Spawn the processing task with the filename:
    tokio::spawn(async move {
        process_file(filename).await;
    });

    Ok(())
}

/// Creates a Tokio runtime and runs [`async_main`] on it.
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { async_main().await }).unwrap();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
