<!-- TODO Reviewed ??-??-2026 -->

# Enumerations

{{#include enumerations.incl.md}}

## Enum Syntax {#enum}

[![Rust by example - Enums][book~rust-by-example~enums~badge]][book~rust-by-example~enums]{{hi:Enums}}{{hi:Variants}}

Enums are custom data types that define a set of possible 'variants' or states that a value can be. A value of that enum type can only be one of those variants at any given time. Each variant can optionally hold associated data (fields) of different types like a `struct` or like a `tuple`:

```rust,editable
{{#include ../../crates/language/examples/enums/enums.rs:example}}
```

If we make an enum{{hi:Enums}} public, all of its variants are then public. We only need [`pub`][book~rust-reference~visibility-and-privacy]↗{{hi:pub}} before the [`enum`][book~rust-reference~enum]↗ keyword.

## Define Generic Enums {#generic-enums}

Generic enums are declared similarly to generic structs, with type parameters between `<` and `>` after the enum name.
The most common example is the `Result` enum.{{hi:Result}} It can either hold a value of type `T` (in the `Ok` variant) or an error value of type `E` (in the `Err` variant):

```rust,noplayground
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Common `enums` {#common-enums}

See:

- [[option | Option]].
- [[result | Result]].

## Convert between Strings and Enum Variants with `strum` {#strum}

[![strum][c~strum~docs~badge]][c~strum~docs] [![strum~crates.io][c~strum~crates.io~badge]][c~strum~crates.io] [![strum~repo][c~strum~repo~badge]][c~strum~repo] [![strum~lib.rs][c~strum~lib.rs~badge]][c~strum~lib.rs]{{hi:strum}}{{hi:Enums}}{{hi:Macros}}{{hi:Proc-macros}}{{hi:Strings}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}} [![cat~development-tools::procedural-macro-helpers][cat~development-tools::procedural-macro-helpers~badge]][cat~development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

The [`strum`][c~strum~docs]↗{{hi:strum}} crate provides helpful macros for working with enums and strings. It also can convert from an integer to an enum, add custom properties to enum variants, etc.

```rust,editable,noplayground
{{#include ../../crates/language/examples/enums/strum.rs:example}}
```

### Label Variants and Build Menus with `strum` {#strum-messages}

[![strum][c~strum~docs~badge]][c~strum~docs]{{hi:strum}}{{hi:EnumMessage}}{{hi:IntoEnumIterator}}{{hi:EnumCount}}{{hi:VariantNames}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

`#[strum(message = "...", detailed_message = "...")]` with the `EnumMessage` derive attaches a label and a tooltip to each variant. `IntoEnumIterator` iterates over the variants to build a settings menu, `VariantNames` and `EnumCount` provide the list of names and the number of variants, and `AsRefStr` borrows the name as a `&'static str` without allocating, unlike `Display`.

```rust,editable,noplayground
{{#include ../../crates/language/examples/enums/strum_messages.rs:example}}
```

### Attach Constant Properties to Variants with `strum` {#strum-props}

[![strum][c~strum~docs~badge]][c~strum~docs]{{hi:strum}}{{hi:EnumProperty}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

`#[strum(props(icon = "save", shortcut = "Ctrl+S"))]` with the `EnumProperty` derive stores arbitrary string properties on a variant. `get_str("icon")` returns `None` for a missing property.

```rust,editable,noplayground
{{#include ../../crates/language/examples/enums/strum_props.rs:example}}
```

### Parse and Print Config Values with `strum` {#strum-case}

[![strum][c~strum~docs~badge]][c~strum~docs]{{hi:strum}}{{hi:serialize_all}}{{hi:Case-insensitive parsing}} [![cat~parsing][cat~parsing~badge]][cat~parsing]{{hi:Parsing tools}}

`#[strum(serialize_all = "snake_case")]` renames every variant at once, and `ascii_case_insensitive` makes `FromStr` ignore ASCII case. The example round-trips every variant and shows the error for an unknown value.

```rust,editable,noplayground
{{#include ../../crates/language/examples/enums/strum_case.rs:example}}
```

If the same enum is also serialized with [`serde`][c~serde~docs]↗{{hi:serde}}, keep the two consistent: `#[serde(rename_all = "snake_case")]` matches `#[strum(serialize_all = "snake_case")]`. Prefer `serde` alone when the value only ever travels through `serde`; use `strum` for command-line arguments, environment variables and menus.

## Related Topics {#related-topics .skip}

- [[match | Match]].
- [[rust-patterns | Rust Patterns]].
- [[functional_programming | Functional Programming]].
- [[data_types | Data Types]].
- [[structs | Structs]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
