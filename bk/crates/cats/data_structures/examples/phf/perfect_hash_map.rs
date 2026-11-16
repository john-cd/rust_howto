// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates a compile-time perfect hash map.

use phf::phf_map;

/// Shared `keywords` used by this module.
static KEYWORDS: phf::Map<&'static str, &'static str> = phf_map! {
    "fn" => "function",
    "let" => "variable binding",
    "mut" => "mutable",
    "struct" => "structure",
    "enum" => "enumeration",
    "impl" => "implementation",
    "trait" => "trait definition",
    "use" => "use declaration",
    "mod" => "module",
    "pub" => "public visibility",
};

/// Looks up keywords and prints the compile-time map's entries.
fn main() {
    example();
}

/// Looks up sample keywords and prints the compile-time map's entries.
fn example() {
    // Lookup is O(1) with no runtime map construction:
    assert_eq!(KEYWORDS.get("fn"), Some(&"function"));
    if let Some(meaning) = KEYWORDS.get("fn") {
        println!("'fn' means: {meaning}");
    }

    println!("Total keywords: {}", KEYWORDS.len());

    // Check for presence of a key:
    assert!(!KEYWORDS.contains_key("for"));
    println!("Is 'for' a keyword? {}", KEYWORDS.contains_key("for"));

    // Iterate over all key-value pairs:
    let mut entries: Vec<_> = KEYWORDS.entries().collect();
    entries.sort_by_key(|(keyword, _)| *keyword);
    for (keyword, description) in entries {
        println!("  {keyword}: {description}");
    }
}
// ANCHOR_END: example

/// Demonstrates a compile-time perfect hash map.
pub fn run() {
    let entry_point = main;
    entry_point();
}

#[cfg(test)]
mod private_main_tests {
    use super::main;

    #[test]
    fn private_main_succeeds() {
        let entry_point = main;
        entry_point();
    }
}
