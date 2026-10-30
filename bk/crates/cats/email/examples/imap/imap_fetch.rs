// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fetch unread messages over IMAP and parse them with [`mail_parser`].
//!
//! Needs a server: set `IMAP_HOST`, `IMAP_USER` and `IMAP_PASSWORD`
//! (use an app password, and never log credentials).
//!
//! IMAP is the protocol mail clients use to read messages that stay stored on
//! the mail server, so several devices see the same mailbox. (Sending mail uses
//! a different protocol, SMTP.) Use an app password, a separate credential you
//! can revoke, rather than your main account password.

use anyhow::Context;
use anyhow::Result;
use async_imap::Client;
use futures::TryStreamExt;
use mail_parser::MessageParser;
use tokio::net::TcpStream;
use tokio_native_tls::TlsConnector;

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
pub async fn run() -> Result<()> {
    let host = std::env::var("IMAP_HOST").context("IMAP_HOST is not set")?;
    let user = std::env::var("IMAP_USER").context("IMAP_USER is not set")?;
    let password =
        std::env::var("IMAP_PASSWORD").context("IMAP_PASSWORD is not set")?;

    let stream = TcpStream::connect((host.as_str(), 993)).await?;
    let tls = TlsConnector::from(native_tls::TlsConnector::builder().build()?);
    let stream = tls.connect(&host, stream).await?;
    let mut client = Client::new(stream);
    client
        .read_response()
        .await?
        .context("IMAP server closed before sending its greeting")?;
    let mut session = client
        .login(&user, &password)
        .await
        .map_err(|(error, _client)| error)?;
    session.select("INBOX").await?;

    for id in session.search("UNSEEN").await? {
        let messages = session.fetch(id.to_string(), "RFC822").await?;
        let messages: Vec<_> = messages.try_collect().await?;
        for message in &messages {
            if let Some(summary) = message.body().and_then(summarize) {
                println!("{summary:?}");
            }
        }
    }
    session.logout().await?;
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
