//! Compare explicit command lists with a distributed inventory.

use plugin_registry::Command;
use plugin_registry::registry;

fn local() -> &'static str {
    "local"
}

// ANCHOR: example
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

fn main() {
    assert_eq!(
        commands(),
        Ok(vec![("greet", "hello"), ("help", "help text")])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_in_sorted_order() {
        main();
    }

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
