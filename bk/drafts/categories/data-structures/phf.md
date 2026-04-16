# Perfect Hash Functions

{{#include phf.incl.md}}

A perfect hash function (PHF) maps a set of keys to a set of values with no collisions. When the keys are known ahead of time (at compile time), you can generate a perfect hash function that eliminates all runtime collision overhead, resulting in extremely fast O(1) lookups.

The [`phf`][c~phf~docs]↗{{hi:phf}} crate provides compile-time generated constant hash maps and sets. It uses the build system to generate code at compile time, so the resulting data structures are stored in read-only memory and require no runtime initialization. This makes `phf` ideal for:

- Keyword lookup tables (e.g., in parsers and lexers).
- Static configuration tables.
- Any case where you need to map a fixed set of keys to values with maximum performance.

`phf` provides three data structures: `phf::Map`, `phf::Set`, and `phf::OrderedMap`. They are created using the `phf_map!`, `phf_set!`, and `phf_ordered_map!` macros respectively.

## Use a Compile-time Perfect Hash Map {#phf-map}

[![phf][c~phf~docs~badge]][c~phf~docs] [![phf~crates.io][c~phf~crates.io~badge]][c~phf~crates.io] [![phf~repo][c~phf~repo~badge]][c~phf~repo]{{hi:phf}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The following example shows how to define a compile-time static map using the `phf_map!` macro. The map is generated at compile time and placed in read-only memory, so there is no runtime overhead for building the map:

```rust,editable,noplayground
// Add to Cargo.toml:
// [dependencies]
// phf = { version = "0.11", features = ["macros"] }

use phf::phf_map;

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

fn main() {
    // Lookup is O(1) with zero runtime overhead:
    if let Some(meaning) = KEYWORDS.get("fn") {
        println!("'fn' means: {meaning}");
    }

    println!("Total keywords: {}", KEYWORDS.len());

    // Check for presence of a key:
    println!("Is 'for' a keyword? {}", KEYWORDS.contains_key("for"));

    // Iterate over all key-value pairs:
    let mut entries: Vec<_> = KEYWORDS.entries().collect();
    entries.sort_by_key(|(k, _)| *k);
    for (keyword, description) in entries {
        println!("  {keyword}: {description}");
    }
}
```

## Related Topics {#related-topics .skip}

- [[hashmaps | HashMaps]].
- [[b-trees | B-Trees]].
- [[algorithms | Algorithms]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
