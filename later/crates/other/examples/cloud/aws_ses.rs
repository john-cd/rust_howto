// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! Send an HTML and plain-text email with Amazon SES v2.

use anyhow::Context;
use anyhow::Result;
use aws_sdk_sesv2::Client;
use aws_sdk_sesv2::types::Body;
use aws_sdk_sesv2::types::Content;
use aws_sdk_sesv2::types::Destination;
use aws_sdk_sesv2::types::EmailContent;
use aws_sdk_sesv2::types::Message;

/// Builds a UTF-8 text or HTML part.
fn content(text: &str) -> Result<Content> {
    Content::builder()
        .data(text)
        .charset("UTF-8")
        .build()
        .context("invalid email content")
}

/// Builds the message: clients that cannot render HTML fall back to the text.
pub fn build_email(
    subject: &str,
    text: &str,
    html: &str,
) -> Result<EmailContent> {
    let body = Body::builder()
        .text(content(text)?)
        .html(content(html)?)
        .build();
    let message = Message::builder()
        .subject(content(subject)?)
        .body(body)
        .build();
    Ok(EmailContent::builder().simple(message).build())
}

/// Sends the email and returns the SES message identifier.
///
/// The sender must be a verified identity; in the SES sandbox, so must the
/// recipient.
pub async fn send_email(
    client: &Client,
    from: &str,
    to: &str,
    email: EmailContent,
) -> Result<Option<String>> {
    let output = client
        .send_email()
        .from_email_address(from)
        .destination(Destination::builder().to_addresses(to).build())
        .content(email)
        .send()
        .await
        .context("SES SendEmail failed")?;
    Ok(output.message_id().map(str::to_owned))
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that both bodies and the subject are in the request.
    #[test]
    fn email_has_text_and_html_parts() -> Result<()> {
        let email = build_email("Hi", "plain", "<b>html</b>")?;
        let message = email.simple().expect("simple email");
        assert_eq!(message.subject().map(Content::data), Some("Hi"));
        let body = message.body().expect("body");
        assert_eq!(body.text().map(Content::data), Some("plain"));
        assert_eq!(body.html().map(Content::data), Some("<b>html</b>"));
        Ok(())
    }
}
