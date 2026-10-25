//! Read mail over IMAP and filter disposable addresses.
//!
//! TODO Reviewed ??-??-2026.

mod imap_fetch;
mod mailchecker;

fn main() -> anyhow::Result<()> {
    imap_fetch::run()
}
