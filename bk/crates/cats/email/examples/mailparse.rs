// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parse a multipart email and print its subject and plain-text part.

use anyhow::Context as _;
use anyhow::Result;
use mailparse::MailHeaderMap as _;
use mailparse::parse_mail;

/// Parses a sample message and prints its subject and plain-text body.
fn main() -> Result<()> {
    let raw_message = concat!(
        "From: sender@example.com\r\n",
        "To: reader@example.com\r\n",
        "Subject: Multipart message example\r\n",
        "MIME-Version: 1.0\r\n",
        "Content-Type: multipart/alternative; boundary=boundary-42\r\n",
        "\r\n",
        "--boundary-42\r\n",
        "Content-Type: text/plain; charset=utf-8\r\n",
        "Content-Transfer-Encoding: quoted-printable\r\n",
        "\r\n",
        "Hello, =E2=9C=93\r\n",
        "--boundary-42\r\n",
        "Content-Type: text/html; charset=utf-8\r\n",
        "\r\n",
        "<p>Hello, &#10003;</p>\r\n",
        "--boundary-42--\r\n",
    );

    let parsed =
        parse_mail(raw_message.as_bytes()).context("parse email message")?;
    let subject = parsed
        .headers
        .get_first_value("Subject")
        .unwrap_or_else(|| "(no subject)".to_owned());
    println!("Subject: {subject}");

    for part in parsed.parts() {
        if part.subparts.is_empty() && part.ctype.mimetype == "text/plain" {
            let body = part.get_body().context("decode text/plain part")?;
            println!("Plain-text body: {body}");
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the sample multipart message parses successfully.
    #[test]
    fn test_mailparse_example_entry_point() -> Result<()> {
        main()
    }
}
