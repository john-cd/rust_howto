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
    mailchecker::run();
    imap_fetch::run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example against a real IMAP server.
    #[tokio::test]
    #[ignore = "needs IMAP settings, an expected subject, and an unseen message"]
    async fn require_external_svc() -> anyhow::Result<()> {
        use anyhow::Context as _;

        let expected_subject = std::env::var("IMAP_EXPECTED_SUBJECT")
            .context("set IMAP_EXPECTED_SUBJECT to the seeded message subject")?;
        let summaries = imap_fetch::fetch_unseen().await?;
        assert!(
            summaries
                .iter()
                .any(|summary| summary.subject == expected_subject),
            "no unseen message had subject {expected_subject:?}"
        );
        Ok(())
    }

    /// The helper modules the example delegates to filter and summarize mail.
    #[test]
    fn helpers_work_offline() {
        assert_eq!(
            mailchecker::deliverable(&["a@gmail.com", "b@mailinator.com"]),
            vec!["a@gmail.com"]
        );
        let summary = imap_fetch::summarize(b"From: a@example.com\r\nSubject: Hi\r\n\r\nBody\r\n")
            .expect("message parses");
        assert_eq!(summary.subject, "Hi");
    }
}
