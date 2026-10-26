<!-- TODO Reviewed ??-??-2026 -->

# Email Parsing

An Internet email message has headers, a blank line, and a body. [RFC 5322][rfc~5322] defines this message format. [MIME][rfc~2045] adds media types, character sets, and content-transfer encodings; multipart messages use boundaries to hold several parts, which may themselves contain nested parts ([RFC 2046][rfc~2046]).

## Parse Messages with `mailparse` {#mailparse}

[![mailparse~website][c~mailparse~website~badge]][c~mailparse~website] [![mailparse][c~mailparse~docs~badge]][c~mailparse~docs] [![mailparse~crates.io][c~mailparse~crates.io~badge]][c~mailparse~crates.io] [![mailparse~repo][c~mailparse~repo~badge]][c~mailparse~repo] [![mailparse~lib.rs][c~mailparse~lib.rs~badge]][c~mailparse~lib.rs]{{hi:mailparse}}{{hi:Email}}{{hi:Maildir}}{{hi:Mime}}{{hi:Parser}}{{hi:Rfc822}} [![cat~email][cat~email~badge]][cat~email]{{hi:Email}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

[`mailparse`][c~mailparse~docs]↗{{hi:mailparse}} parses raw message bytes into a `ParsedMail` tree. The parsed value borrows the input bytes, so keep the input alive while reading headers and bodies. It does not connect to a mailbox or establish who sent the message.

{{#example mailparse}}

The example reads an optional `Subject` header and walks the MIME tree to find a `text/plain` leaf. `get_body()` decodes the part's transfer encoding and character set into a Rust `String`. For binary attachments, work with the decoded bytes instead of treating them as text.

## MIME (Multipurpose Internet Mail Extensions) {#mime}

[![mime][c~mime~docs~badge]][c~mime~docs] [![mime~crates.io][c~mime~crates.io~badge]][c~mime~crates.io] [![mime~repo][c~mime~repo~badge]][c~mime~repo] [![mime~lib.rs][c~mime~lib.rs~badge]][c~mime~lib.rs]{{hi:mime}}{{hi:mime}}{{hi:Media-extensions}}{{hi:Media-types}}

The [`mime`][c~mime~docs]↗ crate represents and compares media types such as `text/plain` and `image/png`. It does not parse an email message or walk its MIME parts; use a message parser such as `mailparse` for that job.

## Handle Parsed Content Safely {#handle-parsed-content-safely .skip}

Treat headers, HTML, filenames, and attachments as untrusted input. Set message-size and part-count limits in the surrounding application, sanitize HTML before rendering it, and validate attachment names before saving files. Parsing a message does not authenticate its sender.

## Related Topics {#related-topics .skip}

- [[email | Email]].
- [[parsing | Parsing]].
- [[encoding | Encoding]].
- [[web-programming | Web Programming]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  write
</div>
