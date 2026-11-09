// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fetch unread messages over IMAP and parse them with [`mail_parser`].
//!
//! Needs a server: set `IMAP_HOST`, `IMAP_USER` and `IMAP_PASSWORD`
//! (use an app password, and never log credentials). The connection uses
//! implicit TLS on port 993. For a local test server, optionally set
//! `IMAP_PORT` and `MAIL_TLS_ROOT_CERT` to trust an additional PEM root
//! certificate while retaining certificate and hostname validation.
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
///
/// # Errors
///
/// Returns an error if configuration, TLS setup, IMAP login, search, fetch, or
/// logout fails.
pub async fn run() -> Result<()> {
    for summary in fetch_unseen().await? {
        println!("{summary:?}");
    }
    Ok(())
}

/// Fetches and summarizes all unseen messages in the configured inbox.
///
/// # Errors
///
/// Returns an error if configuration, TLS setup, IMAP login, search, fetch, or
/// logout fails.
pub async fn fetch_unseen() -> Result<Vec<Summary>> {
    let host = std::env::var("IMAP_HOST").context("IMAP_HOST is not set")?;
    let user = std::env::var("IMAP_USER").context("IMAP_USER is not set")?;
    let password = std::env::var("IMAP_PASSWORD").context("IMAP_PASSWORD is not set")?;

    let port: u16 = match std::env::var("IMAP_PORT") {
        Ok(port) => port.parse().context("IMAP_PORT is not a port number")?,
        Err(std::env::VarError::NotPresent) => 993,
        Err(error) => return Err(error).context("read IMAP_PORT"),
    };
    let stream = TcpStream::connect((host.as_str(), port)).await?;
    let tls = build_tls_connector()?;
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

    let mut summaries = Vec::new();
    for id in session.search("UNSEEN").await? {
        let messages = session.fetch(id.to_string(), "RFC822").await?;
        let messages: Vec<_> = messages.try_collect().await?;
        for message in &messages {
            if let Some(summary) = message.body().and_then(summarize) {
                summaries.push(summary);
            }
        }
    }
    session.logout().await?;
    Ok(summaries)
}

/// Builds an IMAP TLS connector and optionally adds a PEM root certificate.
///
/// # Errors
///
/// Returns an error if the optional root certificate cannot be read or parsed,
/// or if the TLS connector cannot be built.
fn build_tls_connector() -> Result<TlsConnector> {
    let mut builder = native_tls::TlsConnector::builder();
    match std::env::var("MAIL_TLS_ROOT_CERT") {
        Ok(path) => {
            let pem = std::fs::read(&path)
                .with_context(|| format!("read MAIL_TLS_ROOT_CERT from {path}"))?;
            let certificate = native_tls::Certificate::from_pem(&pem)
                .context("parse MAIL_TLS_ROOT_CERT as a PEM certificate")?;
            builder.add_root_certificate(certificate);
        }
        Err(std::env::VarError::NotPresent) => {}
        Err(error) => return Err(error).context("read MAIL_TLS_ROOT_CERT"),
    }
    let connector = builder.build().context("build IMAP TLS connector")?;
    Ok(TlsConnector::from(connector))
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
