// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of sending an email using the `lettre` crate.
//!
//! Set `SMTP_SERVER`, `SMTP_USERNAME`, and `SMTP_PASSWORD` to the relay
//! settings. Set `SMTP_FROM` and `SMTP_TO` to valid mailbox addresses.
//!
//! The offline test builds a message without reading configuration or opening a
//! connection. The ignored integration test sends it through the configured
//! relay.

use anyhow::Context as _;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport as _;
use lettre::message::Mailbox;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

/// Builds a plain-text message from the supplied mailbox addresses.
///
/// # Errors
///
/// Returns an error if either address or the completed message is invalid.
fn build_message(from: &str, to: &str) -> anyhow::Result<Message> {
    let from: Mailbox = from.parse().context("parse SMTP_FROM mailbox")?;
    let to: Mailbox = to.parse().context("parse SMTP_TO mailbox")?;

    let message = Message::builder()
        .from(from)
        .to(to)
        .subject("Subject here")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Body text here"))?;

    Ok(message)
}

/// Runs the standalone SMTP delivery example.
///
/// # Errors
///
/// Returns an error if SMTP configuration is missing or delivery fails.
fn main() -> anyhow::Result<()> {
    send_email()
}

/// Builds a message and sends it through an authenticated SMTP relay.
///
/// # Errors
///
/// Returns an error if required environment variables are missing, the relay
/// configuration is invalid, or message delivery fails.
fn send_email() -> anyhow::Result<()> {
    let smtp_server = std::env::var("SMTP_SERVER")
        .context("set SMTP_SERVER to the SMTP relay host")?;
    let smtp_username = std::env::var("SMTP_USERNAME")
        .context("set SMTP_USERNAME to the SMTP account name")?;
    let smtp_password = std::env::var("SMTP_PASSWORD")
        .context("set SMTP_PASSWORD to the SMTP account password")?;
    let smtp_from = std::env::var("SMTP_FROM")
        .context("set SMTP_FROM to the sender mailbox")?;
    let smtp_to = std::env::var("SMTP_TO")
        .context("set SMTP_TO to the recipient mailbox")?;

    let email = build_message(&smtp_from, &smtp_to)?;

    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::relay(&smtp_server)
        .context("configure the SMTP relay")?
        .credentials(creds)
        .build();

    mailer.send(&email).context("send the email through SMTP")?;
    println!("Email sent successfully!");
    Ok(())
}
// ANCHOR_END: example

/// Runs the SMTP delivery example.
///
/// # Errors
///
/// Returns an error if SMTP configuration is missing or delivery fails.
pub fn run() -> anyhow::Result<()> {
    send_email()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds an email using reserved example addresses without contacting
    /// SMTP.
    #[test]
    fn builds_message_without_network_access() -> anyhow::Result<()> {
        build_message("sender@example.invalid", "recipient@example.invalid")?;
        Ok(())
    }

    /// Runs the example against its required external service.
    #[test]
    #[ignore = "needs SMTP credentials and a reachable SMTP relay"]
    fn require_external_svc() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
