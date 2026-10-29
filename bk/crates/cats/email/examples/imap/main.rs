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

    /// Runs the example against a real IMAP server.
    #[test]
    #[ignore = "needs IMAP_HOST, IMAP_USER, IMAP_PASSWORD and a reachable IMAP server"]
    fn require_external_svc() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// The helper modules the example delegates to filter and summarize mail.
    #[test]
    fn helpers_work_offline() {
        assert_eq!(
            mailchecker::deliverable(&["a@gmail.com", "b@mailinator.com"]),
            vec!["a@gmail.com"]
        );
        let summary = imap_fetch::summarize(
            b"From: a@example.com\r\nSubject: Hi\r\n\r\nBody\r\n",
        )
        .expect("message parses");
        assert_eq!(summary.subject, "Hi");
    }
}
