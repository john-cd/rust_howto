// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parse a multipart email and print its subject and plain-text part.
//!
//! A multipart email carries several versions of the body, for example plain
//! text and HTML, plus attachments, each as a separate part. Mail clients show
//! the richest part they support, and programs usually read the plain-text
//! part.

use anyhow::Context as _;
use anyhow::Result;
use mailparse::MailHeaderMap as _;
use mailparse::parse_mail;

/// A sample multipart message with plain-text and HTML alternatives.
const RAW_MESSAGE: &str = concat!(
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

/// Returns the subject and the decoded text/plain parts of a raw message.
fn parse_subject_and_text(raw: &[u8]) -> Result<(String, Vec<String>)> {
    let parsed = parse_mail(raw).context("parse email message")?;
    let subject = parsed
        .headers
        .get_first_value("Subject")
        .unwrap_or_else(|| "(no subject)".to_owned());

    let mut bodies = Vec::new();
    for part in parsed.parts() {
        if part.subparts.is_empty() && part.ctype.mimetype == "text/plain" {
            bodies.push(part.get_body().context("decode text/plain part")?);
        }
    }
    Ok((subject, bodies))
}

/// Parses a sample message and prints its subject and plain-text body.
fn main() -> Result<()> {
    let (subject, bodies) = parse_subject_and_text(RAW_MESSAGE.as_bytes())?;
    println!("Subject: {subject}");
    for body in bodies {
        println!("Plain-text body: {body}");
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

    /// Only the text/plain part is returned, decoded from quoted-printable.
    #[test]
    fn extracts_subject_and_decoded_plain_text() -> Result<()> {
        let (subject, bodies) = parse_subject_and_text(RAW_MESSAGE.as_bytes())?;
        assert_eq!(subject, "Multipart message example");
        assert_eq!(bodies.len(), 1);
        assert!(bodies[0].starts_with("Hello, \u{2713}"));
        Ok(())
    }

    /// A message without a Subject header falls back to a placeholder.
    #[test]
    fn missing_subject_uses_placeholder() -> Result<()> {
        let (subject, bodies) = parse_subject_and_text(
            b"From: a@example.com\r\n\r\nplain body\r\n",
        )?;
        assert_eq!(subject, "(no subject)");
        assert_eq!(bodies, vec!["plain body\r\n".to_owned()]);
        Ok(())
    }
}
