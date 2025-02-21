# Email

[![cat-email][cat-email-badge]][cat-email]{{hi:Email}}

Crates to help with sending, receiving, formatting, and parsing email.

## Send emails

{{#include send_emails.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[email/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/343)

## Sending Email

`lettre`: A widely used crate for sending emails. Supports various transports (SMTP, sendmail, etc.). Generally the recommended choice.
`mail-send`: Another option for sending emails.

## Email Parsing

`mailparse`: A crate for parsing email messages.

MIME (Multipurpose Internet Mail Extensions):

`mime`: A crate for working with MIME types and structures. Often used when constructing or parsing emails.

Building Email Messages:

Often done using a combination of the crates above. lettre provides convenient builders

## Asynchronous Email Sending

`lettre`: Supports asynchronous sending.

## TLS (Transport Layer Security)

`lettre`: Integrates with TLS for secure email transmission.
Authentication:

`lettre`: Supports various authentication mechanisms for SMTP.

## Email Clients (IMAP/POP3 - Less Common in Rust)

While some crates exist for IMAP/POP3, they are less mature and widely used than the sending crates. If you need to interact with mailboxes, you might need to search for specific IMAP/POP3 crates or consider using a separate email client library.)\

For most email sending needs, `lettre` is the recommended crate. It provides a comprehensive and easy-to-use API. `mailparse` is essential if you need to parse incoming emails. The `mime` crate is helpful for constructing complex email messages.

</div>
