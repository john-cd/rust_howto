// TODO Reviewed ??-??-2026.
//! Compare explicit command lists with a distributed inventory.

use plugin_registry::Command;
use plugin_registry::registry;

/// Supplies a command handler for explicit registration and duplicate-name tests.
fn local() -> &'static str {
    "local"
}

// ANCHOR: example
/// Validates an explicit registry, then runs commands discovered through inventory.
///
/// Linking the extension makes its registrations available; the registry sorts
/// commands by name so discovery does not expose linker-dependent ordering.
///
/// # Errors
///
/// Returns the duplicate command name if either registry contains a collision.
fn commands() -> Result<Vec<(&'static str, &'static str)>, &'static str> {
    // Explicit registration is easy to inspect and works without constructors.
    let local_command = Command {
        name: "local",
        run: local,
    };
    let explicit = registry([&local_command])?;
    assert_eq!(explicit["local"](), "local");

    // Linking the extension crate makes its inventory::submit! available.
    plugin_extension::linked();
    let discovered = registry(inventory::iter::<Command>)?;
    Ok(discovered
        .into_iter()
        .map(|(name, run)| (name, run()))
        .collect())
}
// ANCHOR_END: example

/// Checks the expected names and outputs of the linked extension commands.
fn main() {
    assert_eq!(
        commands(),
        Ok(vec![("greet", "hello"), ("help", "help text")])
    );
}

/// Checks deterministic discovery and duplicate-name rejection.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that discovers in sorted order.
    #[test]
    fn discovers_in_sorted_order() {
        main();
    }

    /// Checks that rejects duplicate names.
    #[test]
    fn rejects_duplicate_names() {
        let first = Command {
            name: "same",
            run: local,
        };
        let second = Command {
            name: "same",
            run: local,
        };
        assert!(matches!(registry([&first, &second]), Err("same")));
    }
}
