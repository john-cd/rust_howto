//! Read mail over IMAP and filter disposable addresses.
//!
//! TODO Reviewed ??-??-2026.

mod imap_fetch;
mod mailchecker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

/// Fetches unread mail from the configured IMAP server.
async fn run() -> anyhow::Result<()> {
    imap_fetch::run().await
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
