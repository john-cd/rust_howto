# Send Emails

{{#include send_emails.incl.md}}

[`lettre`][c-lettre]⮳{{hi:lettre}} is a widely used crate for sending emails and is generally the recommended choice. It supports various transports (SMTP, sendmail, etc.). [`mail-send`][c-mail_send]⮳{{hi:mail-send}} is another option for sending emails.

## Build Email Messages {#build_email_messages}

[![lettre][c-lettre-badge]][c-lettre]{{hi:lettre}} [![cat-email][cat-email-badge]][cat-email]{{hi:Email}}
[![lettre-crates.io][c-lettre-crates.io-badge]][c-lettre-crates.io]
[![lettre-github][c-lettre-github-badge]][c-lettre-github]
[![lettre-lib.rs][c-lettre-lib.rs-badge]][c-lettre-lib.rs]
[![cat-email][cat-email-badge]][cat-email]{{hi:Email}}
[![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

[`lettre`][c-lettre]⮳{{hi:lettre}} provides convenient builders.

{{example build_email_messages}}

## Send Emails {#lettre}

[![lettre][c-lettre-badge]][c-lettre]{{hi:lettre}} [![cat-email][cat-email-badge]][cat-email]{{hi:Email}}
[![lettre-crates.io][c-lettre-crates.io-badge]][c-lettre-crates.io]
[![lettre-github][c-lettre-github-badge]][c-lettre-github]
[![lettre-lib.rs][c-lettre-lib.rs-badge]][c-lettre-lib.rs]
[![cat-email][cat-email-badge]][cat-email]{{hi:Email}}
[![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

```rust,editable,noplayground
{{#include ../../../crates/cats/email/examples/lettre.rs:example}}
```

### Send Emails Asynchronously {#skip}

[`lettre`][c-lettre]⮳{{hi:lettre}} supports asynchronous sending.

### Send Emails Securely {#skip}

[`lettre`][c-lettre]⮳{{hi:lettre}} integrates with TLS (Transport Layer Security) for secure email transmission.

[`lettre`][c-lettre]⮳{{hi:lettre}} also supports various authentication mechanisms for SMTP.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[send_emails: write](https://github.com/john-cd/rust_howto/issues/342)
</div>
