// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Snapshots of structured data with `insta`: JSON, redactions and
//! `Debug` output.

use serde::Serialize;

/// An order as returned by an API.
#[derive(Debug, Serialize)]
struct Order {
    /// Random: differs on every run.
    id: u64,
    customer: String,
    items: Vec<String>,
}

/// Builds an order with deterministic fields and a caller-provided id.
fn create_order(id: u64) -> Order {
    Order {
        id,
        customer: "Ferris".to_owned(),
        items: vec!["crab".to_owned(), "shell".to_owned()],
    }
}

/// Prints one representative order for the example binary.
fn main() {
    println!("{:?}", create_order(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Redacts the generated order id from the JSON snapshot.
    #[test]
    fn json_snapshot_with_redaction() {
        // Redactions replace values that change between runs, so the
        // snapshot stays stable.
        insta::assert_json_snapshot!(create_order(4_242), {
            ".id" => "[id]",
        }, @r#"
        {
          "id": "[id]",
          "customer": "Ferris",
          "items": [
            "crab",
            "shell"
          ]
        }
        "#);
    }

    /// Captures the stable debug representation of an order.
    #[test]
    fn debug_snapshot() {
        insta::assert_debug_snapshot!(create_order(7), @r#"
        Order {
            id: 7,
            customer: "Ferris",
            items: [
                "crab",
                "shell",
            ],
        }
        "#);
    }
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
#[allow(clippy::items_after_test_module)]
pub fn run() {
    main();
}
