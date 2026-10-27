<!-- TODO Reviewed ??-??-2026 -->

# Email

[![cat~email][cat~email~badge]][cat~email]{{hi:Email}}

Crates to help with sending, receiving, formatting, and parsing email.

Sending, mailbox access, and message parsing are separate tasks. [`lettre`][c~lettre~docs]↗{{hi:lettre}} builds messages and sends them through a configured transport such as an SMTP relay. [`imap`][c~imap~docs]↗{{hi:imap}} connects to an existing mailbox to search and fetch messages. Parse the returned RFC 5322/MIME bytes with a message parser; the [`mime`][c~mime~docs]↗{{hi:mime}} crate represents media types but does not parse complete messages.

## Send Emails

[![lettre][c~lettre~docs~badge]][c~lettre~docs]{{hi:lettre}} [![mail-send][c~mail-send~docs~badge]][c~mail-send~docs]{{hi:mail-send}}

See [Send Emails](send_emails.md) to build plain-text or multipart messages and send them through a relay. The chapter uses [`lettre`][c~lettre~docs]↗{{hi:lettre}} as its primary example and notes [`mail-send`][c~mail-send~docs]↗{{hi:mail-send}} as another library.

{{#include send_emails.incl.md}}

## Parse Emails

[![mailparse][c~mailparse~docs~badge]][c~mailparse~docs]{{hi:mailparse}} [![mime][c~mime~docs~badge]][c~mime~docs]{{hi:mime}}

See [Email Parsing](email_parsing.md) when you already have raw message bytes. Its `mailparse` example walks headers and MIME parts; it does not connect to a mailbox or authenticate the sender.

{{#include email_parsing.incl.md}}

## Read Emails

[![imap][c~imap~docs~badge]][c~imap~docs]{{hi:imap}} [![mail-parser][c~mail-parser~docs~badge]][c~mail-parser~docs]{{hi:mail-parser}}

See [Read Emails](read_emails.md) to fetch messages from a mailbox over IMAP, then parse the fetched content. The example needs a reachable server and caller-provided credentials, so it is not run as part of the book.

{{#include read_emails.incl.md}}

## Related Topics

- [Network Programming][p~network-programming] covers transport and protocol clients.
- [Encoding and Serialization][p~encoding] covers character sets and serialized data.
- [Web Programming][p~web-programming] covers related HTML and web content handling.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
