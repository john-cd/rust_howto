<!-- TODO Reviewed ??-??-2026 -->

# Send Emails

{{#include send_emails.incl.md}}

[`lettre`][c~lettre~docs]↗{{hi:lettre}} builds messages and sends them through a configured transport. Its SMTP transport sends to a relay; the relay and its operator handle delivery to recipients. [`mail-send`][c~mail-send~docs]↗{{hi:mail-send}} is another library for sending.

## Build Email Messages {#build_email_messages}

[![lettre][c~lettre~docs~badge]][c~lettre~docs]{{hi:lettre}} [![cat~email][cat~email~badge]][cat~email]{{hi:Email}}
[![lettre~crates.io][c~lettre~crates.io~badge]][c~lettre~crates.io]
[![lettre~repo][c~lettre~repo~badge]][c~lettre~repo]
[![lettre~lib.rs][c~lettre~lib.rs~badge]][c~lettre~lib.rs]
[![cat~email][cat~email~badge]][cat~email]{{hi:Email}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

[`lettre`][c~lettre~docs]↗{{hi:lettre}} provides a typed message builder for headers, addresses, and body content. The complete example below uses a helper to build a message before choosing an SMTP transport.

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

The example requires an SMTP relay and reads `SMTP_SERVER`, `SMTP_USERNAME`, `SMTP_PASSWORD`, `SMTP_FROM`, and `SMTP_TO` from the environment. Its regular test only builds a message with reserved `.invalid` addresses; the ignored integration test sends through the relay and needs credentials and network access. A relay may be hosted by your provider or an email service such as [SendGrid][sendgrid~website].

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

## Add a Plain-Text Alternative with `html2text` {#html2text}

[![html2text][c~html2text~docs~badge]][c~html2text~docs] [![html2text~crates.io][c~html2text~crates.io~badge]][c~html2text~crates.io]{{hi:html2text}} [![cat~email][cat~email~badge]][cat~email]{{hi:Email}}

A multipart email should carry a `text/plain` part next to the HTML one, for clients that cannot show HTML and for spam filters. Instead of writing the text version by hand, generate it from the HTML body with [`html2text`][c~html2text~docs]↗{{hi:html2text}}, then pass both strings to the `lettre` message builder (`MultiPart::alternative_plain_html`). See also [[html_sanitization | Sanitize HTML]].

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/html_text/html2text.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[read_emails | Read Emails]] for retrieving messages from a mailbox.
- [[email_parsing | Email Parsing]] for parsing raw message bytes and MIME parts.
- [Network Programming][p~network-programming] for protocol clients and transports.
- [Transport Layer Security][p~tls] for protecting network connections.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
