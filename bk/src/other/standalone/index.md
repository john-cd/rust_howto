<!-- TODO Reviewed ??-??-2026. -->

# Standalone Topics

These short guides cover Rust features and design choices that do not belong to one crate ecosystem category. They focus on integer overflow, plugin boundaries, address-sensitive values, alternatives to inheritance, and composable predicates.

## Topics

- [Integer Overflow](overflow.md): choose explicit overflow behavior and understand profile defaults.
- [Inheritance](inheritance.md): model data with enums and composition, and behavior with traits.
- [Plugin Architecture](plugin.md): distinguish in-process extension points from native or WebAssembly plugins.
- [Predicates](predicates.md): compose boolean checks for filtering and assertions.
- [Self-Referential Types](self_referential.md): prefer simpler ownership designs and understand the unsafe work behind pinning.

## Related Topics {#related-topics .skip}

- [Language](../../language/index.md).
- [Standard Library](../../standard-library/index.md).
- [Rust Patterns](../../categories/rust-patterns/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
