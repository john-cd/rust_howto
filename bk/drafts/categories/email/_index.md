# Email

[![cat-email][cat-email-badge]][cat-email]{{hi:Email}}

Crates to help with sending, receiving, formatting, and parsing email.

For most email sending needs, [`lettre`][c-lettre]⮳{{hi:lettre}} is the recommended crate. It provides a comprehensive and easy-to-use API. [`mailparse`][c-mailparse]⮳{{hi:mailparse}} is essential if you need to parse incoming emails. The [`mime`][c-mime]⮳{{hi:mime}} crate is helpful for constructing complex email messages.

## Send Emails

{{#include send_emails.incl.md}}

## Parse Emails

{{#include email_parsing.incl.md}}

## Email Clients (IMAP/POP3)

While some crates exist for IMAP/POP3, they are less mature and widely used than the sending crates. If you need to interact with mailboxes, you might need to search for specific IMAP/POP3 crates or consider using a separate email client library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[email/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/343)

</div>
