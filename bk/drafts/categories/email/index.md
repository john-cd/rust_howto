<!-- TODO Reviewed ??-??-2026 -->

# Email

[![cat~email][cat~email~badge]][cat~email]{{hi:Email}}

Crates to help with sending, receiving, formatting, and parsing email.

For most email sending needs, [`lettre`][c~lettre~docs]↗{{hi:lettre}} is the recommended crate. It provides a comprehensive and easy-to-use API. [`imap`][c~imap~docs]↗{{hi:imap}} reads a mailbox. [`mailparse`][c~mailparse~docs]↗{{hi:mailparse}} is essential to parse incoming emails. The [`mime`][c~mime~docs]↗{{hi:mime}} crate is helpful for constructing complex email messages.

## Send Emails

{{#include send_emails.incl.md}}

## Parse Emails

{{#include email_parsing.incl.md}}

## Read Emails

{{#include read_emails.incl.md}}

## Related Topics

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [write](https://github.com/john-cd/rust_howto/issues/343)
</div>
