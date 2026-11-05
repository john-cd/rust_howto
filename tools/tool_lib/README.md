<!-- TODO Reviewed ??-??-2026 -->

# Tools Shared Library

- `src/lib.rs` contains the Rust library to be shared between all Rust CLI tools.

## Examples

Runnable example binaries demonstrating programmatic use of the `tool_lib` API are located in the `examples/` directory:

- `category_badge`: Demonstrates constructing Markdown category badges using `tool_lib::create_category_badge`.
  ```bash
  cargo run --example category_badge
  ```
- `format_crate_section`: Demonstrates formatting crate page sections, category tables, badges, and reference definitions using `tool_lib::create_alphabetical_crate_page_section`, `tool_lib::create_category_and_crates`, and `tool_lib::create_crate_block_badges_or_refdefs`.
  ```bash
  cargo run --example format_crate_section
  ```
- `parse_published_recipes`: Demonstrates scanning an mdBook structure to extract crate-to-recipe links using `tool_lib::collect_published_recipe_links`.
  ```bash
  cargo run --example parse_published_recipes
  ```
