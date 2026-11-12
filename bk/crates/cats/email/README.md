<!-- TODO Reviewed ??-??-2026 -->

# Email

This crate contains code examples for the **Email** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `async-imap`, `futures`, `html2text`, `lettre`, `mail-parser`, `mailchecker`.

## Examples Covered

- **Imap** (`examples/imap/`): Demonstrates concepts like `imap_fetch`, `mailchecker`.
- **Standalone Examples**: `html2text`, `lettre`, `mailparse`.

## Usage

To check or run examples in this crate:

```console
cargo check --package email --examples
cargo test --package email
```

Code examples correspond to chapters in `bk/src/categories/email` or related book sections.

The ignored SMTP and IMAP service tests use the local GreenMail service from
`.devcontainer/compose.heavy-tests.yaml`. From `bk/`, run
`just code heavy_test mail:lettre` and `just code heavy_test mail:imap`.
GreenMail loads a test-only TLS key and an unseen message fixture; both example
clients trust the matching PEM root certificate through `MAIL_TLS_ROOT_CERT`.
