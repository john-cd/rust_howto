# Streams

Futures are about a single value that will eventually be produced, but many event sources naturally produce a stream of values over time.

```rust,ignore
use futures::stream::{self, StreamExt};

async fn count_to_five() -> impl Stream<Item = u32> {
    stream::iter(1..=5)
}

#[tokio::main]
async fn main() {
    let mut stream = count_to_five().await;
    // `for` loops are not usable with Streams, but for imperative-style code, `while let` and the `next`/`try_next` functions can be used:
    while let Some(num) = stream.next().await {
        println!("{}", num);
    }
}
```

There are combinator-style methods such as `map`, `filter`, and `fold`, and their early-exit-on-error cousins `try_map`, `try_filter`, and `try_fold`.

To process multiple items from a stream concurrently, use the `for_each_concurrent` and `try_for_each_concurrent` methods:

```rust,ignore
use futures::StreamExt;
use tokio::fs::File;
use tokio::io;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

async fn download_file(url: &str, filename: &str) -> Result {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    let mut file = File::create(filename).await?;
    io::copy(&mut content.as_ref(), &mut file).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result {
    let urls = ["https://www.gutenberg.org/cache/epub/43/pg43.txt"]; // add more here...
    let filenames = ["file1.txt"];

    let futures = urls.iter().zip(filenames.iter()).map( |(url, filename)| download_file(url, filename));

    let fut = futures::stream::iter(futures).for_each_concurrent(4, |fut| async move {
        if let Err(e) = fut.await {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        }
    });

    fut.await;

    println!("Downloaded files successfully!");
    Ok(())
}
```

See also [Tokio async-stream]( https://github.com/tokio-rs/async-stream )
