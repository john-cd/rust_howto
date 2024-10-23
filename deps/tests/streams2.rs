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

#[tokio::test]
async fn test() -> Result {
    let urls = ["https://www.gutenberg.org/cache/epub/43/pg43.txt"]; // add more here...
    let filenames = ["temp/file1.txt"]; // add more here...

    let futures = urls
        .iter()
        .zip(filenames.iter())
        .map(|(url, filename)| download_file(url, filename));

    let fut = futures::stream::iter(futures).for_each_concurrent(
        4,
        |fut| async move {
            if let Err(e) = fut.await {
                println!("Error: {}", e);
                if let Some(source) = e.source() {
                    println!("  Caused by: {}", source);
                }
            }
        },
    );

    fut.await;

    println!("Downloaded files successfully!");
    Ok(())
}
