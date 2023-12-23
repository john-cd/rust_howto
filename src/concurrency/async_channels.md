# Channels for use in async code

The most common form of synchronization in an async program is message passing. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state. Message passing is implemented using channels.

Tokio's `sync` module provides channels that work well with async code.

## OneShot

`oneshot` sends a single value from a single producer to a single consumer.
This channel is usually used to send the result of a computation to a waiter.

```rust,editable,ignore
use tokio::sync::oneshot;

async fn download_file() -> Result<String, std::io::Error> {
    // Simulate downloading a file
    let filename = "data.txt";
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Downloaded file: {}", filename);
    Ok(filename.to_owned())
}

async fn process_file(filename: String) {
    // Simulate processing the downloaded file
    println!("Processing file: {}", filename);
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Finished processing file.");
}

async fn main() -> Result<(), std::io::Error> {
    let (sender, receiver) = oneshot::channel();

    // Spawn the download task
    tokio::spawn(async move {
        let filename = download_file().await?;
        sender.send(filename).expect("Failed to send filename");
    });

    // Wait for the downloaded filename from the receiver
    let filename = receiver.await?;

    // Spawn the processing task with the filename
    tokio::spawn(async move {
        process_file(filename).await;
    });

    Ok(())
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async { main().await }).unwrap();
}
```

## Alternatives to Tokio `sync`

[Postage]( https://lib.rs/crates/postage )
