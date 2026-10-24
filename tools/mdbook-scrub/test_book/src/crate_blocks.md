<!-- TODO Reviewed ??-??-2026 -->

# Crate Blocks

The supported form expands a crate and its optional extra categories into badges.

{{#crate crt}}

{{#crate crt cat1 cat-2 cat-2-2 cat3::sub-cat-3}}

{{#categories filesystem=Filesystem,testing="Testing"}}

## Badge Migration Fixture {#badge-migration-fixture}

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Sort a Vector of Integers][ex~algorithms~sort-vector-integers] | {{#crate std}} | {{#categories algorithms=Algorithms}} |

[ex~algorithms~sort-vector-integers]: sorting.md#sort-vector-integers
[c~std~docs]: https://doc.rust-lang.org/std
[c~std~docs~badge]: https://img.shields.io/badge/std-firebrick?logo=rust
[cat~algorithms]: https://crates.io/categories/algorithms
[cat~algorithms~badge]: https://img.shields.io/badge/Algorithms-white
