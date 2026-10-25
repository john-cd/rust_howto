#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Fetch unread messages over IMAP and parse them with [`mail_parser`].
//!
//! Needs a server: set `IMAP_HOST`, `IMAP_USER` and `IMAP_PASSWORD`
//! (use an app password, and never log credentials).

use anyhow::Context;
use anyhow::Result;
use mail_parser::MessageParser;

/// A message reduced to the fields we care about.
#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub subject: String,
    pub from: String,
}

/// Parses a raw RFC 822 message. Pure, so it is easy to unit test.
pub fn summarize(raw: &[u8]) -> Option<Summary> {
    let message = MessageParser::default().parse(raw)?;
    let from = message.from()?.first()?.address()?.to_owned();
    Some(Summary {
        subject: message.subject().unwrap_or_default().to_owned(),
        from,
    })
}

/// Connects over TLS (port 993 by default) and summarizes unseen messages.
pub fn run() -> Result<()> {
    let host = std::env::var("IMAP_HOST").context("IMAP_HOST is not set")?;
    let user = std::env::var("IMAP_USER").context("IMAP_USER is not set")?;
    let password =
        std::env::var("IMAP_PASSWORD").context("IMAP_PASSWORD is not set")?;

    let tls = native_tls::TlsConnector::builder().build()?;
    let client = imap::connect((host.as_str(), 993), host.as_str(), &tls)?;
    let mut session = client.login(&user, &password).map_err(|e| e.0)?;
    session.select("INBOX")?;

    for id in session.search("UNSEEN")? {
        let messages = session.fetch(id.to_string(), "RFC822")?;
        for message in messages.iter() {
            if let Some(summary) = message.body().and_then(summarize) {
                println!("{summary:?}");
            }
        }
    }
    session.logout()?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarizes_a_sample_message() {
        let eml = b"From: Alice <alice@example.com>\r\n\
To: bob@example.com\r\n\
Subject: Hello\r\n\r\nBody\r\n";
        assert_eq!(
            summarize(eml),
            Some(Summary {
                subject: "Hello".to_owned(),
                from: "alice@example.com".to_owned(),
            })
        );
    }

    #[test]
    fn rejects_garbage() {
        assert_eq!(summarize(b""), None);
    }
}
