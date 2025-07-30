#![allow(unused)]

// use reqwest::Result;
use std::time::Duration;

use anyhow::Result;
// Use the asynchronous cache.
use moka::future::Cache;
use reqwest::ClientBuilder;
// [finish](https://github.com/john-cd/rust_howto/issues/1421)
// use reqwest::IntoUrl;
// use reqwest::Request;
// use tracing::debug;
// use tracing::error;
// use tracing::info;
// use tracing::trace;
// use tracing::warn;
use url::Url;

/// Launch the Tokio runtime the web link checking will run on.
pub fn launch() -> Result<()> {
    // <https://tokio.rs/tokio/topics/bridging>
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all() // Enables both I/O and time drivers.
        .build()?;

    let cache: Cache<String, Url> = Cache::new(5_000);

    const NUM_TASKS: usize = 20;
    let tasks: Vec<_> = (0..NUM_TASKS)
        .map(|i| {
            // To share the same cache across the async tasks, clone it.
            // This is a cheap operation.
            let my_cache = cache.clone();
            let my_urls = Vec::<Url>::new();

            runtime.spawn(async move {
                // my_cache.insert(key, Value).await;
                // (my_cache.get(&key).await;
                check_web_urls(my_urls).await
            })
        })
        .collect();

    // Synchronous tasks go here.

    // Wait for all asynchronous tasks to complete.
    for t in tasks {
        // The `spawn` method returns a `JoinHandle`. A `JoinHandle` is
        // a future, so we can wait for it using `block_on`.
        let _ = runtime.block_on(t)?; // FIXME
    }
    Ok(())
}

// Check a list of urls:
async fn check_web_urls(urls: Vec<url::Url>) -> Result<UrlChecks> {
    let tasks: Vec<_> = urls.into_iter().map(check_web_url).collect();
    // Wait for all tasks to complete.
    futures_util::future::join_all(tasks)
        .await
        .into_iter()
        .collect::<Result<UrlChecks>>()
}

type UrlChecks = Vec<UrlCheck>;

struct UrlCheck {
    url: Url,
    success: bool,
    status: reqwest::StatusCode,
    final_url: Url,
}

// Query using a HEAD request (Client::head) and then inspect the response code
// to determine success. This is a quick way to query a rest resource without
// needing to receive a body. <https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html#check-if-an-api-resource-exists>
async fn check_web_url(url: Url) -> Result<UrlCheck> {
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new()
        //.user_agent(APP_USER_AGENT)
        //.default_headers(headers)
        .timeout(timeout)
        .build()?;
    let response = client.head(url.clone()).send().await?;

    Ok(UrlCheck {
        url,
        success: response.status().is_success(),
        status: response.status(),
        final_url: response.url().clone(),
    })
}
