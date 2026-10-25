<!-- TODO Reviewed ??-??-2026 -->

# Read Emails

{{#include read_emails.incl.md}}

Read a mailbox over IMAP, then parse the messages. IMAP is stateful: connect, log in, select a mailbox, search for message numbers, and fetch their contents.

## Fetch Unread Messages over IMAP {#imap}

[![imap][c~imap~docs~badge]][c~imap~docs]{{hi:imap}} [![mail-parser][c~mail-parser~docs~badge]][c~mail-parser~docs]{{hi:mail-parser}} [![cat~email][cat~email~badge]][cat~email]{{hi:email}}

[`imap`][c~imap~docs]↗{{hi:imap}} is a synchronous IMAP client. The example connects over TLS (port 993) with [`native-tls`][c~native-tls~docs]↗{{hi:native-tls}}, logs in, selects `INBOX`, searches for `UNSEEN` messages, fetches each as `RFC822`, and parses it with [`mail-parser`][c~mail-parser~docs]↗{{hi:mail-parser}}. For asynchronous code, use [`async-imap`][c~async-imap~docs]↗{{hi:async-imap}}.

The example needs a server, so the book does not run it; it reads `IMAP_HOST`, `IMAP_USER`, and `IMAP_PASSWORD` from the environment and reports a clear error when one is missing. The message-processing step, `summarize`, is a pure function with unit tests over a sample message.

```rust,editable,noplayground
{{#include ../../../crates/cats/email/examples/imap/imap_fetch.rs:example}}
```

- **Credentials.** Gmail and Outlook require an app password or OAuth 2.0 rather than your account password. Read credentials from the environment or a secret store, and never log them or the message bodies.
- **Parsing.** See [[email_parsing | Email Parsing]] for [`mailparse`][c~mailparse~docs]↗{{hi:mailparse}}, an alternative parser.
- **Side effects.** Fetching a message with `RFC822` marks it as seen on most servers; use `BODY.PEEK[]` to read without changing flags.

## Related Topics {#related-topics .skip}

- [[send_emails | Sending Emails]].
- [[email_parsing | Email Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[written for issue #1946](https://github.com/john-cd/rust_howto/issues/1946)
</div>
