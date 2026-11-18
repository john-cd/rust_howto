<!-- TODO Reviewed ??-??-2026. -->

# Database Implementations

This package contains examples for embedded databases and search engines. The database examples use `redb`, `sled`, and SurrealDB 4 with its in-memory engine. The search examples use an in-memory `tantivy` index and the Meilisearch Rust client.

## Examples Covered

- **Databases** (`examples/databases/`): Demonstrates `redb`, `sled`, and SurrealDB.
- **Search Engines** (`examples/search_engines/`): Demonstrates an in-memory `tantivy` index and the Meilisearch SDK. The Meilisearch example requires a running service.

## Usage

From `bk/crates`, check and test all example features with:

```console
cargo check --locked --package database_implementations --examples --all-features
cargo test --locked --package database_implementations --all-targets --all-features
```

The Meilisearch integration test is ignored by default because it requires a running service. Set `MEILISEARCH_URL` and, when needed, `MEILI_MASTER_KEY` to configure it. Code examples correspond to chapters in `bk/src/categories/database-implementations` or related book sections.
