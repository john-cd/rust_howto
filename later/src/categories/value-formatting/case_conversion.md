<!-- TODO Reviewed ??-??-2026. -->

# Identifier Case Conversion

Use this recipe when translating an identifier or field name between naming conventions. Case conversion does not validate an identifier for the target language; check the result against that language's naming and reserved-word rules.

## Convert Identifiers Between Case Styles {#case-conversion}

[`heck`][c~heck~docs]↗{{hi:heck}} converts identifiers between common case conventions.

```rust,editable
{{#include ../../../crates/cats/value_formatting/examples/case_conversion/case_conversion.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Value Formatting](index.md) and [Number Formatting](number_formatting.md).
- [[internationalization | Internationalization]] and [[localization | Localization]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
