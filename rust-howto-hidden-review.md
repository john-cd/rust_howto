<!-- TODO Reviewed ??-??-2026. -->
# Rust How-To hidden notes audit

The initial scan found 282 blocks. GitHub confirmed all 39 issues linked by explicit issue URLs were closed. The cleanup removed 52 link occurrences.

## Proposed follow-up

1. Start with concrete chapter gaps. Reconcile the HTTP client index and its recipe table, then compare the Actix, Axum, GraphQL, and Hyper notes against their existing examples. Give each remaining gap a specific deliverable.
2. Compare the content audit notes with present chapters before restoring material. Prioritize distinct examples or missing explanations over reinstating long crate lists.
3. Map the requested architecture patterns to existing examples. Link Crux to architecture and add a cross-platform explanation of conditional compilation only where the current chapter lacks it.
4. Track broad accuracy reviews per chapter or category with explicit acceptance criteria. Keep useful review evidence and references in the meantime.
5. Promote hidden links only after checking that they serve a reader-facing purpose and adding context. Keep historical comparisons separate from current recipes.

## Inventory after edits

Groups are based on note wording. Mixed blocks appear in one group with their full text.

### Chapter improvement notes (49)

- [bk/drafts/categories/asynchronous/async_channels.md:98](D:/code/rust_howto/bk/drafts/categories/asynchronous/async_channels.md:98)

  async_channels: review add other [`postage`][c~postage~docs]↗{{hi:postage}} channels?

- [bk/drafts/categories/asynchronous/async_traits.md:84](D:/code/rust_howto/bk/drafts/categories/asynchronous/async_traits.md:84)

  async-traits: review new Rust features
  async-trait = Provides a workaround for the lack of [language][p~language] support for [async][p~async] [functions][p~functions] in traits.

- [bk/drafts/categories/asynchronous/tokio.md:77](D:/code/rust_howto/bk/drafts/categories/asynchronous/tokio.md:77)

  tokio: review

- [bk/drafts/categories/authentication/basic_authentication.md:33](D:/code/rust_howto/bk/drafts/categories/authentication/basic_authentication.md:33)

  expand

- [bk/drafts/categories/caching/in_memory_cache.md:59](D:/code/rust_howto/bk/drafts/categories/caching/in_memory_cache.md:59)

  expand; review in depth

- [bk/drafts/categories/command-line-interface/argument_parsing.md:122](D:/code/rust_howto/bk/drafts/categories/command-line-interface/argument_parsing.md:122)

  arguments: review

  [Parsing arguments in Rust with no dependencies][blog~parsing-arguments-rust-no-dep]↗.

- [bk/drafts/categories/command-line-interface/tui.md:57](D:/code/rust_howto/bk/drafts/categories/command-line-interface/tui.md:57)

  tui: expand

- [bk/drafts/categories/command-line-interface/user_interaction.md:49](D:/code/rust_howto/bk/drafts/categories/command-line-interface/user_interaction.md:49)

  user_interaction: write

- [bk/drafts/categories/concurrency/atomics.md:60](D:/code/rust_howto/bk/drafts/categories/concurrency/atomics.md:60)

  fix

- [bk/drafts/categories/concurrency/loom.md:44](D:/code/rust_howto/bk/drafts/categories/concurrency/loom.md:44)

  fix

- [bk/drafts/categories/concurrency/send_sync.md:76](D:/code/rust_howto/bk/drafts/categories/concurrency/send_sync.md:76)

  write; add links?

  - [Implementing `Vec`][book~rustonomicon~implementing-vec]↗.
  - [An unsafe tour of Rust's Send and Sync][blog~an-unsafe-tour-of-rust-s-send-and-sync]↗.
  - [Rust: A unique perspective][blog~rust-a-unique-perspective]↗.

  REFERENCES:

  - [References][primitive~reference]↗.

  POINTERS:

  - [Pointers][primitive~pointer]↗.
  - [Pointer functions][c~std::ptr~docs]↗.
  - [NonNull pointer][c~std::ptr::NonNull~docs]↗.
  - [std::fmt::Pointer][c~std::fmt::Pointer~docs]↗.

  SEND / SYNC:

  - [The Rustonomicon: `Send` and `Sync`][book~rustonomicon~send-and-sync]↗.

  PHANTOM DATA:

  - [phantom-data][book~rustonomicon~phantom-data]↗.
  - [PhantomData][c~std::marker::PhantomData~docs]↗.
  - [Understanding the `Send` trait][stackoverflow~understanding-the-send-trait]↗.

- [bk/drafts/categories/data-structures/index.md:192](D:/code/rust_howto/bk/drafts/categories/data-structures/index.md:192)

  data-structures: expand

- [bk/drafts/categories/date-and-time/parse.md:75](D:/code/rust_howto/bk/drafts/categories/date-and-time/parse.md:75)

  write humantime; review

  The short example focuses on duration parsing and formatting so it complements the timestamp overview without duplicating more sample code in the category index.

- [bk/drafts/categories/development-tools/installation/install.md:53](D:/code/rust_howto/bk/drafts/categories/development-tools/installation/install.md:53)

  review

- [bk/drafts/categories/development-tools_debugging/debugger.md:96](D:/code/rust_howto/bk/drafts/categories/development-tools_debugging/debugger.md:96)

  add Debugger page

- [bk/drafts/categories/development-tools_profiling/index.md:79](D:/code/rust_howto/bk/drafts/categories/development-tools_profiling/index.md:79)

  expand / review

- [bk/drafts/categories/development-tools_testing/index.md:60](D:/code/rust_howto/bk/drafts/categories/development-tools_testing/index.md:60)

  expand

  - [trybuild][c~trybuild~repo]↗: Test harness for ui tests of compiler diagnostics.

- [bk/drafts/categories/development-tools_testing/mocking.md:59](D:/code/rust_howto/bk/drafts/categories/development-tools_testing/mocking.md:59)

  testing: write

- [bk/drafts/categories/development-tools_testing/property_based_testing.md:51](D:/code/rust_howto/bk/drafts/categories/development-tools_testing/property_based_testing.md:51)

  cover property [testing][p~testing] [proptest][c~proptest~repo]

- [bk/drafts/categories/development-tools_testing/test_runners.md:38](D:/code/rust_howto/bk/drafts/categories/development-tools_testing/test_runners.md:38)

  testing: write

- [bk/drafts/categories/development-tools_testing/testing.md:131](D:/code/rust_howto/bk/drafts/categories/development-tools_testing/testing.md:131)

  testing: write

  - [serial_test][c~serial_test~docs]↗ allows for the creation of serialized Rust tests using the serial attribute.

- [bk/drafts/categories/hardware-support/index.md:72](D:/code/rust_howto/bk/drafts/categories/hardware-support/index.md:72)

  expand hardware-support, write missing sections

  - [Multiversion][c~multiversion~lib.rs]↗ - Rust HW library.

- [bk/drafts/categories/hardware-support/processor.md:117](D:/code/rust_howto/bk/drafts/categories/hardware-support/processor.md:117)

  processor: expand

  - [Nine Rules for SIMD Acceleration of Your Rust Code (Part 1)][blog~towardsdatascience-nine-rules-for-simd-acceleration-of-your-rust-code-part-1]↗.
  - [Towards fearless SIMD, 7 years later][blog~towards-fearless-simd]↗ - Linebender.

- [bk/drafts/categories/memory-management/index.md:61](D:/code/rust_howto/bk/drafts/categories/memory-management/index.md:61)

  memory-management/index: organize; align table and sections; write missing sections; cross link

- [bk/drafts/categories/network-programming/server.md:48](D:/code/rust_howto/bk/drafts/categories/network-programming/server.md:48)

  server: write; add cross-links: async, web, `http` server

- [bk/drafts/categories/os/index.md:64](D:/code/rust_howto/bk/drafts/categories/os/index.md:64)

  os/index: fix; organize; add cross links

- [bk/drafts/categories/rust-patterns/error_handling/error_handling.md:165](D:/code/rust_howto/bk/drafts/categories/rust-patterns/error_handling/error_handling.md:165)

  error_handling: fix / organize
  review [![Rust by example - error][book~rust-by-example~error~badge]][book~rust-by-example~error]

  [`std::io::Error`][c~std::io::Error~docs]↗{{hi:std::io::Error}} defined type implementing the [`std::error::Error`][c~std::error::Error~docs]↗{{hi:std::error::Error}} trait.

  - [`human-panic`][c~human-panic~repo]↗: Panic messages for humans.

- [bk/drafts/categories/rust-patterns/functional_programming.md:88](D:/code/rust_howto/bk/drafts/categories/rust-patterns/functional_programming.md:88)

  functional_programming: organize / align intro and sections / add examples
  review
  - [rust-type-level-programming][blog~rust-type-level-programming]↗.
  - [Type-Level Programming in Rust][blog~type-level-programming-in-rust]↗.

  review [tap][c~tap~crates.io]↗.

  - [Type-level Programming in Rust][blog~willcrichton-type-level-programming]↗.

- [bk/drafts/categories/text-processing/diffing.md:40](D:/code/rust_howto/bk/drafts/categories/text-processing/diffing.md:40)

  write; titles

- [bk/drafts/categories/text-processing/other_strings.md:67](D:/code/rust_howto/bk/drafts/categories/text-processing/other_strings.md:67)

  review

- [bk/drafts/categories/text-processing/unicode.md:33](D:/code/rust_howto/bk/drafts/categories/text-processing/unicode.md:33)

  review

  - [unicode_width][unicode-width~website]↗: Determine displayed width of [`char`][primitive~char]↗{{hi:char}} and [`str`][primitive~str]↗{{hi:str}} types according to Unicode Standard Annex #11 rules.

- [bk/drafts/categories/web-programming/index.md:70](D:/code/rust_howto/bk/drafts/categories/web-programming/index.md:70)

  web-programming/index: organize; need full review; further cross link

  Frameworks:

  - [Which Rust web framework to choose in 2022 (with code examples)][blog~kerkour-rust-web-framework-2022]↗.

  QUIC:

  - [quiche][c~quiche~repo]↗: Savory implementation of the QUIC transport protocol and HTTP/3.
  - [Quinn - Rust network library][c~quinn~lib.rs]↗.
  - [msquic][c~msquic~repo]↗: Cross-platform, C implementation of the IETF QUIC protocol, exposed to C, C++, C# and Rust..
  - [Mockito - Rust web dev library][c~mockito~lib.rs]↗.

  HTTP:

  - [H2 - Rust web dev library][c~h2~lib.rs]↗.

  OTHER:

  - [webbrowser - Rust web dev library][c~webbrowser~lib.rs]↗ open URLs and local files in the web browsers available on a platform.

  - [webpki-root-certs - Rust crypto library][c~webpki-root-certs~lib.rs]↗.

  - [Sycamore][sycamore~website]↗.

  - [sycamore][c~sycamore~repo]↗: A library for creating reactive web apps in Rust and WebAssembly.

- [bk/drafts/categories/web-programming_http-client/http_clients.md:65](D:/code/rust_howto/bk/drafts/categories/web-programming_http-client/http_clients.md:65)

  http_clients: expand; link to hyper.md in server.

- [bk/drafts/categories/web-programming_http-client/index.md:60](D:/code/rust_howto/bk/drafts/categories/web-programming_http-client/index.md:60)

  web-programming_http-client/index: fix, sync table and contents

  - [rquest - Rust HTTP client][c~rquest~lib.rs]↗.

- [bk/drafts/categories/web-programming_http-server/_actix.md:35](D:/code/rust_howto/bk/drafts/categories/web-programming_http-server/_actix.md:35)

  actix: organize / write

- [bk/drafts/categories/web-programming_http-server/_axum.md:35](D:/code/rust_howto/bk/drafts/categories/web-programming_http-server/_axum.md:35)

  axum: write

  [Rust Axum clean demo][clean_axum_demo~repo]↗ is a one-stop, production-ready API template.

  **Rust Axum Clean Demo** is a GitHub template that brings together almost all the best practices and features for building a production-ready API server with [Axum][c~axum~repo]↗ and [SQLx][c~sqlx~repo]↗.

  While Axum's official examples do a fantastic job of demonstrating individual features (routing, extractors, middleware, etc.), I found it really helpful to have everything wired up in **one** cohesive structure:

  - **Domain-first modularity**: each domain (user, auth, device, file...) lives in its own module, with controllers, DB layer, and models neatly organized
  - **Clean Architecture** & dependency inversion via traits,
  - **SQLx** for compile-time checked queries + offline mode setup,
  - **JWT-based auth** (login endpoint + `Extension<Claims>`),
  - **File upload & protected file serving** with multipart extractors,
  - **Swagger UI docs** powered by `utoipa-swagger-ui` (Authorize, try out endpoints in-browser),
  - **Database seeding** scripts to spin up your schema & seed data,
  - **Unit & integration tests** out of the box,
  - Example on how to [implement OpenTelemetry for Axum][implement-OpenTelemetry-for-Axum~repo]↗.
  - Database migrations: review [axum-rest-api-sample][axum-rest-api-sample~repo]↗.

  Other Axum template:

  - [keterrest][keterrest~repo]↗.

  Other Generators:

  - [protypo~repo]↗.
  - [gerust~website]↗.

- [bk/drafts/categories/web-programming_http-server/_graphql.md:46](D:/code/rust_howto/bk/drafts/categories/web-programming_http-server/_graphql.md:46)

  graphql: write integrate in seography graphql section?

- [bk/drafts/categories/web-programming_http-server/_grpc.md:90](D:/code/rust_howto/bk/drafts/categories/web-programming_http-server/_grpc.md:90)

  grpc: write

- [bk/drafts/categories/web-programming_http-server/_hyper.md:54](D:/code/rust_howto/bk/drafts/categories/web-programming_http-server/_hyper.md:54)

  hyper: fix

- [later/src/categories/game-engines/index.md:36](D:/code/rust_howto/later/src/categories/game-engines/index.md:36)

  expand
  [pyxel][pyxel~repo]↗: a retro game engine for Python (not Rust; listed for comparison).

- [later/src/categories/games/index.md:33](D:/code/rust_howto/later/src/categories/games/index.md:33)

  write; need in depth review; review [are-we-game-yet? games][are-we-game-yet?-games~website]

  ```rust,editable
  {{#include ../../../crates/cats/games/examples/games.rs:example}}
  ```

- [later/src/categories/gui/index.md:113](D:/code/rust_howto/later/src/categories/gui/index.md:113)

  organize
  [Are we (I)DE yet?][are-we-ide-yet?~website]↗.
  [Porting a cross-platform GUI application to Rust][blog~porting-a-cross-platform-gui-application-to-rust]↗.
  cover zola?

- [later/src/categories/internationalization/index.md:53](D:/code/rust_howto/later/src/categories/internationalization/index.md:53)

  Review `whoami`

- [later/src/categories/rendering/svg_rendering.md:37](D:/code/rust_howto/later/src/categories/rendering/svg_rendering.md:37)

  write.
  Decide what to cover.

- [later/src/other/architecture/architectural_patterns.md:50](D:/code/rust_howto/later/src/other/architecture/architectural_patterns.md:50)

  write. create missing examples. cover [`inventory`][c~inventory~docs]↗{{hi:inventory}} crate for DI?

- [later/src/other/cloud/_fragment.md:22](D:/code/rust_howto/later/src/other/cloud/_fragment.md:22)

  decide where that goes
  | [Use the `dapr` distributed runtime][ex~cloud~dapr]↗ | [![dapr-rust-sdk~repo][dapr-rust-sdk~repo~badge]][dapr-rust-sdk~repo] |

- [later/src/other/cross-platform/crux.md:46](D:/code/rust_howto/later/src/other/cross-platform/crux.md:46)

  crux: add / edit; link to architecture pages

- [later/src/other/cross-platform/index.md:94](D:/code/rust_howto/later/src/other/cross-platform/index.md:94)

  add / write; link to other chapters; conditional compile

- [later/src/other/data-processing/index.md:72](D:/code/rust_howto/later/src/other/data-processing/index.md:72)

  organize / edit

### Content and scope audits (13)

- [bk/src/categories/algorithms/hashing.md:143](D:/code/rust_howto/bk/src/categories/algorithms/hashing.md:143)

  accuracy review: algorithms
  content audit: crate pointers (AEAD, signatures, certificates, ahash) removed in 68a11801; restore or keep a short list

- [later/src/categories/compilers/index.md:51](D:/code/rust_howto/later/src/categories/compilers/index.md:51)

  content audit: category index content removed by recent commits; review and restore

- [later/src/categories/multimedia_audio/index.md:43](D:/code/rust_howto/later/src/categories/multimedia_audio/index.md:43)

  content audit: category index content removed by recent commits; review and restore

- [later/src/categories/no-std_no-alloc/index.md:39](D:/code/rust_howto/later/src/categories/no-std_no-alloc/index.md:39)

  content audit: category index content removed by recent commits; review and restore

- [later/src/categories/os_freebsd-apis/index.md:33](D:/code/rust_howto/later/src/categories/os_freebsd-apis/index.md:33)

  content audit: category index content removed by recent commits; review and restore

  - Crates.io categories: [API Bindings][cat~api-bindings]↗ and [External FFI Bindings][cat~external-ffi-bindings]↗.

- [later/src/categories/os_linux-apis/index.md:36](D:/code/rust_howto/later/src/categories/os_linux-apis/index.md:36)

  content audit: category index content removed by recent commits; review and restore

- [later/src/categories/os_macos-apis/index.md:49](D:/code/rust_howto/later/src/categories/os_macos-apis/index.md:49)

  content audit: category index content removed by recent commits; review and restore

  - Historical research notes: interface with Cocoa using Objective-C bindings where needed, and see the [Swift bindings project][swift~website]↗ for Swift interoperability options. Prefer the maintained `objc2-*` crates described above for new AppKit work.

- [later/src/categories/science_geo/index.md:36](D:/code/rust_howto/later/src/categories/science_geo/index.md:36)

  content audit: category index content removed by recent commits; review and restore

- [later/src/categories/value-formatting/index.md:36](D:/code/rust_howto/later/src/categories/value-formatting/index.md:36)

  content audit: category index content removed by recent commits; review and restore

- [later/src/other/scripting/index.md:57](D:/code/rust_howto/later/src/other/scripting/index.md:57)

  content audit: category index content removed by recent commits; review and restore

  - Additional script tools: [`rust-script`][c~rust-script~website]↗, [`cargo-scripter`][c~cargo-scripter~docs]↗, [`shell2batch`][c~shell2batch~docs]↗, and the [Pre-RFC: `cargo-script` for everyone][forum~pre-rfc-cargo-script-for-everyone]↗. Roto project links: [![website][c~roto~website~badge]][c~roto~website]↗ [![docs][c~roto~docs~badge]][c~roto~docs]↗ [![crates.io][c~roto~crates.io~badge]][c~roto~crates.io]↗ [![repository][c~roto~repo~badge]][c~roto~repo]↗ [![lib.rs][c~roto~lib.rs~badge]][c~roto~lib.rs]↗ and an [introduction blog post][blog~roto]↗.
  - Curated overview: [Awesome WebAssembly Languages][awesome-wasm-langs-typescript~repo]↗.
  - `rlua` was superseded by `mlua`; it is intentionally omitted from the current Lua recommendation above.

- [later/src/other/written-in-rust/development_tools.md:112](D:/code/rust_howto/later/src/other/written-in-rust/development_tools.md:112)

  content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries

  - The [Awesome Rust Tools][awesome-rust-tools~repo]↗ and [Awesome Alternatives in Rust][awesome-alternatives-in-rust~repo]↗ lists are broader catalogs, not endorsements or maintenance guarantees.

- [later/src/other/written-in-rust/index.md:67](D:/code/rust_howto/later/src/other/written-in-rust/index.md:67)

  content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries

- [later/src/other/written-in-rust/python_tools.md:63](D:/code/rust_howto/later/src/other/written-in-rust/python_tools.md:63)

  python_tools: write
  content audit: Written-in-Rust tool lists trimmed by recent commits; review and restore removed entries

  - [`uv` documentation][uv~docs~website]↗.
  - [`pyapp`][pyapp~website]↗ is retained as a Python application packaging resource; verify its current status before recommending it.

### Accuracy review labels (92)

- [bk/src/appendices/contributing/api_documentation.md:16](D:/code/rust_howto/bk/src/appendices/contributing/api_documentation.md:16)

  accuracy review: contributing guide

- [bk/src/appendices/contributing/book_editing_and_example_code_development.md:52](D:/code/rust_howto/bk/src/appendices/contributing/book_editing_and_example_code_development.md:52)

  accuracy review: contributing guide

- [bk/src/appendices/contributing/dev_container_and_docker.md:116](D:/code/rust_howto/bk/src/appendices/contributing/dev_container_and_docker.md:116)

  accuracy review: contributing guide

- [bk/src/appendices/contributing/development_environment_setup.md:153](D:/code/rust_howto/bk/src/appendices/contributing/development_environment_setup.md:153)

  accuracy review: contributing guide

- [bk/src/appendices/contributing/documentation_generation.md:114](D:/code/rust_howto/bk/src/appendices/contributing/documentation_generation.md:114)

  accuracy review: contributing guide

- [bk/src/appendices/contributing/index.md:188](D:/code/rust_howto/bk/src/appendices/contributing/index.md:188)

  accuracy review: contributing guide

- [bk/src/appendices/contributing/mdBook_pre-processors.md:39](D:/code/rust_howto/bk/src/appendices/contributing/mdBook_pre-processors.md:39)

  - [[mdbook | Mdbook]].

  accuracy review: contributing guide

- [bk/src/appendices/contributing/publication_to_crates.io.md:36](D:/code/rust_howto/bk/src/appendices/contributing/publication_to_crates.io.md:36)

  - [[cargo | Cargo]].
  - [[crate_registries | Crate Registries]].
  - [[github_actions | Github Actions]].
  - [[release_automation | Release Automation]].

  accuracy review: contributing guide

- [bk/src/appendices/contributing/repository_structure.md:106](D:/code/rust_howto/bk/src/appendices/contributing/repository_structure.md:106)

  - [[cargo | Cargo]].

  accuracy review: contributing guide

- [bk/src/appendices/contributing/topics_of_interest.md:78](D:/code/rust_howto/bk/src/appendices/contributing/topics_of_interest.md:78)

  accuracy review: contributing guide

- [bk/src/appendices/thanks.md:22](D:/code/rust_howto/bk/src/appendices/thanks.md:22)

  accuracy review: decisions for the maintainer

- [bk/src/categories/algorithms/graph_algorithms.md:93](D:/code/rust_howto/bk/src/categories/algorithms/graph_algorithms.md:93)

  accuracy review: algorithms

- [bk/src/categories/algorithms/index.md:133](D:/code/rust_howto/bk/src/categories/algorithms/index.md:133)

  accuracy review: algorithms

- [bk/src/categories/algorithms/random_value_generation.md:100](D:/code/rust_howto/bk/src/categories/algorithms/random_value_generation.md:100)

  accuracy review: algorithms

- [bk/src/categories/algorithms/sorting.md:71](D:/code/rust_howto/bk/src/categories/algorithms/sorting.md:71)

  accuracy review: algorithms

- [bk/src/categories/data-structures/b-trees.md:52](D:/code/rust_howto/bk/src/categories/data-structures/b-trees.md:52)

  accuracy review: data structures

- [bk/src/categories/data-structures/binary_heaps.md:46](D:/code/rust_howto/bk/src/categories/data-structures/binary_heaps.md:46)

  accuracy review: data structures

- [bk/src/categories/data-structures/bit_arrays.md:74](D:/code/rust_howto/bk/src/categories/data-structures/bit_arrays.md:74)

  accuracy review: data structures

- [bk/src/categories/data-structures/hashmaps.md:77](D:/code/rust_howto/bk/src/categories/data-structures/hashmaps.md:77)

  - [[concurrent_data_structures | Concurrent Data Structures]].
  - [[databases | Databases]].
  - [[hashing | Hashing]].
  - [[key_value_stores | Key-Value Stores]].

  accuracy review: data structures

- [bk/src/categories/data-structures/index.md:102](D:/code/rust_howto/bk/src/categories/data-structures/index.md:102)

  data-structures: expand
  accuracy review: data structures

- [bk/src/categories/data-structures/linked_lists.md:34](D:/code/rust_howto/bk/src/categories/data-structures/linked_lists.md:34)

  accuracy review: data structures

- [bk/src/categories/data-structures/other_maps.md:99](D:/code/rust_howto/bk/src/categories/data-structures/other_maps.md:99)

  accuracy review: data structures

- [bk/src/categories/data-structures/stack-allocated.md:91](D:/code/rust_howto/bk/src/categories/data-structures/stack-allocated.md:91)

  accuracy review: data structures

- [bk/src/categories/data-structures/strings.md:82](D:/code/rust_howto/bk/src/categories/data-structures/strings.md:82)

  - [[encoding | Encoding]].
  - [[internationalization | Internationalization]].
  - [[localization | Localization]].
  - [[search | Search]].
  - [[rust_search_engines | Rust Search Engines]].
  - [[template_engine | Template Engine]].
  - [[text_layout | Text Layout]].
  - [[unicode | Unicode]] handling.
  - [[value_formatting | Value Formatting]].

  accuracy review: data structures

- [bk/src/categories/data-structures/unique_identifiers.md:46](D:/code/rust_howto/bk/src/categories/data-structures/unique_identifiers.md:46)

  accuracy review: data structures

- [bk/src/categories/data-structures/vectors.md:80](D:/code/rust_howto/bk/src/categories/data-structures/vectors.md:80)

  accuracy review: data structures

- [bk/src/categories/filesystem/directories.md:51](D:/code/rust_howto/bk/src/categories/filesystem/directories.md:51)

  accuracy review: file system

- [bk/src/categories/filesystem/directory_traversal.md:139](D:/code/rust_howto/bk/src/categories/filesystem/directory_traversal.md:139)

  accuracy review: file system

- [bk/src/categories/filesystem/file_metadata.md:34](D:/code/rust_howto/bk/src/categories/filesystem/file_metadata.md:34)

  accuracy review: file system

- [bk/src/categories/filesystem/file_watching.md:28](D:/code/rust_howto/bk/src/categories/filesystem/file_watching.md:28)

  accuracy review: file system

- [bk/src/categories/filesystem/index.md:78](D:/code/rust_howto/bk/src/categories/filesystem/index.md:78)

  accuracy review: file system

- [bk/src/categories/filesystem/paths.md:78](D:/code/rust_howto/bk/src/categories/filesystem/paths.md:78)

  accuracy review: file system

- [bk/src/categories/filesystem/reading_and_writing_files.md:81](D:/code/rust_howto/bk/src/categories/filesystem/reading_and_writing_files.md:81)

  accuracy review: file system

- [bk/src/categories/filesystem/symbolic_links.md:25](D:/code/rust_howto/bk/src/categories/filesystem/symbolic_links.md:25)

  accuracy review: file system

- [bk/src/categories/filesystem/temporary_files_and_directories.md:31](D:/code/rust_howto/bk/src/categories/filesystem/temporary_files_and_directories.md:31)

  accuracy review: file system

- [bk/src/categories/filesystem/user_directories_and_preferred_applications.md:49](D:/code/rust_howto/bk/src/categories/filesystem/user_directories_and_preferred_applications.md:49)

  accuracy review: file system

- [bk/src/categories/filesystem/working_with_the_standard_input_and_output.md:25](D:/code/rust_howto/bk/src/categories/filesystem/working_with_the_standard_input_and_output.md:25)

  accuracy review: file system

- [bk/src/categories/mathematics/additional_numeric_types.md:116](D:/code/rust_howto/bk/src/categories/mathematics/additional_numeric_types.md:116)

  accuracy review: mathematics

- [bk/src/categories/mathematics/differential_equations.incl.md:21](D:/code/rust_howto/bk/src/categories/mathematics/differential_equations.incl.md:21)

  accuracy review: mathematics

- [bk/src/categories/mathematics/discrete_mathematics.md:25](D:/code/rust_howto/bk/src/categories/mathematics/discrete_mathematics.md:25)

  accuracy review: mathematics

- [bk/src/categories/mathematics/graph_theory.md:35](D:/code/rust_howto/bk/src/categories/mathematics/graph_theory.md:35)

  accuracy review: mathematics

- [bk/src/categories/mathematics/index.md:123](D:/code/rust_howto/bk/src/categories/mathematics/index.md:123)

  accuracy review: mathematics
  accuracy review: decisions for the maintainer

- [bk/src/categories/mathematics/linear_algebra.md:175](D:/code/rust_howto/bk/src/categories/mathematics/linear_algebra.md:175)

  accuracy review: mathematics

- [bk/src/categories/mathematics/numerical_analysis.md:25](D:/code/rust_howto/bk/src/categories/mathematics/numerical_analysis.md:25)

  accuracy review: mathematics

- [bk/src/categories/mathematics/optimization.md:74](D:/code/rust_howto/bk/src/categories/mathematics/optimization.md:74)

  accuracy review: mathematics

- [bk/src/categories/mathematics/statistics.md:52](D:/code/rust_howto/bk/src/categories/mathematics/statistics.md:52)

  accuracy review: mathematics

- [bk/src/categories/mathematics/units.md:25](D:/code/rust_howto/bk/src/categories/mathematics/units.md:25)

  accuracy review: mathematics

- [bk/src/crate-selection/current_state_of_the_art.md:40](D:/code/rust_howto/bk/src/crate-selection/current_state_of_the_art.md:40)

  accuracy review: links and crate selection

- [bk/src/crate-selection/index.md:72](D:/code/rust_howto/bk/src/crate-selection/index.md:72)

  accuracy review: links and crate selection

- [bk/src/introduction/how_to_read_this_book.md:97](D:/code/rust_howto/bk/src/introduction/how_to_read_this_book.md:97)

  ## Important Sections {#important-sections .skip}

  - The book covers cross-cutting concerns that affect most aspects of development e.g. [error handling](../standard-library/recoverable_error_handling.md), [[error_customization | error customization]], [[config | configuration]], [[development-tools_debugging | debugging]]...
  - [[concurrency | Concurrency]], including [[asynchronous | asynchronous programming]], is covered in details.
  - So are [[development tools]] and programming domains such as [[command-line-interface | CLI]] and [[web_programming | Web]] development.

  accuracy review: introduction and language basics

- [bk/src/introduction/index.md:67](D:/code/rust_howto/bk/src/introduction/index.md:67)

  accuracy review: introduction and language basics
  accuracy review: decisions for the maintainer

- [bk/src/language/attributes.md:149](D:/code/rust_howto/bk/src/language/attributes.md:149)

  accuracy review: traits, iterators, attributes and macros

- [bk/src/language/closures.md:88](D:/code/rust_howto/bk/src/language/closures.md:88)

  accuracy review: introduction and language basics

- [bk/src/language/constants_and_statics.md:67](D:/code/rust_howto/bk/src/language/constants_and_statics.md:67)

  [When are Rust's `const fn`s executed?][blog~const-fn]↗.

  accuracy review: introduction and language basics

- [bk/src/language/data_types.md:111](D:/code/rust_howto/bk/src/language/data_types.md:111)

  accuracy review: introduction and language basics

- [bk/src/language/entry_points.md:56](D:/code/rust_howto/bk/src/language/entry_points.md:56)

  - [[asynchronous | Asynchronous]].

  accuracy review: introduction and language basics

- [bk/src/language/functions.md:99](D:/code/rust_howto/bk/src/language/functions.md:99)

  - [[rust-patterns | Rust Patterns]].

  accuracy review: introduction and language basics

- [bk/src/language/impl_trait.md:42](D:/code/rust_howto/bk/src/language/impl_trait.md:42)

  - [[rust-patterns | Rust Patterns]].

  accuracy review: traits, iterators, attributes and macros

- [bk/src/language/iterators.md:111](D:/code/rust_howto/bk/src/language/iterators.md:111)

  accuracy review: traits, iterators, attributes and macros

- [bk/src/language/lifetimes.md:138](D:/code/rust_howto/bk/src/language/lifetimes.md:138)

  accuracy review: introduction and language basics

- [bk/src/language/macros.md:431](D:/code/rust_howto/bk/src/language/macros.md:431)

  accuracy review: traits, iterators, attributes and macros

- [bk/src/language/ownership_and_borrowing.md:161](D:/code/rust_howto/bk/src/language/ownership_and_borrowing.md:161)

  - [[rust-patterns | Rust Patterns]].

  accuracy review: introduction and language basics

- [bk/src/language/pattern_matching.md:229](D:/code/rust_howto/bk/src/language/pattern_matching.md:229)

  - [[rust-patterns | Rust Patterns]].

  accuracy review: introduction and language basics

- [bk/src/language/slices.md:79](D:/code/rust_howto/bk/src/language/slices.md:79)

  accuracy review: introduction and language basics

- [bk/src/language/structs.md:152](D:/code/rust_howto/bk/src/language/structs.md:152)

  - [[rust-patterns | Rust Patterns]].

  accuracy review: introduction and language basics

- [bk/src/language/trait_objects_and_dynamic_dispatch.md:182](D:/code/rust_howto/bk/src/language/trait_objects_and_dynamic_dispatch.md:182)

  accuracy review: traits, iterators, attributes and macros

- [bk/src/language/traits.md:186](D:/code/rust_howto/bk/src/language/traits.md:186)

  - [[rust-patterns | Rust Patterns]].

  accuracy review: traits, iterators, attributes and macros

- [bk/src/language/variables.md:49](D:/code/rust_howto/bk/src/language/variables.md:49)

  accuracy review: introduction and language basics

- [bk/src/links/blogs_and_forums.md:103](D:/code/rust_howto/bk/src/links/blogs_and_forums.md:103)

  accuracy review: links and crate selection

- [bk/src/links/books.md:211](D:/code/rust_howto/bk/src/links/books.md:211)

  accuracy review: links and crate selection

- [bk/src/links/example_code.md:100](D:/code/rust_howto/bk/src/links/example_code.md:100)

  accuracy review: links and crate selection

- [bk/src/links/index.md:61](D:/code/rust_howto/bk/src/links/index.md:61)

  accuracy review: links and crate selection

- [bk/src/links/learning_rust.md:110](D:/code/rust_howto/bk/src/links/learning_rust.md:110)

  accuracy review: links and crate selection

- [bk/src/links/meetings.md:56](D:/code/rust_howto/bk/src/links/meetings.md:56)

  accuracy review: links and crate selection

- [bk/src/links/rust_companies_and_jobs.md:226](D:/code/rust_howto/bk/src/links/rust_companies_and_jobs.md:226)

  accuracy review: decisions for the maintainer

- [bk/src/rust-installation/index.md:96](D:/code/rust_howto/bk/src/rust-installation/index.md:96)

  accuracy review: introduction and language basics

- [bk/src/standard-library/automatic_trait_derivation.md:78](D:/code/rust_howto/bk/src/standard-library/automatic_trait_derivation.md:78)

  accuracy review: standard library

- [bk/src/standard-library/borrowed_types.md:79](D:/code/rust_howto/bk/src/standard-library/borrowed_types.md:79)

  accuracy review: standard library

- [bk/src/standard-library/clone-on-write.md:103](D:/code/rust_howto/bk/src/standard-library/clone-on-write.md:103)

  accuracy review: standard library

- [bk/src/standard-library/conversion_traits.md:66](D:/code/rust_howto/bk/src/standard-library/conversion_traits.md:66)

  accuracy review: standard library

- [bk/src/standard-library/default_values.md:29](D:/code/rust_howto/bk/src/standard-library/default_values.md:29)

  accuracy review: standard library

- [bk/src/standard-library/heap_storage.md:87](D:/code/rust_howto/bk/src/standard-library/heap_storage.md:87)

  accuracy review: standard library

- [bk/src/standard-library/index.md:136](D:/code/rust_howto/bk/src/standard-library/index.md:136)

  - [Tour of Rust's Standard Library Traits][tour-of-rusts-standard-library-traits~repo]↗.
  - [`black_box` in `core::hint`][c~core::hint::black_box~docs]↗.
  - [`std::pin`][c~std::pin~projections-and-structural-pinning~docs]↗.
  - [`core::iter`][c~core::iter~docs]↗.

  accuracy review: standard library

- [bk/src/standard-library/interior_mutability.md:78](D:/code/rust_howto/bk/src/standard-library/interior_mutability.md:78)

  accuracy review: standard library

- [bk/src/standard-library/memory_address_pinning.md:53](D:/code/rust_howto/bk/src/standard-library/memory_address_pinning.md:53)

  - [[async_programming | Async Programming]].

  accuracy review: standard library

- [bk/src/standard-library/optional_values.md:147](D:/code/rust_howto/bk/src/standard-library/optional_values.md:147)

  accuracy review: standard library

- [bk/src/standard-library/recoverable_error_handling.md:102](D:/code/rust_howto/bk/src/standard-library/recoverable_error_handling.md:102)

  - [[recoverable_error_handling | Error Handling]].
  - [[error_customization | Error Customization]].
  - [[rust-patterns | Rust Patterns]].

  accuracy review: standard library

- [bk/src/standard-library/reference-to-reference_conversions.md:68](D:/code/rust_howto/bk/src/standard-library/reference-to-reference_conversions.md:68)

  accuracy review: standard library

- [bk/src/standard-library/reference_counting.md:51](D:/code/rust_howto/bk/src/standard-library/reference_counting.md:51)

  - [[concurrency | Concurrency]].

  accuracy review: standard library

- [bk/src/standard-library/resource_cleanup.md:51](D:/code/rust_howto/bk/src/standard-library/resource_cleanup.md:51)

  - [[memory-management | Memory Management]].

  accuracy review: standard library

- [bk/src/standard-library/smart_pointers.md:125](D:/code/rust_howto/bk/src/standard-library/smart_pointers.md:125)

  accuracy review: standard library

- [bk/src/standard-library/value_comparison.md:55](D:/code/rust_howto/bk/src/standard-library/value_comparison.md:55)

  accuracy review: standard library

### References, evidence, and other notes (85)

- [bk/src/appendices/contributing/mdBook_pre-processors.md:26](D:/code/rust_howto/bk/src/appendices/contributing/mdBook_pre-processors.md:26)

  and

- [bk/src/appendices/contributing/mdBook_pre-processors.md:26](D:/code/rust_howto/bk/src/appendices/contributing/mdBook_pre-processors.md:26)

- [bk/src/categories/data-structures/graphs.md:31](D:/code/rust_howto/bk/src/categories/data-structures/graphs.md:31)

- [bk/src/categories/data-structures/stacks_and_queues.md:38](D:/code/rust_howto/bk/src/categories/data-structures/stacks_and_queues.md:38)

- [bk/src/code-organization/dependencies.md:42](D:/code/rust_howto/bk/src/code-organization/dependencies.md:42)

  - [[cargo | Cargo]].
    - [[dependency_management | Dependency Management]].
    - [[development-tools_cargo-plugins | Development Tools - Cargo Plugins]].

- [bk/src/indices/crates_alphabetical.md:366](D:/code/rust_howto/bk/src/indices/crates_alphabetical.md:366)

- [bk/src/indices/crates_by_category.md:50](D:/code/rust_howto/bk/src/indices/crates_by_category.md:50)

- [bk/src/indices/examples_index.md:402](D:/code/rust_howto/bk/src/indices/examples_index.md:402)

- [bk/src/language/control_flow.md:81](D:/code/rust_howto/bk/src/language/control_flow.md:81)

  - [[rust-patterns | Rust Patterns]].

- [bk/src/language/enumerations.md:91](D:/code/rust_howto/bk/src/language/enumerations.md:91)

- [bk/src/language/generics.md:92](D:/code/rust_howto/bk/src/language/generics.md:92)

  - [[rust-patterns | Rust Patterns]].

- [bk/src/language/index.md:92](D:/code/rust_howto/bk/src/language/index.md:92)

- [bk/src/links/rust_cheatsheets.md:32](D:/code/rust_howto/bk/src/links/rust_cheatsheets.md:32)

- [bk/src/links/videos_and_podcasts.md:64](D:/code/rust_howto/bk/src/links/videos_and_podcasts.md:64)

- [bk/src/standard-library/operators.md:41](D:/code/rust_howto/bk/src/standard-library/operators.md:41)

- [bk/src/SUMMARY.md:152](D:/code/rust_howto/bk/src/SUMMARY.md:152)

- [bk/drafts/categories/asynchronous/futures.md:62](D:/code/rust_howto/bk/drafts/categories/asynchronous/futures.md:62)

  futures = Utility [functions][p~functions] for working with Futures and Streams

- [bk/drafts/categories/caching/index.md:70](D:/code/rust_howto/bk/drafts/categories/caching/index.md:70)

  Added a cache-selection overview that compares process-local, hybrid, and distributed options, and explains cache scope, expiration, invalidation, and cache-aside flows.

- [bk/drafts/categories/command-line-interface/ansi_terminal.md:143](D:/code/rust_howto/bk/drafts/categories/command-line-interface/ansi_terminal.md:143)

  ansi_terminal: `ansi_term` is archived; write / decide what to cover

- [bk/drafts/categories/concurrency/concurrent_data_structures.md:87](D:/code/rust_howto/bk/drafts/categories/concurrency/concurrent_data_structures.md:87)

- [bk/drafts/categories/concurrency/data_parallelism.md:112](D:/code/rust_howto/bk/drafts/categories/concurrency/data_parallelism.md:112)

  data_parallelism: polish; dedupe with multithreading.md

- [bk/drafts/categories/concurrency/message_passing.md:119](D:/code/rust_howto/bk/drafts/categories/concurrency/message_passing.md:119)

- [bk/drafts/categories/config/environment_variables.md:58](D:/code/rust_howto/bk/drafts/categories/config/environment_variables.md:58)

  environment_variables: interaction between [config][p~config] and env variables

- [bk/drafts/categories/data-structures/elsa.md:49](D:/code/rust_howto/bk/drafts/categories/data-structures/elsa.md:49)

- [bk/drafts/categories/data-structures/phf.md:37](D:/code/rust_howto/bk/drafts/categories/data-structures/phf.md:37)

- [bk/drafts/categories/database/amqp.incl.md:16](D:/code/rust_howto/bk/drafts/categories/database/amqp.incl.md:16)

  This shared include introduces AMQP, RabbitMQ's queue flow, and the `lapin` client used by the recipe.

- [bk/drafts/categories/database/amqp.md:58](D:/code/rust_howto/bk/drafts/categories/database/amqp.md:58)

  The `lapin` recipe now uses a temporary queue, publisher confirms, message verification, consumption, and acknowledgement. Its RabbitMQ integration test stays ignored unless a broker is running; the Docker command and invocation are documented above.

- [bk/drafts/categories/database/key_value_stores.md:52](D:/code/rust_howto/bk/drafts/categories/database/key_value_stores.md:52)

- [bk/drafts/categories/database/oracle.md:68](D:/code/rust_howto/bk/drafts/categories/database/oracle.md:68)

  The example reads the built-in `DUAL` table so it works without Oracle's sample schemas. Build it with `cargo run -p database --example oracle --features sibyl -- sibyl`. Run the ignored integration test with `cargo test -p database --example oracle --features sibyl -- sibyl::tests::connects_to_oracle_with_sibyl --ignored`; set `ORACLE_CONNECT_STRING`, `ORACLE_USERNAME`, and `ORACLE_PASSWORD` first.

- [bk/drafts/categories/database/postgres.md:59](D:/code/rust_howto/bk/drafts/categories/database/postgres.md:59)

  The ignored integration test creates a connection-local `artists` table and verifies the counts with a small fixture. Running the example itself reads the existing `artists` table from `DATABASE_URL`.

- [bk/drafts/categories/database/query_builders_orms.md:93](D:/code/rust_howto/bk/drafts/categories/database/query_builders_orms.md:93)

  Issue #709 replaces the SQL-printing placeholder with a runnable Diesel flow using `diesel_migrations::embed_migrations!`. The example owns its migration files and uses SQLite `:memory:`, so it needs neither `DATABASE_URL` nor an external database service.

- [bk/drafts/categories/database/sqlite.md:53](D:/code/rust_howto/bk/drafts/categories/database/sqlite.md:53)

- [bk/drafts/categories/database-implementations/databases.md:66](D:/code/rust_howto/bk/drafts/categories/database-implementations/databases.md:66)

  The example uses SurrealDB 3's current `SurrealValue` derive and the in-memory `Mem` engine so its test exercises the full CRUD path without a server or network service.

- [bk/drafts/categories/date-and-time/duration.md:52](D:/code/rust_howto/bk/drafts/categories/date-and-time/duration.md:52)

- [bk/drafts/categories/date-and-time/index.md:35](D:/code/rust_howto/bk/drafts/categories/date-and-time/index.md:35)

  The `time` crate represents UTC offsets and can query the system offset when its `local-offset` feature is enabled; it does not provide a bundled IANA time-zone database. `chrono-tz` supplies that database for Chrono. Keep this category in drafts while its detail pages still contain unresolved placeholders.

- [bk/drafts/categories/date-and-time/time_crate.md:28](D:/code/rust_howto/bk/drafts/categories/date-and-time/time_crate.md:28)

- [bk/drafts/categories/development-tools/index.md:97](D:/code/rust_howto/bk/drafts/categories/development-tools/index.md:97)

  index: reorganize; dedupe alternatives / [`log`][c~log~docs]↗{{hi:log}} / config_log

  ## Editors

  - [`lapce`][lapce~repo]↗: Lightning-fast and Powerful Code Editor written in Rust.
  - [`intellij-rust`][intellij-rust~repo]↗: Rust plugin for the IntelliJ Platform.

  ## Registries

  - [`alexandrie`][c~alexandrie~repo]↗: An alternative crate registry, implemented in Rust.
  - [`meuse`][meuse~repo]↗: A private Cargo crate registry, for Rust.
  - [`kellnr`][kellnr~repo]↗: The registry for Rust crates.
  - [`panamax`][c~panamax~repo]↗: Mirror rustup and crates.io repositories, for offline Rust and cargo usage..

  ## Tests

  - [`assertables`][c~assertables~docs]↗.
  - [`proptest`][c~proptest~repo]↗: Hypothesis-like property testing for Rust.

  ## Cargo Plugins

  - [`cargo-semver-checks`][c~cargo-semver-checks~crates.io]↗.
  - [`cargo-c`][c~cargo-c~crates.io]↗.
  - [`kondo`][kondo~repo]↗: Cleans dependencies and build artifacts from your projects.
  - [`cargo-sweep`][c~cargo-sweep~repo]↗: A cargo subcommand for cleaning up unused build files generated by Cargo.
  - [`cargo-update`][c~cargo-update~repo]↗: A cargo subcommand for checking and applying updates to installed executables.
  - [`cargo-xtask`][c~cargo-xtask~repo]↗.
  - [`cargo-script`][c~cargo-script~crates.io]↗.
  - [`cargo-play`][c~cargo-play~crates.io]↗.
  - [`sccache-action`][sccache-action~repo]↗: `sccache` github action.
  - [`cargo-spellcheck`][c~cargo-spellcheck~repo]↗ checks all your documentation for spelling and grammar mistakes with `hunspell` and a `nlprule`-based checker for grammar.
  - [`cargo-limit`][cargo-limit~repo]↗: Productivity improvements for Rust ecosystem: warnings are skipped until errors are fixed, LSP-independent Neovim integration, etc.
  - [`cargo-bloat`][c~cargo-bloat~repo]↗: Find out what takes most of the space in your executable.
  - [`cargo-generate`][c~cargo-generate~crates.io]↗.
  - [`cargo-workspaces`][c~cargo-workspaces~crates.io]↗.

  ## Linkers

  - [Mold][mold~repo]↗: A Modern Linker.

  ## Cross Compilation

  - [Cross-Platform Development][rustmeup-tutorials-cross-platform-development~website]↗.
  - [Cross-compilation in Rust][blog~kerkour-rust-cross-compilation]↗.

  ## Live Reload

  - [So you want to live-reload Rust][blog~fasterthanli.me-so-you-want-to-live-reload-rust]↗.
  - [`watchexec`][c~watchexec~repo]↗: Executes commands in response to file modifications.

  ## CD / CI

  - [`act`][act~repo]↗: Run your GitHub Actions locally.

  ## Others

  - [`vscode-just`][vscode-just~repo]↗: VSCode syntax highlighting for just files.

  - [`dotbot`][dotbot~repo]↗: A tool that bootstraps your dotfiles.

  - [Bazel at Enterprise Scale | BuildBuddy][buildbuddy~website]↗.

  - [The Pants Build System][pants~repo]↗.

  - [`mini-docker-rust`][mini-docker-rust~repo]↗: Very small rust docker image.

  - [`contracts`][c~contracts~crates.io]↗.

  - [Vagrant by HashiCorp][vagrantup~website]↗.

  - [`include_dir` - Rust dev tool][c~include_dir~lib.rs]↗: An evolution of the `include_str!()` and `include_bytes!()` macros for embedding an entire directory tree into your binary.

- [bk/drafts/categories/development-tools/installation/rustup.md:39](D:/code/rust_howto/bk/drafts/categories/development-tools/installation/rustup.md:39)

- [bk/drafts/categories/development-tools/other/miri.md:42](D:/code/rust_howto/bk/drafts/categories/development-tools/other/miri.md:42)

  miri: polish

- [bk/drafts/categories/development-tools_build-utils/build_time_tooling.incl.md:11](D:/code/rust_howto/bk/drafts/categories/development-tools_build-utils/build_time_tooling.incl.md:11)

  The build information example uses `build-info` and `build-info-build` to embed build metadata in the binary.

- [bk/drafts/categories/development-tools_debugging/config_log.md:90](D:/code/rust_howto/bk/drafts/categories/development-tools_debugging/config_log.md:90)

- [bk/drafts/categories/development-tools_debugging/diagnostic_functions.md:26](D:/code/rust_howto/bk/drafts/categories/development-tools_debugging/diagnostic_functions.md:26)

  - The debugging category is the intended location because these functions assist runtime inspection; `type_name` is unsuitable as a stable identifier.

- [bk/drafts/categories/development-tools_testing/assertions.md:69](D:/code/rust_howto/bk/drafts/categories/development-tools_testing/assertions.md:69)

  - [pretty_assertions - Rust dev tool][c~pretty_assertions~lib.rs]↗.

- [bk/drafts/categories/encoding/_binary_encoders.md:108](D:/code/rust_howto/bk/drafts/categories/encoding/_binary_encoders.md:108)

  - [pot][c~pot~docs]↗.
    - [`protobuf`][c~protobuf~docs]↗: another Protocol Buffers implementation; its recipe was dropped in favor of `prost`.
    - [bytes][c~bytes~lib.rs]↗.

- [bk/drafts/categories/encoding/no_external_schema.md:46](D:/code/rust_howto/bk/drafts/categories/encoding/no_external_schema.md:46)

  This chapter compares formats that use application types without a separate schema file, including their wire-level type information and `no_std` support. The `rkyv` example remains as a distinct type-specific archive approach.

- [bk/drafts/categories/hardware-support/peripherals.md:67](D:/code/rust_howto/bk/drafts/categories/hardware-support/peripherals.md:67)

  Expanded the peripheral guide with USB, serial, HID, GPIO, networking, GPU, driver, and user-space access guidance, including platform permission boundaries and local topic references.

- [bk/drafts/categories/memory-management/lazy_initialization.md:98](D:/code/rust_howto/bk/drafts/categories/memory-management/lazy_initialization.md:98)

- [bk/drafts/categories/memory-management/memory_allocation.md:102](D:/code/rust_howto/bk/drafts/categories/memory-management/memory_allocation.md:102)

  - [Jemalloc][c~jemalloc~lib.rs]↗.

- [bk/drafts/categories/parser-implementations/html.md:70](D:/code/rust_howto/bk/drafts/categories/parser-implementations/html.md:70)

  This chapter now distinguishes HTML5 parsing, DOM-style CSS selection, and CSS syntax parsing, adds a tested `scraper` recipe, and links related XML, scraping, and templating guidance.

- [bk/drafts/categories/parser-implementations/markdown.md:53](D:/code/rust_howto/bk/drafts/categories/parser-implementations/markdown.md:53)

- [bk/drafts/categories/parser-implementations/yaml.md:31](D:/code/rust_howto/bk/drafts/categories/parser-implementations/yaml.md:31)

  This chapter recommends an actively maintained Serde-based YAML implementation, distinguishes it from lower-level parsers, and demonstrates typed serialization and deserialization with a tested round trip.

- [bk/drafts/categories/parsing/parsing.md:79](D:/code/rust_howto/bk/drafts/categories/parsing/parsing.md:79)

  - [Awesome Pest][awesome-pest~repo]↗: A curated list of resources, projects, and tools using or for the `pest` parser generator in Rust.
  - [ABNF to Pest][c~abnf_to_pest~crates.io]↗.
  - [`winnow`][c~winnow~docs]↗.
  - [parse-rosetta-rs][parse-rosetta-rs~repo]↗.

- [bk/drafts/categories/rust-patterns/behavioral_patterns.md:41](D:/code/rust_howto/bk/drafts/categories/rust-patterns/behavioral_patterns.md:41)

- [bk/drafts/categories/rust-patterns/builder_pattern.md:45](D:/code/rust_howto/bk/drafts/categories/rust-patterns/builder_pattern.md:45)

  - [Next-gen builder macro Bon Revolutional typestate design][blog~bon-v3-release]↗.

- [bk/drafts/categories/rust-patterns/index.md:147](D:/code/rust_howto/bk/drafts/categories/rust-patterns/index.md:147)

  - [patterns][rust-unofficial-patterns~repo]↗: A catalog of Rust design patterns, anti-patterns and idioms.

  ```rust,editable
  {{#include ../../../crates/cats/rust_patterns/examples/scopeguard.rs:example}}
  ```

- [bk/drafts/categories/rust-patterns/rust_specific_patterns.md:104](D:/code/rust_howto/bk/drafts/categories/rust-patterns/rust_specific_patterns.md:104)

  - [& vs. ref in Rust patterns][blog~rust-patterns-ref]↗.

- [bk/drafts/categories/template-engine/index.md:40](D:/code/rust_howto/bk/drafts/categories/template-engine/index.md:40)

  [askama][c~askama~repo]↗: Type-safe, compiled Jinja-like templates for Rust.

- [bk/drafts/categories/text-editors/ides.md:73](D:/code/rust_howto/bk/drafts/categories/text-editors/ides.md:73)

- [bk/drafts/categories/text-editors/index.md:42](D:/code/rust_howto/bk/drafts/categories/text-editors/index.md:42)

  The table distinguishes native support from plugin and LSP setups, fills in integration details, removes Atom from the current-editor list because GitHub sunset it, and replaces the unfinished related-topics placeholder. Recheck vendor setup links when updating this index.

- [bk/drafts/categories/text-processing/regex.md:93](D:/code/rust_howto/bk/drafts/categories/text-processing/regex.md:93)

  - [grex][c~grex~repo]↗: A command-line tool and Rust library with Python bindings for generating regular expressions from user-provided test cases.

  - [regex_automata][c~regex-automata~docs]↗: Multi-pattern searches with capture groups.

- [bk/drafts/categories/text-processing/string_concat.md:42](D:/code/rust_howto/bk/drafts/categories/text-processing/string_concat.md:42)

  This review separates runtime concatenation methods from compile-time concatenation, adds the requested capacity-reservation example, and gives the existing second example a visible `constcat` section. The obsolete `concat_strs` dependency and duplicated recipes were removed.

- [bk/drafts/categories/text-processing/string_manipulation.md:71](D:/code/rust_howto/bk/drafts/categories/text-processing/string_manipulation.md:71)

  This chapter compares the old `Inflector` release with `heck`, adds tested title-case and snake-case conversions, and demonstrates Unicode-safe first-character capitalization.

- [bk/drafts/categories/text-processing/string_parsing.md:27](D:/code/rust_howto/bk/drafts/categories/text-processing/string_parsing.md:27)

  This review replaces unchecked byte slicing with explicit format and ASCII validation, gives parse failures a focused error type, and covers valid and malformed input in unit tests. The unrelated validator bookmark was dropped.

- [bk/drafts/categories/web-programming/http_types_and_interfaces.md:21](D:/code/rust_howto/bk/drafts/categories/web-programming/http_types_and_interfaces.md:21)

  Issue #1355 expands the sample from a basic request/response builder into header construction and interpretation. It covers typed `Accept`, `Authorization`, `Content-Type`, and `Content-Length` names, a custom name, text and numeric parsing, repeated `Set-Cookie` values, invalid-value rejection, and offline tests.

- [bk/drafts/categories/web-programming/mime.md:51](D:/code/rust_howto/bk/drafts/categories/web-programming/mime.md:51)

  This review replaces the hand-written extension match with `mime_guess`, documents that its result comes from the filename, and adds local tests for response `Content-Type` parsing. The live HTTP example is now opt-in.

- [bk/drafts/categories/web-programming/scraping.md:53](D:/code/rust_howto/bk/drafts/categories/web-programming/scraping.md:53)

  This review replaces the older `select` API with `scraper`, fixes base URL resolution and HTTP status handling, and adds deterministic local HTML tests. Browser automation alternatives are visible for sites that depend on JavaScript.

- [bk/drafts/categories/web-programming/url.md:75](D:/code/rust_howto/bk/drafts/categories/web-programming/url.md:75)

  This review removes a stale commented implementation, makes the URL joining example show relative-path behavior, and ensures the query/fragment example actually contains and removes both components. It adds focused examples for path, query, fragment, and port handling.

- [bk/drafts/categories/web-programming_http-client/download.md:53](D:/code/rust_howto/bk/drafts/categories/web-programming_http-client/download.md:53)

  This review changes the basic download example to stream binary chunks into a temporary file, verifies that behavior with a local server, and corrects the upload and byte-range descriptions to match the existing examples.

- [bk/drafts/categories/web-programming_http-client/requests.md:57](D:/code/rust_howto/bk/drafts/categories/web-programming_http-client/requests.md:57)

  This review makes success-status handling explicit, uses HTTPS for the remote examples, and keeps network-dependent checks opt-in. A local HTTP echo test covers default headers, bearer authentication, and JSON decoding without relying on an external service.

- [bk/drafts/categories/web-programming_http-server/middleware.md:52](D:/code/rust_howto/bk/drafts/categories/web-programming_http-server/middleware.md:52)

  This review adds a concrete Axum `from_fn` middleware example, verifies it together with the existing CORS layer using a local request, and clarifies the permissive CORS and alternative-framework guidance. The Tower concepts remain concise because the adjacent code demonstrates the composition directly.

- [bk/drafts/indices/crates_and_examples.md:18](D:/code/rust_howto/bk/drafts/indices/crates_and_examples.md:18)

  create script to generate crate pages

- [bk/drafts/self_referential.md:42](D:/code/rust_howto/bk/drafts/self_referential.md:42)

- [later/src/categories/computer-vision/index.md:59](D:/code/rust_howto/later/src/categories/computer-vision/index.md:59)

- [later/src/categories/emulators/emulators.md:47](D:/code/rust_howto/later/src/categories/emulators/emulators.md:47)

  [in-depth-analysis-of-polkavm-a-perfect-path-to-understanding-polkadot][blog~medium~in-depth-analysis-of-polkavm].

- [later/src/categories/science_neuroscience/index.md:42](D:/code/rust_howto/later/src/categories/science_neuroscience/index.md:42)

  also cover Biology! see lib.rs categorization

- [later/src/categories/value-formatting/number_formatting.md:68](D:/code/rust_howto/later/src/categories/value-formatting/number_formatting.md:68)

  [`num-format`][c~num-format~docs]↗{{hi:num-format}}, [`icu`][c~icu~docs]↗{{hi:icu}}, and [`fluent`][c~fluent~docs]↗{{hi:fluent}} address grouped integers, locale-aware decimal output, and translated messages, respectively.

- [later/src/categories/virtualization/using_containers.incl.md:3](D:/code/rust_howto/later/src/categories/virtualization/using_containers.incl.md:3)

  Empty

- [later/src/categories/wasm/wasm_basics.incl.md:3](D:/code/rust_howto/later/src/categories/wasm/wasm_basics.incl.md:3)

  Empty.

- [later/src/categories/wasm/wasm_basics.md:51](D:/code/rust_howto/later/src/categories/wasm/wasm_basics.md:51)

  final review

- [later/src/categories/web-programming_websocket/websocket.md:80](D:/code/rust_howto/later/src/categories/web-programming_websocket/websocket.md:80)

  Reviewed upstream tokio-tungstenite client and echo-server examples; updated the Tokio recipe to use `Utf8Bytes` and a tested local client/server round trip.

- [later/src/other/architecture/architectural_patterns.incl.md:3](D:/code/rust_howto/later/src/other/architecture/architectural_patterns.incl.md:3)

  Empty.

- [later/src/other/architecture/index.md:24](D:/code/rust_howto/later/src/other/architecture/index.md:24)

  - [tangleguard~website][tangleguard~website]↗. (poor quality?)

- [later/src/other/cloud/index.md:70](D:/code/rust_howto/later/src/other/cloud/index.md:70)

  edit

- [later/src/other/machine-learning/classical_ml.md:20](D:/code/rust_howto/later/src/other/machine-learning/classical_ml.md:20)

  ML examples; coordinate with #1757 (Linfa / ndarray compatibility)

- [later/src/other/scripting/task_automation.md:52](D:/code/rust_howto/later/src/other/scripting/task_automation.md:52)

  Editorial placement: keep command execution and scheduling in Scripting. See the Scheduling chapter for recurring jobs.
