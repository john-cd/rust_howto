# Builder derive crates

{{#include builder_pattern.incl.md}}

## `typed-builder` {#typed-builder}

[![typed-builder][c-typed_builder-badge]][c-typed_builder] [![typed-builder-crates.io][c-typed_builder-crates.io-badge]][c-typed_builder-crates.io] [![typed-builder-github][c-typed_builder-github-badge]][c-typed_builder-github] [![typed-builder-lib.rs][c-typed_builder-lib.rs-badge]][c-typed_builder-lib.rs]{{hi:typed-builder}}{{hi:Builder}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

`typed-builder` lets you derive compile-time type-checked builders. The oldest crate for compile-time-checked builders that has (987K downloads/month, 916 stars, 7 years old)

## `derive_builder` {#derive_builder}

[![derive_builder][c-derive_builder-badge]][c-derive_builder] [![derive_builder-crates.io][c-derive_builder-crates.io-badge]][c-derive_builder-crates.io] [![derive_builder-github][c-derive_builder-github-badge]][c-derive_builder-github] [![derive_builder-lib.rs][c-derive_builder-lib.rs-badge]][c-derive_builder-lib.rs]{{hi:derive_builder}}{{hi:Builder}}{{hi:Derive}}{{hi:Macro}}{{hi:Setter}}{{hi:Struct}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

`derive_builder` provides a Rust macro to automatically implement the builder pattern for arbitrary structs.

runtime-checked builders, works with &self, &mut self builder patterns. The oldest crate for runtime-checked builders overall (1,58M downloads/month, 1285 stars, 8 years old)

## `bon` {#bon}

[![bon-website][c-bon-website-badge]][c-bon-website] [![bon][c-bon-badge]][c-bon] [![bon-crates.io][c-bon-crates.io-badge]][c-bon-crates.io] [![bon-github][c-bon-github-badge]][c-bon-github] [![bon-lib.rs][c-bon-lib.rs-badge]][c-bon-lib.rs]{{hi:bon}}{{hi:Builder}}{{hi:Constructor}}{{hi:Derive}}{{hi:Macro}}{{hi:Setter}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

`bon` is a next-gen compile-time-checked builder generator, named function arguments.

bon - compile-time-checked builders, named function arguments via builders (foo().arg(...).call()), fallible/async builders, method builders (self.foo(...).arg(...).call()). The newest crate built based on lessons learned from typed-builder and derive_builder (33K downloads/month, but gaining popularity, 1095 stars, 3 months old).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write and add to index and SUMMARY](https://github.com/john-cd/rust_howto/issues/648)
</div>
