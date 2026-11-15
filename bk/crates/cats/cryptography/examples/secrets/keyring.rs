// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Stores, loads and deletes a secret in the operating-system credential store
//! (Windows Credential Manager, macOS Keychain or the Secret Service) with
//! `keyring`.

use ::keyring::Entry;
use ::keyring::Error;

/// Service name under which the application files its credentials.
const SERVICE: &str = "rust-howto-example";

/// A thin wrapper over one credential store entry.
struct Vault {

    /// The entry in the credential store.
    entry: Entry,
}

impl Vault {

    /// Opens the entry for `user`.
    fn new(user: &str) -> Result<Self, Error> {
        Ok(Self {
            entry: Entry::new(SERVICE, user)?,
        })
    }

    /// Stores (or overwrites) the secret.
    fn store(&self, secret: &[u8]) -> Result<(), Error> {
        self.entry.set_secret(secret)
    }

    /// Loads the secret, or `None` if nothing is stored.
    fn load(&self) -> Result<Option<Vec<u8>>, Error> {
        match self.entry.get_secret() {
            Ok(bytes) => Ok(Some(bytes)),
            Err(Error::NoEntry) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Deletes the secret. Deleting a missing secret is not an error.
    fn delete(&self) -> Result<(), Error> {
        match self.entry.delete_credential() {
            Ok(()) | Err(Error::NoEntry) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Creates a secret vault and demonstrates storing, loading, and deleting a secret.
///
/// # Errors
///
/// Returns an error if creating the vault or storing, loading, or deleting the secret fails.
fn main() -> Result<(), Error> {
    let vault = Vault::new("alice")?;
    vault.store(b"s3cret")?;
    assert_eq!(vault.load()?.as_deref(), Some(b"s3cret".as_slice()));
    vault.delete()?;
    assert_eq!(vault.load()?, None);
    Ok(())
}
// ANCHOR_END: example

/// Returns an error if creating the vault or storing, loading, or deleting the secret fails.
pub fn run() {
    if let Err(e) = main() {
        eprintln!("credential store unavailable: {e}");
    }
}

/// Checks that an entry can be created when a store is available.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that an entry can be created when a store is available.
    #[test]
    #[ignore = "requires an OS credential store"]
    fn entry_can_be_opened() -> Result<(), Error> {
        let _vault = Vault::new("unit-test")?;
        Ok(())
    }

    /// Stores and retrieves a credential through the OS keyring.
    #[test]
    #[ignore = "needs an OS credential store"]
    fn os_credential_store_round_trips() -> Result<(), Error> {
        main()
    }
}
