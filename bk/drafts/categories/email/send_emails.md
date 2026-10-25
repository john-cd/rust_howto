<!-- TODO Reviewed ??-??-2026 -->

# Send Emails

{{#include send_emails.incl.md}}

[`lettre`][c~lettre~docs]↗{{hi:lettre}} is a widely used crate for sending emails and is generally the recommended choice. It supports various transports (SMTP, sendmail, etc.). [`mail-send`][c~mail-send~docs]↗{{hi:mail-send}} is another option for sending emails.

## Build Email Messages {#build_email_messages}

[![lettre][c~lettre~docs~badge]][c~lettre~docs]{{hi:lettre}} [![cat~email][cat~email~badge]][cat~email]{{hi:Email}}
[![lettre~crates.io][c~lettre~crates.io~badge]][c~lettre~crates.io]
[![lettre~repo][c~lettre~repo~badge]][c~lettre~repo]
[![lettre~lib.rs][c~lettre~lib.rs~badge]][c~lettre~lib.rs]
[![cat~email][cat~email~badge]][cat~email]{{hi:Email}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

[`lettre`][c~lettre~docs]↗{{hi:lettre}} provides convenient builders.

{{example build_email_messages}}

## Send Emails {#lettre}

[![lettre][c~lettre~docs~badge]][c~lettre~docs]{{hi:lettre}} [![cat~email][cat~email~badge]][cat~email]{{hi:Email}}
[![lettre~crates.io][c~lettre~crates.io~badge]][c~lettre~crates.io]
[![lettre~repo][c~lettre~repo~badge]][c~lettre~repo]
[![lettre~lib.rs][c~lettre~lib.rs~badge]][c~lettre~lib.rs]
[![cat~email][cat~email~badge]][cat~email]{{hi:Email}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

```rust,editable,noplayground
{{#include ../../../crates/cats/email/examples/lettre.rs:example}}
```

<div class="hidden">
[mailchecker recipe written for issue #1946](https://github.com/john-cd/rust_howto/issues/1946)

Message construction is tested offline with reserved example addresses; sending remains an ignored integration test because it needs caller-provided relay credentials and a reachable SMTP server.
</div>

### Send Emails Asynchronously {#send-emails-asynchronously .skip}

[`lettre`][c~lettre~docs]↗{{hi:lettre}} supports asynchronous sending.

### Send Emails Securely {#send-emails-securely .skip}

[`lettre`][c~lettre~docs]↗{{hi:lettre}} integrates with TLS (Transport Layer Security) for secure email transmission.

[`lettre`][c~lettre~docs]↗{{hi:lettre}} also supports various authentication mechanisms for SMTP.

## Filter Invalid and Disposable Addresses {#mailchecker}

[![mailchecker][c~mailchecker~docs~badge]][c~mailchecker~docs]{{hi:mailchecker}} [![cat~email][cat~email~badge]][cat~email]{{hi:email}}

Before queueing a message, drop recipients that cannot or should not receive it. [`mailchecker`][c~mailchecker~docs]↗{{hi:mailchecker}} checks an address's syntax and compares its domain with a list of disposable-address providers, so `user@mailinator.com` is rejected while an ordinary address passes. The check runs offline.

```rust,editable,noplayground
{{#include ../../../crates/cats/email/examples/imap/mailchecker.rs:example}}
```

A syntax-and-domain-list check cannot tell whether a mailbox exists. For syntax alone use an address-parsing crate, and for deliverability rely on bounce handling and a confirmation email.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[send_emails: write](https://github.com/john-cd/rust_howto/issues/342)

- [SendGrid][sendgrid~website]↗.

</div>
