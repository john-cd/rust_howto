// ANCHOR: example
//! Example of sending an email using the `lettre` crate.
//!
//! This example demonstrates how to construct an email message,
//! configure an SMTP transport, and send the email.
//!
//! **Note:** This example requires valid SMTP credentials to run.
//! You will need to replace `"smtp_username"` and `"smtp_password"`
//! with your actual credentials.

use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

fn main() -> anyhow::Result<()> {
    let smtp_server = std::env::var("SMTP_SERVER")
        .unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_username = std::env::var("SMTP_USERNAME")
        .unwrap_or_else(|_| "smtp_username".to_string());
    let smtp_password = std::env::var("SMTP_PASSWORD")
        .unwrap_or_else(|_| "smtp_password".to_string());

    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Someone <someone@domain.tld>".parse().unwrap())
        .to("Buddy <buddy@domain.tld>".parse().unwrap())
        .subject("Subject here")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Body text here"))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    println!("Email sent successfully!");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Requires SMTP credentials"]
    fn require_external_svc() -> anyhow::Result<()> {
        if std::env::var("SMTP_USERNAME").is_err()
            || std::env::var("SMTP_PASSWORD").is_err()
        {
            eprintln!(
                "Skipping SMTP integration test; set SMTP_USERNAME and SMTP_PASSWORD to run."
            );
            return Ok(());
        }

        main()?;
        Ok(())
    }
}
// [review; Requires valid SMTP credentials to run](https://github.com/john-cd/rust_howto/issues/1144)
