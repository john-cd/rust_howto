<!-- TODO Reviewed ??-??-2026 -->

# Rust Patterns

This crate contains code examples for the **Rust Patterns** category on crates.io. It demonstrates integration with key ecosystem crates including `anyhow`, `clap`, `color-eyre`, `csv`, `dyn-clone`, `either`.

## Examples Covered

- **Behavioral Patterns** (`examples/behavioral_patterns/`): Demonstrates concepts like `undo_redo`.
- **Builder Pattern** (`examples/builder_pattern/`): Demonstrates concepts like `builder_pattern`.
- **Business Rules** (`examples/business_rules/`): Demonstrates concepts like `decision_table`.
- **Creational Patterns** (`examples/creational_patterns/`): Demonstrates concepts like `abstract_factory`, `factory`, `singleton`.
- **Error** (`examples/error/`): Demonstrates concepts like `and_then`, `anyhow`, `backtrace`, `color_eyre`, `error_handling`, `map`, `miette`, `panic`, `question_mark`, `question_mark2`, `result`, `retain`, `thiserror`, `thiserror2`, `unwrap`, `unwrap_or_else`.
- **Functional Programming** (`examples/functional_programming/`): Demonstrates concepts like `either`, `frunk`, `imbl`, `iterators`, `itertools`, `rpds`.
- **Rust Specific Patterns** (`examples/rust_specific_patterns/`): Demonstrates concepts like `dyn_clone`, `newtype_equality`, `newtype_units`, `pin_project`, `validated_newtype`.
- **Structural Patterns** (`examples/structural_patterns/`): Demonstrates concepts like `adapter`, `composite`, `decorator`, `facade`, `newtype`.
- **Standalone Examples**: `scopeguard`.

## Usage

To check or run examples in this crate:

```console
cargo check --package rust_patterns --examples
cargo test --package rust_patterns
```

Code examples correspond to chapters in `bk/src/categories/rust_patterns` or related book sections.
