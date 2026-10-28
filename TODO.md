<!-- TODO Reviewed 10-25-2026 -->

# TO DO List

- [ ] JOHN review tools/
- [ ] JOHN review .github\actions
- [ ] JOHN review / rewrite the .py scripts
- [ ] JOHN Review .pre-commit-config.yaml vs prek.toml
- [ ] JOHN review LICENSE
- [ ] JOHN bookmarks.json
- [ ] cargo husky?
- [ ] prek or pre-commit?

## Tasks

### Urgent

- [/] meaningful, not generic, README.md - see for example bk\crates\cats\asynchronous\README.md
- [ ] review all TODO comments in the codebase and fix
- [/] fix "function xyz is never used". most book examples should have a private main() function and a pub fn run() function + a test that call main(). run() should be called by the main() function of the example.
- [/] all book examples should have a test that calls the private main() function and verifies the expected behavior. also add tests to verify other functions / code items within the exanple. ensure coverage for edge cases and error handling.
- [ ] review dirty worktrees, create commits, integrate useful branches, discard stale work, and clean up topic refs
- [ ] integrate all useful topic branches into develop and discard the rest

### Scripts, Tools

- [ ] add just command for cargo fmt-toml; update Dockerfile and install scripts
- [/] verify that all .sh files can run in Git Bash (all 60 passed `bash -n`; runtime-dependent commands are deferred).

### Code Examples

- [/] improve comments within examples. add context and explanations where necessary. end with a period.
- [ ] WAIT move code examples from later/ to bk/ when code + text ready

---

## In progress

Linux tooling validation (2026-10-27): the previously unavailable `book_tooling` revision `404e8fe0d4adc40517b0b40a2536cc6833b29059` is published on `codex/rust-howto-pin-404e8fe`. In native Linux slot `a`, `tools` all-features Clippy passed and `just nta` passed 159 tests and doctests after isolating the mdBook fixture from the optional Pandoc renderer, surfacing doctest failures, and refreshing the alphabetical index. Full repository and container gates remain outstanding.

Windows tooling validation (2026-10-27): `tools` all-features Clippy and `just nta` passed 159 tests and doctests with `RUSTC_WRAPPER=''` and `CARGO_INCREMENTAL=1`. The first test attempt failed because the sccache server could not exchange data; the shared cache still needs investigation.

Tooling validation (2026-10-27), issue [#1997](https://github.com/john-cd/rust_howto/issues/1997): slot `e` contains in-progress fixes for optional submodule parsing, aggregate recipe failure reporting, Windows recipe-list path quoting, and Linux native prerequisites. Windows and WSL workspace listings run except for the uninitialized `mdbook-utils` submodule, and deliberately failing aggregate runs return nonzero on both platforms. NASM was installed on Windows; NASM and Ninja were installed in Ubuntu. Docker Compose configuration and shell syntax checks pass on Windows and Linux. Windows `just bb` passed with NASM on PATH; linkcheck produced warnings, indexing reported a version mismatch, and the optional Pandoc backend was unavailable. Complete runtime coverage of scripts and recipes, container builds, full workspace gates, and issue-by-issue validation remain. `dprint check` reports existing formatting drift in `bk/crates/cats/mathematics/Cargo.toml`; `rumdl` passes for book and staging prose.

B1 implementation commits are integrated into local `develop` through `7761c5e6c` (not pushed). Seven B1 issues are closed: #1941, #1940, #939, #1252, #1281, #1912, and #1950. #912 and #1058 remain open because their done criteria require PR links; #864 remains open because its human-review marker is preserved. Formatting, focused tests/Clippy, reference checks, Markdown lint, mdBook build/linkcheck, sitemap generation, static asset copy, and `git diff --check` passed. `just bb` passed in the elevated run; mdBook reported an indexing-plugin version warning and the optional Pandoc backend was unavailable.

Selected topic branches are integrated linearly into local `develop` through `fa58e2dba` (not pushed). This includes the async/concurrency examples and the science changes for #835, #837, #473, #839, and #840; the existing NIfTI example was retained alongside the new neuron simulation. GitHub marks #473 closed. Issues #835, #837, #839, and #840 remain open because their done criteria require PR links. Focused Clippy and tests passed for the changed science packages (including Candle), async/concurrency packages, MIME and GraphQL examples, and link-generation fixture. The full all-features Clippy gate is blocked by the missing Oracle `OCI_LIB_DIR`; the broad `development_tools_debugging` test run still fails in existing log examples that initialize the process-wide logger together.

Validation (2026-10-27): `dprint check` and `rumdl` across `bk/src`, `bk/drafts`, and `later/src` pass. All-features Clippy passes for `tools`, `xmpl`, `playground`, `publish`, the isolated WASM, Vizia, Python-extension, and embedded workspaces; the embedded all-target tests pass. The PostgreSQL-feature database package also passes Clippy with the Cornucopia client pinned to 0.4.0. GUI all-target Clippy passes with default features after disabling the stale Tauri build hook, keeping the disabled Tao source out of auto-discovered targets, and fixing Taffy `f32` literals. Full all-features Clippy for `bk` and the main `later` workspace remains blocked by the Windows `aws-lc-sys` 0.46.0 MSVC archive failure (LNK1114, error 5). The `just` entry point cannot start Git Bash in this session because MSYS cannot create its `BaseNamedObjects` directory.

### Code examples

#### ACTIVE-CODE-01 — Networking, authentication and email

- [#812](https://github.com/john-cd/rust_howto/issues/812) — **P2** — Finish and locally verify the Pingora reverse-proxy example — **PARTIAL; OPEN — Unix/Pingora validation and book build blocked**
- [#1058](https://github.com/john-cd/rust_howto/issues/1058) — **P4** — Replace the obsolete public WebSocket endpoint in the async-tungstenite example — Everyday examples and concrete fixes chat — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**

#### ACTIVE-CODE-02 — Databases and service clients

- [#1021](https://github.com/john-cd/rust_howto/issues/1021) — **P3** — Finish the Oracle example and document its optional Instant Client test — **PARTIAL; OPEN — Oracle test cannot link; live service validation pending**

#### ACTIVE-CODE-04 — Cryptography and Windows APIs

- [#705](https://github.com/john-cd/rust_howto/issues/705) — **P3** — Finish the X.509 certificate example — **PARTIAL; OPEN — book build blocked; human review pending**
- [#822](https://github.com/john-cd/rust_howto/issues/822) — **P3** — Review and test the Windows API example — **PARTIAL; OPEN — book build blocked; human review pending**

#### ACTIVE-CODE-05 — Parsing and XML

- [#826](https://github.com/john-cd/rust_howto/issues/826) — **P3** — Finish and test the Pest arithmetic example — **PARTIAL; OPEN — book build blocked; human review pending**
- [#1102](https://github.com/john-cd/rust_howto/issues/1102) — **P3** — Finish the XML parser example — **PARTIAL; OPEN — XML test blocked by onig_sys LNK1114; human review pending**

#### ACTIVE-CODE-07 — Scheduling and sandboxed runtimes

- [#1926](https://github.com/john-cd/rust_howto/issues/1926) — **P4** — Add Wasmtime fuel, memory-limit, and host-function tests — Text processing, automation, and local runtimes chat — **PARTIAL; OPEN — exact-source harness 5/5; Windows workspace test blocked by Wasmer Cranelift compilation**

#### ACTIVE-CODE-09 — ORMs, GraphQL, metrics and decision tables

- [#864](https://github.com/john-cd/rust_howto/issues/864) — **P3** — Finish Async-GraphQL with deterministic schema/query tests — Everyday examples and concrete fixes chat — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — HUMAN REVIEW MARKER PRESERVED**
- [#912](https://github.com/john-cd/rust_howto/issues/912) — **P3** — Extract the Toasty example and finish ORM guidance using in-memory SQLite — Everyday examples and concrete fixes chat — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**

### Prose and book structure

#### ACTIVE-PROSE-01 — Foreign function interfaces

- [#1073](https://github.com/john-cd/rust_howto/issues/1073) — **P4** — Complete Lua integration recipes and tested includes — Text processing, automation, and local runtimes chat — **PARTIAL; OPEN — sandbox guidance is documented, but a hostile-script sandbox and external module runtime test remain**

### Tools and infrastructure

#### ACTIVE-INFRA-01 — Preprocessors, references and recipe portability

- [#1253](https://github.com/john-cd/rust_howto/issues/1253) — **P3** — Verify the supported `just` version and modular recipe behavior — **PARTIAL; OPEN — Windows just 1.59.0 verified; Linux entry-point verification unavailable**

## Available for Codex now

### Code examples

#### NOW-CODE-01 — Build scripts, Cargo and procedural macros

- [#901](https://github.com/john-cd/rust_howto/issues/901) — **P3** — build_time_tooling/cc_defines1.rs: finish; deal with extern — **NOT STARTED, BLOCKED**
- [#744](https://github.com/john-cd/rust_howto/issues/744) — **P4** — Finish the Watt compile-time macro example J — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — tests, Clippy, and book build pending (skipped as requested)**
- [#1917](https://github.com/john-cd/rust_howto/issues/1917) — **P4** — Add examples: analyse and generate Rust source (syn, quote, prettyplease, cargo_metadata) — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — required validation pending (skipped as requested)**

#### NOW-CODE-02 — Foreign function interfaces

- [#1751](https://github.com/john-cd/rust_howto/issues/1751) — **P3** — Verify FFI platform prerequisites and demonstrate Python module invocation — **DEFERRED — JDK, Ruby headers, and maturin unavailable; blocker recorded on the issue**
- [#78](https://github.com/john-cd/rust_howto/issues/78) — **P4** — `python/pyo3.rs`: finish; fix py examples — **DEFERRED — maturin unavailable and PyO3 is Linux-only in this crate; blocker recorded on the issue**
- [#996](https://github.com/john-cd/rust_howto/issues/996) — **P4** — Fix and finish the Python-to-Rust example J — **DEFERRED — maturin unavailable; blocker recorded on the issue**
- [#1028](https://github.com/john-cd/rust_howto/issues/1028) — **P4** — Finish the Flutter Rust Bridge example without adding a Flutter app to the workspace J — **DEFERRED — Flutter and Dart SDKs unavailable; blocker recorded on the issue**
- [#1029](https://github.com/john-cd/rust_howto/issues/1029) — **P4** — Finish the Java JNI example and its Rust test path J — **DEFERRED — JDK unavailable; blocker recorded on the issue**
- [#1032](https://github.com/john-cd/rust_howto/issues/1032) — **P4** — Finish the Node N-API example J — **DEFERRED — requires napi-derive plus a native compiler/linker; blocker recorded on the issue**
- [#1033](https://github.com/john-cd/rust_howto/issues/1033) — **P4** — node/neon.rs: finish how to test — **DEFERRED — native compiler/linker unavailable; blocker recorded on the issue**
- [#1034](https://github.com/john-cd/rust_howto/issues/1034) — **P4** — objc/objc2.rs: finish; fix — **DEFERRED — macOS validation unavailable; blocker recorded on the issue**
- [#1035](https://github.com/john-cd/rust_howto/issues/1035) — **P4** — ruby/magnus.rs: finish — **DEFERRED — Ruby and development headers unavailable; blocker recorded on the issue**
- [#1036](https://github.com/john-cd/rust_howto/issues/1036) — **P4** — ruby/rutie.rs: finish — **DEFERRED — Ruby and development headers unavailable; blocker recorded on the issue**
- [#1037](https://github.com/john-cd/rust_howto/issues/1037) — **P4** — examples/uniffi.rs: finish — **DEFERRED — native compiler/linker unavailable; blocker recorded on the issue**

#### NOW-CODE-03 — Desktop UI, accessibility, games and GPU

- [#676](https://github.com/john-cd/rust_howto/issues/676) — **P4** — Finish the AccessKit screen-reader example I — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#767](https://github.com/john-cd/rust_howto/issues/767) — **P4** — Finish the Bevy example G — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#768](https://github.com/john-cd/rust_howto/issues/768) — **P4** — game_engines/fyrox.rs: finish — **BLOCKED — feature build cost and native platform dependencies need verification**
- [#769](https://github.com/john-cd/rust_howto/issues/769) — **P4** — Finish or remove the ggez example G — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#771](https://github.com/john-cd/rust_howto/issues/771) — **P4** — Finish the macroquad example G — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#772](https://github.com/john-cd/rust_howto/issues/772) — **P4** — Finish the wgpu compute example G — **BLOCKED — GPU adapter or configured software adapter and Naga validation dependency needed**
- [#774](https://github.com/john-cd/rust_howto/issues/774) — **P4** — text_layout/cosmic_text.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#775](https://github.com/john-cd/rust_howto/issues/775) — **P4** — Finish the Dioxus example G — **BLOCKED — SSR test requires the undeclared `dioxus-ssr` crate and feature validation**
- [#777](https://github.com/john-cd/rust_howto/issues/777) — **P4** — Finish the femtovg example G — **BLOCKED — OpenGL context and GPU/window validation needed**
- [#778](https://github.com/john-cd/rust_howto/issues/778) — **P4** — Finish the Floem example G — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#780](https://github.com/john-cd/rust_howto/issues/780) — **P4** — Finish the GTK4 example G — **BLOCKED — GTK4 development SDK is unavailable on the current Windows host**
- [#781](https://github.com/john-cd/rust_howto/issues/781) — **P4** — Finish the Iced example G — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#782](https://github.com/john-cd/rust_howto/issues/782) — **P4** — ui_layout/morphorm.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#783](https://github.com/john-cd/rust_howto/issues/783) — **P4** — text_layout/parley.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#785](https://github.com/john-cd/rust_howto/issues/785) — **P4** — file_dialogs/rfd.rs: how to test — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#786](https://github.com/john-cd/rust_howto/issues/786) — **P4** — 2d_renderers/skia_safe.rs: finish — **BLOCKED — Skia requires external binary downloads or a C++ source build**
- [#788](https://github.com/john-cd/rust_howto/issues/788) — **P4** — ui_layout/taffy.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#789](https://github.com/john-cd/rust_howto/issues/789) — **P4** — window_creation/tao.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#790](https://github.com/john-cd/rust_howto/issues/790) — **P4** — tauri/mod.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#791](https://github.com/john-cd/rust_howto/issues/791) — **P4** — 2d_renderers/vello.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#792](https://github.com/john-cd/rust_howto/issues/792) — **P4** — 2d_renderers/vger.rs: finish — **BLOCKED — upstream maintenance and WGPU API compatibility review needed**
- [#793](https://github.com/john-cd/rust_howto/issues/793) — **P4** — 2d_renderers/webrender.rs: finish — **BLOCKED — required file deletion would remove its human-only TODO Reviewed marker**
- [#794](https://github.com/john-cd/rust_howto/issues/794) — **P4** — window_creation/winit.rs: review — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#880](https://github.com/john-cd/rust_howto/issues/880) — **P4** — Finish the Crux cross-platform example I — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#1047](https://github.com/john-cd/rust_howto/issues/1047) — **P4** — 2d_renderers/main.rs: review — **BLOCKED — WebRender removal is held by the human-only marker constraint in #793**
- [#1048](https://github.com/john-cd/rust_howto/issues/1048) — **P4** — clipboard/main.rs: review — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#1052](https://github.com/john-cd/rust_howto/issues/1052) — **P4** — retained_mode_gui/vizia.rs: finish — **BLOCKED — current Vizia API differs from the requested model/lens design; feature validation needed**
- [#1056](https://github.com/john-cd/rust_howto/issues/1056) — **P4** — window_creation/baseview.rs: write; review — **BLOCKED — unpinned external Git dependency needs upstream review and feature-build validation**

#### NOW-CODE-04 — Machine learning, data and robotics

- [#764](https://github.com/john-cd/rust_howto/issues/764) — **P4** — quant/rustquant.rs: finish; review — **PARTIAL — #1283 decision keeps RustQuant out; complete the standalone example and its acceptance checks**
- [#835](https://github.com/john-cd/rust_howto/issues/835) — **P4** — Finish the Candle machine-learning example I — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#837](https://github.com/john-cd/rust_howto/issues/837) — **P4** — Finish the SmartCore machine-learning example I — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#839](https://github.com/john-cd/rust_howto/issues/839) — **P4** — geo/geo.rs: write — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#840](https://github.com/john-cd/rust_howto/issues/840) — **P4** — neuroscience/neuro.rs: write — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#843](https://github.com/john-cd/rust_howto/issues/843) — **P4** — robotics_frameworks/openrr.rs: write — **BLOCKED — upstream describes OpenRR as heavily under development and ROS 2 support as experimental; the current dependency graph includes affected `fast-float` and `serde_with` releases; confirm a portable feature path and native prerequisites**
- [#845](https://github.com/john-cd/rust_howto/issues/845) — **P4** — Add the Zenoh robotics example I — **BLOCKED — the current workspace dependency graph includes affected `lz4_flex 0.10`; feature/example registration is disabled until the advisory is resolved**
- [#885](https://github.com/john-cd/rust_howto/issues/885) — **P4** — Finish the Polars example I — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#1929](https://github.com/john-cd/rust_howto/issues/1929) — **P4** — Validate the ONNX inference example with ort A — **BLOCKED — ONNX Runtime binary download prerequisite and multi-example chapter work remain**

#### NOW-CODE-05 — Recipe batches with accompanying prose

- [#1970](https://github.com/john-cd/rust_howto/issues/1970) — **P4** — Follow up on the web examples and pages — **BLOCKED — dedicated localization crate, Leptos wasm32 dependency split, and full book/toolchain gates remain**
- [#1972](https://github.com/john-cd/rust_howto/issues/1972) — **P4** — Validate the Arrow IPC, optimization, and AWS SDK examples — **BLOCKED — MinIO/LocalStack, Windows NASM, full validation, and human review remain**
- [#1974](https://github.com/john-cd/rust_howto/issues/1974) — **P4** — Follow up on Excel, profiling, geo, audio, wry, and egui examples — **PARTIAL — egui_kittest example already exists; HRTF fixture, native audio alignment, Wry display validation, and full gates remain**
- [#1977](https://github.com/john-cd/rust_howto/issues/1977) — **P4** — Finish the remaining dev-tools, parsing, text, and testing examples — **BLOCKED — external crate integrations, rust-analyzer, and full book/toolchain validation remain; unclaimed subtasks only**
- [#1986](https://github.com/john-cd/rust_howto/issues/1986) — **P4** — Finish remaining cloud recipe batches — **BLOCKED — external services, native GUI validation, dependency work, and parent-issue coordination remain; unclaimed subtasks only**
- [#1991](https://github.com/john-cd/rust_howto/issues/1991) — **Unprioritized** — Resolve local recipe-batch follow-ups — **BLOCKED — crypto dependency mismatch, external databases, Linux, Cap'n Proto tooling, and native compilers remain**

#### NOW-CODE-06 — Examples derived from repository tools

- [#41](https://github.com/john-cd/rust_howto/issues/41) — **P4** — Make additional examples out of mdbook-utils B — **IMPLEMENTED ON TOPIC BRANCH; OPEN — offline feature scripts documented; live broken-link check omitted; focused checks, book build, and PR link pending**
- [#43](https://github.com/john-cd/rust_howto/issues/43) — **P4** — Make additional examples out of the templ tool B — **PARTIAL — document deterministic help/category_badge/rbe commands; crates.io-backed metadata commands and final gates remain**

#### NOW-CODE-07 — WebAssembly and browser applications

- [#856](https://github.com/john-cd/rust_howto/issues/856) — **P4** — yew/yew.rs: review — **IMPLEMENTED ON TOPIC BRANCH; OPEN — browser renderer gated to wasm32, native SSR test added, Trunk steps documented; focused checks and PR evidence pending**
- [#1944](https://github.com/john-cd/rust_howto/issues/1944) — **P4** — Add examples: Leptos web app building blocks (leptos_router, gloo-net, gloo-storage, icons, gRPC-Web client) — **BLOCKED — route/fetch/storage/gRPC examples exist; icondata example, Leptos browser dependency split tracked by #1970, and wasm/book validation remain**

#### NOW-CODE-08 — Media encoding, images and video

- [#1943](https://github.com/john-cd/rust_howto/issues/1943) — **P4** — Add examples: audio files, playback and MIDI (hound, rodio with a custom Source, midir, hrtf) — **BLOCKED ON FINAL VALIDATION — WAV, Rodio, MIDI, and HRTF examples exist; book refdefs and build checks remain (Cargo test/Clippy evidence is Linux-only and requires ALSA development files)**

#### NOW-CODE-09 — Networking, authentication and email

- [#867](https://github.com/john-cd/rust_howto/issues/867) — **P3** — examples/leptos.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — optional feature now enables Leptos CSR; feature build/test and book validation pending**
- [#1939](https://github.com/john-cd/rust_howto/issues/1939) — **P3** — Add examples: axum add-ons (axum-extra Host and TypedHeader, axum-server TLS, flash messages, #[debug_handler], tower-cookies) — **BLOCKED ON FINAL VALIDATION — examples, recipe page, and compile-fail tip exist in the checkout; Cargo, Clippy, format, refdefs, and book gates remain**
- [#2017](https://github.com/john-cd/rust_howto/issues/2017) — **P3** — Add an offline-testable OAuth 2.0 client-credentials example — **IMPLEMENTED ON DEVELOP; BLOCKED — Windows auth test fails building aws-lc-sys 0.46.0 (NASM absent; cl.exe alignment probe/LNK1114); retry on a supported host**
- [#2018](https://github.com/john-cd/rust_howto/issues/2018) — **P3** — Demonstrate mutual TLS with a client certificate — **IMPLEMENTED ON DEVELOP; BLOCKED ON BOOK BUILD — TLS tests/Clippy and Markdown/refdefs pass; just bb cannot start Git Bash (NtCreateDirectoryObject 0xC0000022)**

#### NOW-CODE-10 — Cloud services and SDKs

- [#1913](https://github.com/john-cd/rust_howto/issues/1913) — **P4** — Add examples: message bus over AWS SQS/SNS with Protobuf payloads (aws-sdk, prost) — **BLOCKED ON FINAL VALIDATION — implementation and offline Cargo evidence are in merged PR #1971; cross-links, book refdefs/build, and optional LocalStack coverage remain**

#### NOW-CODE-11 — Tabular data and finance

- [#1909](https://github.com/john-cd/rust_howto/issues/1909) — **P3** — Add examples: read and write Excel files (calamine, rust_xlsxwriter) — **BLOCKED — examples and refs are in merged PR #1973; the finance index target from #365 is absent, and local Cargo/refdefs/book gates remain**

#### NOW-CODE-12 — Embedded systems and hardware interfaces

- [#751](https://github.com/john-cd/rust_howto/issues/751) — **P4** — embassy/embassy.rs: finish — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — embedded all-target Clippy and tests pass; book gate pending**

### Prose and book structure

#### NOW-PROSE-01 — Systems, embedded, WebAssembly and virtualization

- [#348](https://github.com/john-cd/rust_howto/issues/348) — **P4** — emulators/emulators.md: write — **PARTIAL ON ISSUE BRANCH; emulator core-loop change overlaps open #752; final validation pending**
- [#349](https://github.com/john-cd/rust_howto/issues/349) — **P4** — emulators/index.md: write — **IMPLEMENTED ON ISSUE BRANCH; OPEN — final validation pending**
- [#426](https://github.com/john-cd/rust_howto/issues/426) — **P4** — no-std/no_std.md: no_std: write — **IMPLEMENTED ON ISSUE BRANCH; OPEN — final validation and issue-link removal pending**
- [#428](https://github.com/john-cd/rust_howto/issues/428) — **P4** — no-std_no-alloc/no_alloc.md: no_alloc: write — **IMPLEMENTED ON ISSUE BRANCH; OPEN — final validation and issue-link removal pending**
- [#433](https://github.com/john-cd/rust_howto/issues/433) — **P4** — os_freebsd-apis/freebsd.md: freebsd: write — **PARTIAL ON ISSUE BRANCH; #817 sysctl example and FreeBSD CI check remain; final validation pending**
- [#434](https://github.com/john-cd/rust_howto/issues/434) — **P4** — os_linux-apis/linux.md: linux: write — **PARTIAL ON ISSUE BRANCH; #818 inotify/statx example and final validation pending**
- [#435](https://github.com/john-cd/rust_howto/issues/435) — **P4** — os_macos-apis/macos.md: macos: write — **PARTIAL ON ISSUE BRANCH; #819 legacy example and macOS CI inclusion remain; final validation pending**
- [#496](https://github.com/john-cd/rust_howto/issues/496) — **P4** — wasm/wasm_standalone_runtimes.md: others: organize/write — **DEFERRED — current support checks for six runtimes and Wasmer/Wasmtime example builds/tests are held for the final validation pass; blocker details are on the issue**
- [#498](https://github.com/john-cd/rust_howto/issues/498) — **P4** — wasm/yew.md: yew: write / organize — **IMPLEMENTED ON ISSUE BRANCH; final validation pending**
- [#1217](https://github.com/john-cd/rust_howto/issues/1217) — **P4** — `wasm/wasm_development.md`: write — **IMPLEMENTED ON ISSUE BRANCH; final validation pending**
- [#1218](https://github.com/john-cd/rust_howto/issues/1218) — **P4** — `wasm/interfacing_with_javascript.md`: write — **IMPLEMENTED ON ISSUE BRANCH; final validation pending**

#### NOW-PROSE-02 — UI, accessibility, media and rendering

- [#367](https://github.com/john-cd/rust_howto/issues/367) — **P4** — game-development/index.md: expand — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#369](https://github.com/john-cd/rust_howto/issues/369) — **P4** — game-engines/game_engines.md: game_engines: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#375](https://github.com/john-cd/rust_howto/issues/375) — **P4** — rendering_graphics-api/gpu_abstraction_layers.md: webgpu: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#377](https://github.com/john-cd/rust_howto/issues/377) — **P4** — rendering/2d_renderers.md: 2d_renderers: write; titles — **BLOCKED — #792 Vger status/compatibility and #793 WebRender disposition unresolved**
- [#381](https://github.com/john-cd/rust_howto/issues/381) — **P4** — gui/file_dialogs.md: file_dialogs: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#383](https://github.com/john-cd/rust_howto/issues/383) — **P4** — gui/gtk.md: gtk: write — **BLOCKED — GTK 4 development SDK and pkg-config metadata unavailable**
- [#385](https://github.com/john-cd/rust_howto/issues/385) — **P4** — gui/immediate_mode_gui.md: immediate_mode_gui: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#389](https://github.com/john-cd/rust_howto/issues/389) — **P4** — gui/retained_mode_gui.md: retained_mode_gui: organize/write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — book and reference checks and the Slint-feature check remain; stale undeclared Tauri build-hook reference removed**
- [#391](https://github.com/john-cd/rust_howto/issues/391) — **P4** — gui/text_layout.md: text_layout: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#394](https://github.com/john-cd/rust_howto/issues/394) — **P4** — gui/web_based_gui.md: web_based_gui: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#396](https://github.com/john-cd/rust_howto/issues/396) — **P4** — Write the window creation chapter G — **BLOCKED — #1056 baseview disposition and new softbuffer dependency review pending**
- [#413](https://github.com/john-cd/rust_howto/issues/413) — **P4** — multimedia/multimedia.md: multimedia: write — **BLOCKED — #805 playback example is unfinished; a distinct cross-media scope remains**
- [#417](https://github.com/john-cd/rust_howto/issues/417) — **P4** — multimedia_encoding/encoding.md: encoding: write — **BLOCKED — symphonia, hound, and mp4 dependencies are not in the workspace**
- [#418](https://github.com/john-cd/rust_howto/issues/418) — **P4** — multimedia_encoding/index.md: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#420](https://github.com/john-cd/rust_howto/issues/420) — **P4** — multimedia_images/images.md: images: write — **BLOCKED — required example split and EXIF/oxipng dependency review in #808 are incomplete**
- [#455](https://github.com/john-cd/rust_howto/issues/455) — **P4** — rendering_engine/rendering_engines.md: rendering_engines: write — **BLOCKED — #955 category pointer decision and #830 example disposition remain unresolved**
- [#457](https://github.com/john-cd/rust_howto/issues/457) — **P4** — rendering_graphics-api/native_graphics_apis.md: graphics_apis: write — **BLOCKED — required wgpu adapter-info example in #831 remains a stub**
- [#608](https://github.com/john-cd/rust_howto/issues/608) — **P4** — gpu/index.md: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#934](https://github.com/john-cd/rust_howto/issues/934) — **P4** — gui/ui_layout.md: review — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — focused GUI all-target Clippy passes; book and reference checks remain**
- [#954](https://github.com/john-cd/rust_howto/issues/954) — **P4** — rendering_data-formats/index.md: review — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#1229](https://github.com/john-cd/rust_howto/issues/1229) — **P4** — rendering_graphics-api/opengl.md: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#1999](https://github.com/john-cd/rust_howto/issues/1999) — **Unprioritized** — categories/multimedia: write a streaming media page (RTSP, WebRTC, RTP) — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**

#### NOW-PROSE-03 — Science, finance, data processing and visualization

- [#257](https://github.com/john-cd/rust_howto/issues/257) — **P4** — computer-vision/opencv.md: opencv: expand, add example — **BLOCKED — OpenCV/libclang SDKs unavailable in this environment**
- [#364](https://github.com/john-cd/rust_howto/issues/364) — **P4** — finance/quant.md: quant: write — **DEFERRED — requested `financial`, `rand`, and `rand_distr` examples require new dependencies; skipped under the no-external-dependency constraint**
- [#365](https://github.com/john-cd/rust_howto/issues/365) — **P4** — `finance/index.md`: write — **PARTIAL — #1283 decision keeps RustQuant out; remaining index work and checks require review**
- [#472](https://github.com/john-cd/rust_howto/issues/472) — **P4** — science/index.md: write — **IMPLEMENTED ON DEVELOP; OPEN — final validation and PR link pending**
- [#474](https://github.com/john-cd/rust_howto/issues/474) — **P4** — science_geo/geo.md: geo: write — **BLOCKED — GeoJSON dependencies and PROJ C library required**
- [#476](https://github.com/john-cd/rust_howto/issues/476) — **P4** — science_neuroscience/neuroscience.md: neuro: write — **IMPLEMENTED ON DEVELOP; final validation pending**
- [#477](https://github.com/john-cd/rust_howto/issues/477) — **P4** — science_robotics/robot_operating_systems.md: robotics: organize — **BLOCKED — ROS 2 client/example issue #844 pending**
- [#479](https://github.com/john-cd/rust_howto/issues/479) — **P4** — science_robotics/robotics_frameworks.md: useful_robotics_tools_and_libs: locate libs, organize, write — **IMPLEMENTED ON DEVELOP; OPEN — comparison page and references complete; final validation and PR link pending; Zenoh/OpenRR examples remain blocked under #845/#843**
- [#480](https://github.com/john-cd/rust_howto/issues/480) — **P4** — science_robotics/index.md: organize — **IMPLEMENTED ON DEVELOP; OPEN — final validation and PR link pending**
- [#481](https://github.com/john-cd/rust_howto/issues/481) — **P4** — simulation/simulation.md: simulation: write — **IMPLEMENTED ON DEVELOP; OPEN — final validation and PR link pending**
- [#494](https://github.com/john-cd/rust_howto/issues/494) — **P4** — `visualization/visualization.md`: visualization: write — **BLOCKED — Plotters dependency required for SVG example**
- [#2019](https://github.com/john-cd/rust_howto/issues/2019) — **P4** — Add dependency-free numerical integration recipes — **IMPLEMENTED ON DEVELOP; OPEN — focused tests, Clippy, reference checks, and book validation deferred to the final pass**
- [#587](https://github.com/john-cd/rust_howto/issues/587) — **P4** — data-processing/dataframes.md: dataframes: organize / edit — **IMPLEMENTED ON DEVELOP; OPEN — final validation and PR link pending**
- [#1207](https://github.com/john-cd/rust_howto/issues/1207) — **P4** — science_robotics/perception_and_sensors.md: write — **PARTIAL ON DEVELOP; BLOCKED — `kiddo` example dependency unavailable**
- [#1210](https://github.com/john-cd/rust_howto/issues/1210) — **P4** — science_robotics/simulation_visualization.md: write — **IMPLEMENTED ON DEVELOP; OPEN — final validation and PR link pending**
- [#1211](https://github.com/john-cd/rust_howto/issues/1211) — **P4** — science_robotics/hardware_integration.md: write — **PARTIAL ON DEVELOP; BLOCKED — `embedded-hal-mock` example dependency unavailable**

#### NOW-PROSE-04 — Publishing, curation and book coverage

- [#1720](https://github.com/john-cd/rust_howto/issues/1720) — **P3** — Curate remaining VS Code, browser and TOREVIEW bookmarks into book content — **BLOCKED — the Chrome bookmarks export (`bookmarks.json`) is absent from this checkout; request the source before further curation**
- [#1957](https://github.com/john-cd/rust_howto/issues/1957) — **Unprioritized** — Audit references and links in restored book pages A — **BLOCKED — restored-page reference and wikilink audits remain; the requested `bk/drafts` and `later` book builds lack standalone book configurations, as documented in the issue comment**

#### NOW-PROSE-05 — Cloud platforms and containers

- [#574](https://github.com/john-cd/rust_howto/issues/574) — **P4** — cloud/aws.md: aws: write — **IMPLEMENTED — pending final crate and book validation**
- [#576](https://github.com/john-cd/rust_howto/issues/576) — **P4** — cloud/rust_native_cloud_development.md: write — **BLOCKED — Shuttle's official notice says hosted operations are ceasing and its old pricing is stale; the Shuttle-specific example needs a replacement target decision**
- [#988](https://github.com/john-cd/rust_howto/issues/988) — **P4** — virtualization/containers.md: review/write — **PARTIAL — `containerization.md` is nested under Containers; blocked on #1224's missing Docker client/test dependencies before adding the thin `using_containers.md` sibling and finalizing the section structure**
- [#1224](https://github.com/john-cd/rust_howto/issues/1224) — **P4** — virtualization/using_containers.md: write / fix — **BLOCKED — required `bollard` and `testcontainers` dependencies are absent; the integration also requires a Docker daemon, as noted in the issue comment**
- [#1225](https://github.com/john-cd/rust_howto/issues/1225) — **P4** — virtualization/containerization.md: write — **IMPLEMENTED — pending final Markdown/book validation**

#### NOW-PROSE-06 — Security and cryptocurrencies

- [#1179](https://github.com/john-cd/rust_howto/issues/1179) — **P3** — cryptography/certificates.md: write — **IMPLEMENTED — pending final cryptography crate, draft book, reference, spelling, and diff validation**

#### NOW-PROSE-07 — Foreign function interfaces - IN PROGRESS

- [#355](https://github.com/john-cd/rust_howto/issues/355) — **P4** — external-ffi-bindings/external_ffi_bindings.md: external_ffi_bindings: write — **IMPLEMENTED; OPEN — Windows checks passed; Unix run, full book gate and PR link pending**
- [#1070](https://github.com/john-cd/rust_howto/issues/1070) — **P4** — development-tools_ffi/erlang_elixir.md: write — **IMPLEMENTED; OPEN — focused checks passed; full book gate and PR link pending**
- [#1071](https://github.com/john-cd/rust_howto/issues/1071) — **P4** — development-tools_ffi/flutter.md: write; review in depth — **IMPLEMENTED AND REVIEWED; OPEN — focused checks passed; full book gate and PR link pending**
- [#1072](https://github.com/john-cd/rust_howto/issues/1072) — **P4** — development-tools_ffi/java.md: write — **IMPLEMENTED; OPEN — focused checks passed; full book gate and PR link pending**
- [#1074](https://github.com/john-cd/rust_howto/issues/1074) — **P4** — development-tools_ffi/node.md: write — **IMPLEMENTED; OPEN — focused checks passed; full book gate and PR link pending**
- [#1075](https://github.com/john-cd/rust_howto/issues/1075) — **P4** — development-tools_ffi/objc.md: write — **IMPLEMENTED; OPEN — Windows checks passed; native runtime, full book gate and PR link pending**
- [#1076](https://github.com/john-cd/rust_howto/issues/1076) — **P4** — development-tools_ffi/ruby.md: write — **IMPLEMENTED; OPEN — Windows checks passed; native runtime, full book gate and PR link pending**

#### NOW-PROSE-08 — Web, networking and databases

- [#288](https://github.com/john-cd/rust_howto/issues/288) — **P3** — database/search.md: expand database search guidance — **IMPLEMENTED ON LOCAL DEVELOP — focused database Clippy/nextest passed; full book rendering blocked by MSYS/AWS-LC; PR link required**
- [#290](https://github.com/john-cd/rust_howto/issues/290) — **P3** — database-implementations/databases.md: databases: expand / write — **BLOCKED — requires adding and validating a new `redb` or `fjall` dependency and tempfile-backed example; deferred per request**
- [#292](https://github.com/john-cd/rust_howto/issues/292) — **P3** — database-implementations/index.md: database-implementations: expand — **IMPLEMENTED ON LOCAL DEVELOP — draft lint/spelling passed; full book rendering blocked by MSYS/AWS-LC; PR link required**
- [#342](https://github.com/john-cd/rust_howto/issues/342) — **P3** — email/send_emails.md: send_emails: write — **IMPLEMENTED ON LOCAL DEVELOP — draft lint/spelling and generated crate references passed; full book rendering blocked by MSYS/AWS-LC; PR link required**
- [#343](https://github.com/john-cd/rust_howto/issues/343) — **P3** — email/index.md: write — **IMPLEMENTED ON LOCAL DEVELOP — draft lint/spelling passed; full book rendering blocked by MSYS/AWS-LC; PR link required**
- [#509](https://github.com/john-cd/rust_howto/issues/509) — **P3** — web-programming_http-server/_batteries-included_frameworks.md: batteries-included_frameworks: write — **IMPLEMENTED ON LOCAL DEVELOP — draft lint/spelling passed; full book rendering blocked by MSYS/AWS-LC; PR link required**
- [#518](https://github.com/john-cd/rust_howto/issues/518) — **P3** — web-programming_http-server/other_frameworks.md: other_frameworks: organize — **IMPLEMENTED ON LOCAL DEVELOP — draft lint/spelling passed; full book rendering blocked by MSYS/AWS-LC; PR link required**
- [#1065](https://github.com/john-cd/rust_howto/issues/1065) — **P3** — database/index.md: organize — **IMPLEMENTED ON LOCAL DEVELOP — draft lint/spelling passed; full book rendering blocked by MSYS/AWS-LC; PR link required**
- [#1067](https://github.com/john-cd/rust_howto/issues/1067) — **P3** — database/mssql.md: organize / write — **BLOCKED — Docker CLI is installed, but Docker API access fails with `permission denied`; SQL Server service validation is unavailable**
- [#1068](https://github.com/john-cd/rust_howto/issues/1068) — **P3** — database/nosql.md: review — **IMPLEMENTED ON LOCAL DEVELOP — focused database Clippy and 9/9 portable tests passed (4 external-service tests skipped); draft lint/spelling and generated crate refs passed; full book gate and PR link pending**
- [#1069](https://github.com/john-cd/rust_howto/issues/1069) — **P3** — database/oracle.md: write — **BLOCKED — Oracle Instant Client is unavailable (`OCI_LIB_DIR` unset and `sqlplus` absent); Docker API access is denied, so the required local Oracle service cannot be validated**

Validation (2026-10-27, NOW-PROSE-08): focused database Clippy and nextest passed; the test run passed 9 tests and skipped four ignored external-service tests. `dprint check`, `rumdl` across `bk/drafts`, `typos` across the book and future content, and the generated dependency-reference check passed. The local draft book gate remains blocked: Git Bash cannot create its MSYS `BaseNamedObjects` directory, the direct tools release build fails in `aws-lc-sys` with MSVC LNK1114/error 5, and Docker API access is denied. The Windows `just refdefs check` recipe is unavailable; its Linux-only script could not run. Issues remain open because full book/reference gates and PR links are not complete.

#### NOW-PROSE-09 — Development tools, build systems and editors — IMPLEMENTED; FULL VALIDATION BLOCKED

- [#600](https://github.com/john-cd/rust_howto/issues/600) — **P4** — devops/github_actions.md: complete Rust CI, test matrices, tool installation, caching, and security guidance — **IMPLEMENTED; INTEGRATED LOCALLY** (`a499e04d7`).
- [#615](https://github.com/john-cd/rust_howto/issues/615) — **P4** — written-in-rust/other_tools.md: group tools into tables, preserve anchors, remove duplicate entries and the ChatGPT bookmark — **IMPLEMENTED; INTEGRATED LOCALLY** (`2a62690df`).

Validation (2026-10-26): dprint, rumdl, `just spell`, changed-page references and local fragments, focused mdBook rendering, rendered tables/anchors, and `git diff --check` passed. All three YAML workflows passed Zizmor's offline audit. `just bb` is blocked before rendering because `cargo tree --locked` needs to update the existing `bk/crates/Cargo.lock`; no Cargo inputs changed in this batch. `just refdefs check` is Unix-only; running its script through Git Bash reports existing unresolved global labels, while all labels used by the two completed chapters resolve. Issues remain open pending the full book gate. Commits are local; no push was requested.

#### NOW-PROSE-10 — Localization and internationalization - IN PROGRESS

- [#404](https://github.com/john-cd/rust_howto/issues/404) — **P4** — localization/localization.md: localization: write — **IMPLEMENTED ON DEVELOP; OPEN — PR link pending**
- [#405](https://github.com/john-cd/rust_howto/issues/405) — **P4** — localization/index.md: write — **IMPLEMENTED ON DEVELOP; OPEN — PR link pending**

#### NOW-PROSE-11 — Asynchronous programming and caching - IN PROGRESS

- [#269](https://github.com/john-cd/rust_howto/issues/269) — **P3** — concurrency/_actors.md: _actors: organize — **IMPLEMENTED ON DEVELOP; OPEN — full gates and PR pending**
  - Focused actor Clippy, four actor tests, dprint, rumdl, chapter spelling, and direct refdefs check pass. `just ca`, `just ntt actors`, `just bb`, and `just spell` stop at the Git Bash `NtCreateDirectoryObject` access error; `just refdefs check` is not a defined recipe. Direct all-target Clippy also reports a pre-existing semicolon lint in `message_passing_crossbeam_channel.rs`.

#### NOW-PROSE-12 — Aerospace and device protocols — IMPLEMENTED; FULL VALIDATION BLOCKED

- [#190](https://github.com/john-cd/rust_howto/issues/190) — **P4** — aerospace/index.md: review — **IMPLEMENTED ON DEVELOP; OPEN — required book gates and PR link pending**
- [#193](https://github.com/john-cd/rust_howto/issues/193) — **P4** — aerospace_drones/index.md: write — **IMPLEMENTED ON DEVELOP; OPEN — required book gates and PR link pending**
- [#196](https://github.com/john-cd/rust_howto/issues/196) — **P4** — aerospace_protocols/index.md: aerospace protocols: write — **IMPLEMENTED ON DEVELOP; OPEN — required book gates and PR link pending**
- [#199](https://github.com/john-cd/rust_howto/issues/199) — **P4** — aerospace_simulation/aerospace_simulation.md: aerospace_simulation: write — **IMPLEMENTED ON DEVELOP; OPEN — required book gates and PR link pending**
- [#201](https://github.com/john-cd/rust_howto/issues/201) — **P4** — aerospace_space-protocols/space_protocols.md: space_protocols: write — **IMPLEMENTED ON DEVELOP; OPEN — required book gates and PR link pending**
- [#202](https://github.com/john-cd/rust_howto/issues/202) — **P4** — aerospace_space-protocols/index.md: write — **IMPLEMENTED ON DEVELOP; OPEN — required book gates and PR link pending**

Validation (2026-10-26): Markdown formatting, lint, spelling, and whitespace checks passed. Focused tests and all-target Clippy passed for the aerospace simulation and space-protocol examples. The complete book/reference gates remain blocked: the direct reference check reaches `bk/crates` but `cargo tree --locked` requires an existing lockfile update; Git Bash-backed `just bb` cannot start because of the `NtCreateDirectoryObject` access denial. A direct mdBook build reports repository-wide unresolved links, and optional `mdbook-pandoc` is unavailable. Issues remain open pending complete book gates and PR links.

### Tools and infrastructure

#### NOW-INFRA-01 — Preprocessors, references and recipe portability

- [#248](https://github.com/john-cd/rust_howto/issues/248) — **P1** — replace current badge markdown copy-pasted text by a preprocessor command {#crate ...} {#categories... } — **BLOCKED — waiting on full-page parser fixes in external book_tooling#34/#47 and rendered publication validation**
  - The sorting-table migration and representative rendering are integrated; broad filesystem-page conversion was reverted after raw badge directives remained in output. Resume after the parser fix is available, then verify directive expansion, rendered output parity, index anchors, and idempotence before closing.

#### NOW-INFRA-02 — Dependencies, CI and containers

- [#1998](https://github.com/john-cd/rust_howto/issues/1998) — **P1** — Port remaining examples across major dependency upgrades — **PARTIAL** — upstream-blocked subset

#### NOW-INFRA-03 — Branch integration and retained work

- [#1992](https://github.com/john-cd/rust_howto/issues/1992) — **P1** — Triage and integrate unique work from retained example branches — **BLOCKED — the three `claude/examples-*` refs are absent locally and from current GitHub/remote branch listings; owner confirmation or commit IDs are needed before diffing or integrating**

#### NOW-INFRA-04 — Windows validation and tooling repairs

- [#1983](https://github.com/john-cd/rust_howto/issues/1983) — **P3** — Finish Windows validation and reference-generator repairs — **BLOCKED FOR COMPLETION — Windows full-suite/linker failures remain; Oracle Instant Client and a compatible OpenTelemetry release are unavailable; reference generation and book-link checks are unfinished**

#### NOW-INFRA-05 — Contributor hooks and container setup

- [#1247](https://github.com/john-cd/rust_howto/issues/1247) — **P3** — precommit/mod.just: move to Dockerfile — **BLOCKED — waiting for the human tool choice in prerequisite issue #1245 (pre-commit vs. cargo-husky)**

### Roadmaps and epics

#### NOW-EPIC-01 — Build and repository maintenance

- [#1997](https://github.com/john-cd/rust_howto/issues/1997) — **P3** — Coordinate tooling, CI, and repository maintenance — **OPEN TRACKER**

#### NOW-EPIC-02 — Unfinished example coverage

- [#1995](https://github.com/john-cd/rust_howto/issues/1995) — **P4** — Coordinate unfinished-example completion — **OPEN TRACKER**

#### NOW-EPIC-03 — Draft chapter coverage

- [#1993](https://github.com/john-cd/rust_howto/issues/1993) — **P4** — Coordinate draft-chapter completion — **OPEN TRACKER**

#### NOW-EPIC-04 — New recipe coordination

- [#1955](https://github.com/john-cd/rust_howto/issues/1955) — **P3** — Coordinate new recipes from the local-project review — **OPEN TRACKER**

#### NOW-EPIC-05 — First-edition roadmap

- [#1698](https://github.com/john-cd/rust_howto/issues/1698) — **P1** — Complete the first edition and integrate the companion tools — **OPEN TRACKER**

## Validation prerequisites not ready

### Code examples

#### VALIDATE-CODE-01 — Networking, authentication and email

- [#225](https://github.com/john-cd/rust_howto/issues/225) — **P3** — Offline Basic authentication example F — **IMPLEMENTED ON DEVELOP; BLOCKED ON FULL GATES — focused offline tests, Clippy, and formatting passed; `just ca` needs a Cargo.lock update and `just bb` is blocked by the Windows AWS-LC/NASM build; details are on the issue**
- [#810](https://github.com/john-cd/rust_howto/issues/810) — **P3** — Run native Linux tests and strict Clippy for the Glommio example — **IMPLEMENTED ON DEVELOP; BLOCKED — Linux-only tests/Clippy cannot run here; WSL startup returns `Wsl/E_ACCESSDENIED`; the issue comment records the native Linux follow-up**
- [#1145](https://github.com/john-cd/rust_howto/issues/1145) — **P3** — `websocket/tokio_tungstenite.rs`: review https://github.com/snapview/tokio-tungstenite/tree/master/examples — **IMPLEMENTED ON DEVELOP; OPEN — local echo client/server and loopback test are present (`8c757a2c0`); prior focused tests/format passed; full Clippy blocked by Windows `aws-lc-sys`/`naga`; PR link pending**
- [#1910](https://github.com/john-cd/rust_howto/issues/1910) — **P3** — Add examples: web login with axum-login, tower-sessions and Argon2 password hashing — **IMPLEMENTED ON DEVELOP; OPEN — login/hash/CSRF and permission examples, HTTPS redirect recipe, and session-fixation guidance are present; formatted; focused tests, Clippy, refdefs, and book build deferred to final validation**
- [#1911](https://github.com/john-cd/rust_howto/issues/1911) — **P3** — Add examples: client-side rate limiting and retries (governor, backon/backoff) — **IMPLEMENTED ON DEVELOP; OPEN — governor/backon examples, fake-clock and mock HTTP coverage, Retry-After handling, combined quota/retry guidance, page rows, and refdefs are present; formatted; focused tests, Clippy, refdefs, and book build deferred to final validation**
- [#1945](https://github.com/john-cd/rust_howto/issues/1945) — **P3** — Add examples: gRPC-Web with tonic and protobuf codegen without a system protoc (tonic-web, tonic-prost-build, protobuf-src) — **IMPLEMENTED ON DEVELOP; OPEN — gRPC-Web example, protox codegen, protocol comparison, page rows, and refdefs are present; corrected browser streaming and Envoy guidance; formatted; focused tests, Clippy, refdefs, and book build deferred to final validation**
- [#1946](https://github.com/john-cd/rust_howto/issues/1946) — **P3** — Add examples: read mail over IMAP and filter disposable addresses before sending (imap, mailchecker) — **IMPLEMENTED ON DEVELOP; OPEN — IMAP/parser and mailchecker examples, pages, tables, and refdefs are present; live IMAP requires a server and credentials; offline tests, Clippy, refdefs, and book build deferred to final validation; details are on the issue**
- [#1916](https://github.com/john-cd/rust_howto/issues/1916) — **P4** — Add examples: OpenAPI docs with utoipa and JSON Schema generation with schemars — **IMPLEMENTED ON DEVELOP; OPEN — Swagger UI/spec routes, schema validation, book page/table/refdefs, and CI/versioning prose are present; schema errors propagate instead of silently defaulting; formatted; focused tests, Clippy, refdefs, and book build deferred to final validation**

#### VALIDATE-CODE-02 — Databases and service clients

- [#1017](https://github.com/john-cd/rust_howto/issues/1017) — **P2** — Finish the cdrs_tokio example — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused tests/Clippy passed; ignored CRUD test needs Cassandra; full native-dependency/book gates remain; details are on the issue**
- [#1022](https://github.com/john-cd/rust_howto/issues/1022) — **P2** — Finish the Sibyl example and review installation steps — **IMPLEMENTED ON DEVELOP; BLOCKED — Oracle Instant Client is unavailable (`OCI_LIB_DIR` required); Linux/Windows installation and Oracle service checks remain; details are on the issue**
- [#1148](https://github.com/john-cd/rust_howto/issues/1148) — **P2** — Review the SurrealDB example — **IMPLEMENTED ON DEVELOP; BLOCKED — prior embedded-Mem test and focused Clippy passed; full book/native-dependency gates remain unresolved; details are on the issue**
- [#1162](https://github.com/john-cd/rust_howto/issues/1162) — **P2** — Review the PostgreSQL aggregate_data example — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused target tests/Clippy passed; the aggregate query test needs configured PostgreSQL; full book/native-dependency gates remain; details are on the issue**
- [#708](https://github.com/john-cd/rust_howto/issues/708) — **P3** — Finish the Cornucopia example B — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused build/tests/Clippy and live PostgreSQL checks passed; full Clippy/book gates remain unresolved; details are on the issue**

#### VALIDATE-CODE-03 — Compression and archives

- [#254](https://github.com/john-cd/rust_howto/issues/254) — **P3** — `tar/tar_decompress.rs`: review; tar_decompress.rs is noplayground - fix? — **IMPLEMENTED ON DEVELOP; OPEN — explicit archive/destination paths, self-contained extraction test with automatic tempfile cleanup, and noplayground rationale are present; formatted; tests, Clippy, and book checks deferred to final validation**
- [#255](https://github.com/john-cd/rust_howto/issues/255) — **P3** — `tar/tar_compress.rs`: review; tar_compress.rs is noplayground - fix? — **IMPLEMENTED ON DEVELOP; OPEN — explicit paths, finished gzip stream, archive-content test with automatic tempfile cleanup, and noplayground rationale are present; formatted; tests, Clippy, and book checks deferred to final validation**

#### VALIDATE-CODE-04 — Build scripts, Cargo and procedural macros

- [#897](https://github.com/john-cd/rust_howto/issues/897) — **P3** — Complete the C++ linkage example — **IMPLEMENTED ON DEVELOP; BLOCKED — package linkage/test stops in vendored libgit2-sys (`LNK1114`); full Clippy/book gates remain; details are on the issue**
- [#898](https://github.com/john-cd/rust_howto/issues/898) — **P3** — Complete the C++ Rust wrapper example — **IMPLEMENTED ON DEVELOP; BLOCKED — package linkage/test stops in vendored libgit2-sys (`LNK1114`); full Clippy/book gates remain; details are on the issue**
- [#899](https://github.com/john-cd/rust_howto/issues/899) — **P3** — Complete the C static-library example — **IMPLEMENTED ON DEVELOP; BLOCKED — package linkage/test stops in vendored libgit2-sys (`LNK1114`); full Clippy/book gates remain; details are on the issue**
- [#900](https://github.com/john-cd/rust_howto/issues/900) — **P3** — Complete the C Rust wrapper example — **IMPLEMENTED ON DEVELOP; BLOCKED — package linkage/test stops in vendored libgit2-sys (`LNK1114`); full Clippy/book gates remain; details are on the issue**
- [#1000](https://github.com/john-cd/rust_howto/issues/1000) — **P3** — `build_time_tooling/cc_defines.rs`: finish; deal wth cc — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused tests/Clippy and direct mdBook build passed; required book wrapper gate and issue-linked PR remain; details are on the issue**
- [#1060](https://github.com/john-cd/rust_howto/issues/1060) — **P3** — `buildinfo/build-info.rs`: review / convert into example — **IMPLEMENTED ON DEVELOP; BLOCKED — anchored build_info example is present; libgit2-sys/native builds block focused/full gates; issue-linked PR remains; details are on the issue**
- [#1951](https://github.com/john-cd/rust_howto/issues/1951) — **P3** — Cargo subcommand and coverage examples — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused Cargo/CLI tests, Clippy, and Markdown checks passed; full native-dependency/book gates remain; details are on the issue**
- [#739](https://github.com/john-cd/rust_howto/issues/739) — **P4** — Finish the `darling` procedural-macro helper example J — **IMPLEMENTED ON DEVELOP; BLOCKED — prior helper-package tests/strict Clippy and Markdown checks passed; required book gate stops in Windows AWS-LC; details are on the issue**
- [#741](https://github.com/john-cd/rust_howto/issues/741) — **P4** — Finish the `proc_macro2` helper example J — **IMPLEMENTED ON DEVELOP; BLOCKED — prior helper-package tests/strict Clippy and Markdown checks passed; required book gate stops in Windows AWS-LC; details are on the issue**
- [#742](https://github.com/john-cd/rust_howto/issues/742) — **P4** — Review and expand the `quote` helper example J — **IMPLEMENTED ON DEVELOP; BLOCKED — prior helper-package tests/strict Clippy and Markdown checks passed; required book gate stops in Windows AWS-LC; details are on the issue**
- [#743](https://github.com/john-cd/rust_howto/issues/743) — **P4** — Finish the `syn` procedural-macro helper example J — **IMPLEMENTED ON DEVELOP; BLOCKED — prior helper-package tests/strict Clippy and Markdown checks passed; required book gate stops in Windows AWS-LC; details are on the issue**

#### VALIDATE-CODE-05 — Foreign function interfaces

- [#738](https://github.com/john-cd/rust_howto/issues/738) — **P4** — CXX bridge F — **IMPLEMENTED ON DEVELOP; BLOCKED — prior CXX example tests and strict Clippy passed; required book gate stops in Windows AWS-LC/NASM/MSVC; details are on the issue**

#### VALIDATE-CODE-06 — Desktop UI, accessibility, games and GPU

- [#1155](https://github.com/john-cd/rust_howto/issues/1155) — **P3** — `clipboard/arboard.rs`: review — **IMPLEMENTED ON DEVELOP; BLOCKED — prior GUI package tests passed; ignored desktop clipboard run, full Clippy, and issue-linked PR remain; details are on the issue**
- [#1933](https://github.com/john-cd/rust_howto/issues/1933) — **P4** — Add examples: egui add-on crates (tables, plots, docking tiles, toasts, file dialog, graphs, Markdown, icons, log viewer) — **IMPLEMENTED ON DEVELOP; OPEN — add-on examples, headless coverage, and book page/table are present; prior cloud tests/Clippy are recorded in the issue; refdefs/book checks deferred to final validation**
- [#1952](https://github.com/john-cd/rust_howto/issues/1952) — **P4** — Add example: show a local web page in a native window (wry, tao) — **IMPLEMENTED ON DEVELOP; OPEN — native webview example, URL allow-list coverage, and platform prose are present; manual window run and refdefs/book checks remain; details are on the issue**

#### VALIDATE-CODE-07 — Machine learning, data and robotics

- [#1079](https://github.com/john-cd/rust_howto/issues/1079) — **P4** — Review and expand the OpenCV example — **IMPLEMENTED ON DEVELOP; BLOCKED — Linux/OpenCV/libclang tests and Clippy require a Linux host; WSL access and Docker Linux engine are unavailable; details are on the issue**
- [#1915](https://github.com/john-cd/rust_howto/issues/1915) — **P4** — Add examples: numerical optimization (argmin, CMA-ES, particle swarm, genetic algorithms) — **IMPLEMENTED ON DEVELOP; OPEN — seeded solver examples and book tables/prose are present; prior cloud tests/Clippy are recorded in the issue; required workspace/refdefs/book checks deferred to final validation**
- [#1942](https://github.com/john-cd/rust_howto/issues/1942) — **P4** — Add examples: geocode addresses and compute distances (geocoding, haversine, us-state-info) — **IMPLEMENTED ON DEVELOP; OPEN — distance/state/cache examples and fake-geocoder coverage are present; live provider access is deliberately excluded; required refdefs/book checks deferred to final validation**

#### VALIDATE-CODE-08 — Concurrency

- [#268](https://github.com/john-cd/rust_howto/issues/268) — **P3** — Fractal/threadpool example E — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused nextest/Clippy passed and the concurrency dependency graph excludes ring/cc; required full Clippy/book gates remain; details are on the issue**
- [#1011](https://github.com/john-cd/rust_howto/issues/1011) — **P3** — `actors/actors.rs`: finish — **IMPLEMENTED ON DEVELOP; BLOCKED — task-owned actor state, bounded messages, replies, shutdown, and chapter are present; prior focused tests/Clippy passed; required repository gates and issue-linked PR remain; details are on the issue**
- [#1907](https://github.com/john-cd/rust_howto/issues/1907) — **P3** — Add example: graceful shutdown on Ctrl-C (ctrlc, tokio::signal) — **IMPLEMENTED ON DEVELOP; OPEN — synchronous/async signal examples and shutdown fixtures, OS page/table, and refdefs are present; required tests/Clippy/refdefs/book checks deferred to final validation**
- [#1919](https://github.com/john-cd/rust_howto/issues/1919) — **P3** — Add examples: async functions in traits (native AFIT/RPITIT with Send bounds, trait_variant, async_trait for dyn) — **IMPLEMENTED ON DEVELOP; BLOCKED — examples and source docs are integrated; affected-package tests and full gates remain unresolved after Windows native-build/private-tooling failures; details are on the issue**
- [#1920](https://github.com/john-cd/rust_howto/issues/1920) — **P3** — Add a "Cancellation and Structured Concurrency" topic: JoinSet worker pools, CancellationToken trees, watch/broadcast state — **IMPLEMENTED ON DEVELOP; BLOCKED — pool, cancellation tree, watch state, supervisor, and source docs are integrated; affected-package tests and full gates remain unresolved; details are on the issue**
- [#1908](https://github.com/john-cd/rust_howto/issues/1908) — **P4** — Add examples: real-time-safe communication between threads (rtrb, ringbuf, flume) — **IMPLEMENTED ON DEVELOP; OPEN — bounded allocation-free SPSC drain with silence on underrun, command-channel examples, worker error propagation, and callback guidance are present; formatted; tests, Clippy, refdefs, and book checks deferred to final validation**

#### VALIDATE-CODE-09 — Tabular data and finance

- [#1949](https://github.com/john-cd/rust_howto/issues/1949) — **P4** — Add examples: finance building blocks (option pricing, rolling statistics, trading calendars, market-data and broker clients) — **PARTIAL ON DEVELOP; BLOCKED — pricing, statistics, Yahoo fixture, and pages are present; calendar code uses supplied holidays rather than the requested exchange-calendar integration; provider access and final checks deferred; remaining work is on the issue**

#### VALIDATE-CODE-10 — Cloud services and SDKs

- [#1928](https://github.com/john-cd/rust_howto/issues/1928) — **P4** — Add example: write a Kubernetes operator with kube-rs (CRD, reconcile, error policy, status patch) — **PARTIAL ON DEVELOP; BLOCKED — CRD/resource-builder/reconcile examples and page are present; requested CRD YAML snapshot is absent; controller validation needs a cluster; final checks deferred; remaining work is on the issue**
- [#1938](https://github.com/john-cd/rust_howto/issues/1938) — **P4** — Add examples: AWS SDK for Rust basics (shared config, SES v2 email, CloudWatch metrics, invoke Lambda, S3-compatible endpoints) — **PARTIAL ON DEVELOP; BLOCKED — SDK examples/pages are present; noted CloudWatch/Prometheus comparison and custom TLS guidance remain; live AWS/S3-compatible checks need credentials/services; final checks deferred; remaining work is on the issue**

#### VALIDATE-CODE-11 — Algorithms, data structures and memory

- [#1936](https://github.com/john-cd/rust_howto/issues/1936) — **P3** — Add examples: fast non-cryptographic hashing (ahash, rustc-hash, xxhash-rust) and bit sets (fixedbitset) — **IMPLEMENTED ON DEVELOP; BLOCKED — examples, visited-set flow, and source docs are integrated; affected-package tests and required full gates remain unresolved; details are on the issue**
- [#1914](https://github.com/john-cd/rust_howto/issues/1914) — **P4** — Add examples: SIMD vector math with std::arch (and portable alternatives) — **IMPLEMENTED ON DEVELOP; OPEN — runtime-dispatched intrinsics, scalar/wide alternatives, and page are present; portable dot product now shares scalar common-prefix semantics with unequal-length coverage; formatted; tests and final gates deferred**
- [#1925](https://github.com/john-cd/rust_howto/issues/1925) — **P4** — Add examples: memory techniques: arena allocation (bumpalo), string interning (ustr), feature-selected global allocator — **PARTIAL ON DEVELOP; BLOCKED — arena/interner/allocator examples and pages are present; allocation measurement and strict mutually exclusive feature selection remain; selection conflicts with the current all-features gate, and jemalloc needs a non-MSVC host; details are on the issue**

#### VALIDATE-CODE-12 — Language and domain models

- [#1923](https://github.com/john-cd/rust_howto/issues/1923) — **P3** — Add a "Domain Newtypes" recipe: validated value types with operator overloading, Display/FromStr and serde — **IMPLEMENTED ON DEVELOP; BLOCKED — examples, book coverage, and source docs are integrated; affected-package tests and full Clippy/refdefs/book gates remain unresolved; details are on the issue**
- [#1935](https://github.com/john-cd/rust_howto/issues/1935) — **P3** — Add examples: more strum derives (EnumMessage, EnumProperty, serialize_all, case-insensitive parsing, variant lists for UIs) — **IMPLEMENTED ON DEVELOP; BLOCKED — enum message/property/case examples, book coverage, and source docs are integrated; affected-package tests and full Clippy/refdefs/book gates remain unresolved; details are on the issue**

#### VALIDATE-CODE-13 — Testing and observability

- [#1918](https://github.com/john-cd/rust_howto/issues/1918) — **P3** — Add testing recipes: snapshots, parameterized cases, env-var isolation, HTTP mocks, paused time, heap assertions — **IMPLEMENTED ON DEVELOP; BLOCKED — recipes and source docs are integrated; prior testing-package run passed 44 with 1 ignored; affected-package/Clippy/refdefs/book gates remain unresolved; details are on the issue**
- [#1934](https://github.com/john-cd/rust_howto/issues/1934) — **P3** — Add examples: instrument code for frame profiling (profiling, puffin, puffin_http) — **IMPLEMENTED ON DEVELOP; BLOCKED — profiling facade/Puffin server examples, prose, and source docs are integrated; affected-package/Clippy/refdefs/book gates remain unresolved; details are on the issue**

#### VALIDATE-CODE-14 — Text processing and HTML

- [#1932](https://github.com/john-cd/rust_howto/issues/1932) — **P3** — Add examples: sanitize untrusted HTML (ammonia) and convert HTML to plain text (html2text) — **IMPLEMENTED ON DEVELOP; BLOCKED — sanitization/Markdown/html2text examples, template-escaping guidance, and email plain-text recipe are present; affected-package/Clippy/refdefs/book gates remain unresolved; details are on the issue**

#### VALIDATE-CODE-15 — Parsing and XML

- [#1924](https://github.com/john-cd/rust_howto/issues/1924) — **P3** — Add example: write an expression parser (winnow lexer + precedence climbing), and validate SQL with sqlparser — **IMPLEMENTED ON DEVELOP; OPEN — calculator/parser comparison and SQL examples are present; SQL helper/page now describe syntax classification rather than proving read-only execution, with side-effecting-function coverage; formatted; tests and final gates deferred**

#### VALIDATE-CODE-16 — Filesystem and process management

- [#1906](https://github.com/john-cd/rust_howto/issues/1906) — **P3** — Add example: crash-safe atomic file replacement (temp file + fsync + rename) — **IMPLEMENTED ON DEVELOP; OPEN — tempfile/atomicwrites variants and replacement tests are present; Unix parent-directory sync and post-replacement error reporting added; Windows durability limits documented; formatted; tests and required gates deferred**
- [#1922](https://github.com/john-cd/rust_howto/issues/1922) — **P3** — Add examples: manage child processes (kill a process tree, resource limits, sandboxed workers) — **IMPLEMENTED ON DEVELOP; BLOCKED — process groups, Windows Job Objects, timeout and kill guard examples, confinement guidance, and book page are present; Linux/Windows and required full gates remain; details are on the issue**

### Prose and book structure

#### VALIDATE-PROSE-01 — Language, mathematics, date and time

- [#407](https://github.com/john-cd/rust_howto/issues/407) — **P2** — `mathematics/additional_numeric_types.md`: additional_numeric_types: write — **IMPLEMENTED ON DEVELOP; BLOCKED — numeric choices and examples are present; prior focused checks passed; full Clippy/reference gates remain; details are on the issue**
- [#938](https://github.com/john-cd/rust_howto/issues/938) — **P2** — Final review of trigonometry — **IMPLEMENTED ON DEVELOP; BLOCKED — prior example suite passed; full book validation remains; details are on the issue**
- [#1188](https://github.com/john-cd/rust_howto/issues/1188) — **P2** — `date-and-time/index.md`: review in depth — **IMPLEMENTED ON DEVELOP; BLOCKED — humantime and guidance are present; book validation and human review/promotion remain; details are on the issue**
- [#293](https://github.com/john-cd/rust_howto/issues/293) — **P3** — Write the time-crate chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused tests/Clippy and documentation checks passed; full book build remains; details are on the issue**
- [#913](https://github.com/john-cd/rust_howto/issues/913) — **P3** — Review the duration chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused tests/Clippy and documentation checks passed; full book build remains; details are on the issue**
- [#1890](https://github.com/john-cd/rust_howto/issues/1890) — **P3** — Fix Code Organization chapters and visibility examples — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused tests/Clippy, refdefs, and book build passed; full-workspace Clippy remains; details are on the issue**
- [#401](https://github.com/john-cd/rust_howto/issues/401) — **P4** — Complete the internationalization chapter D — **IMPLEMENTED ON DEVELOP; BLOCKED — prior ICU4X tests/Clippy and targeted documentation checks passed; full book/reference gates remain; details are on the issue**
- [#490](https://github.com/john-cd/rust_howto/issues/490) — **P4** — Write the number-formatting chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused tests/Clippy and documentation checks passed; full book build remains; details are on the issue**

#### VALIDATE-PROSE-02 — Asynchronous programming and caching

- [#633](https://github.com/john-cd/rust_howto/issues/633) — **P2** — `asynchronous/async.md`: write; review in depth — **IMPLEMENTED ON DEVELOP; BLOCKED — async fundamentals and runtime guidance are present; prior focused tests/Clippy and book build passed; full Clippy and Windows spelling gate remain; details are on the issue**
- [#1173](https://github.com/john-cd/rust_howto/issues/1173) — **P3** — `caching/index.md`: write — **IMPLEMENTED ON DEVELOP; BLOCKED — caching overview, crate table, and child links are present; book/spelling gates and issue-linked PR remain; details are on the issue**

#### VALIDATE-PROSE-03 — Development tools, build systems and editors

- [#294](https://github.com/john-cd/rust_howto/issues/294) — **P2** — `cargo/crate_registries.md`: write; expand — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#299](https://github.com/john-cd/rust_howto/issues/299) — **P2** — `documentation/mdbook.md`: mdbook: organize, expand — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#305](https://github.com/john-cd/rust_howto/issues/305) — **P2** — `other/other.md`: other: expand; revise refs.incl.md — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#311](https://github.com/john-cd/rust_howto/issues/311) — **P2** — `development-tools_cargo-plugins/index.md`: review in depth, reorg table — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#919](https://github.com/john-cd/rust_howto/issues/919) — **P2** — `other/code_build.md`: review; dedupe this page and cargo plugins > building; move from other section — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#962](https://github.com/john-cd/rust_howto/issues/962) — **P2** — `text-editors/index.md`: write; review in depth finish the table — **IMPLEMENTED ON DEVELOP; BLOCKED — reference/spelling and Windows AWS-LC book gates remain; details are on the issue**
- [#240](https://github.com/john-cd/rust_howto/issues/240) — **P3** — Cross-compilation chapter E — **IMPLEMENTED ON DEVELOP; BLOCKED — prior spelling/refdefs checks passed; Windows AWS-LC book wrapper remains; details are on the issue**
- [#242](https://github.com/john-cd/rust_howto/issues/242) — **P3** — Faster-linking review E — **IMPLEMENTED ON DEVELOP; BLOCKED — prior spelling/refdefs checks passed; Windows AWS-LC book wrapper remains; details are on the issue**
- [#245](https://github.com/john-cd/rust_howto/issues/245) — **P3** — Compilation-duration chapter E — **IMPLEMENTED ON DEVELOP; BLOCKED — prior spelling/refdefs checks passed; Windows AWS-LC book wrapper remains; details are on the issue**
- [#296](https://github.com/john-cd/rust_howto/issues/296) — **P3** — Expand badge documentation — **IMPLEMENTED ON DEVELOP; BLOCKED — focused Markdown checks passed; repository-wide Markdown and book gates remain; details are on the issue**
- [#921](https://github.com/john-cd/rust_howto/issues/921) — **P3** — Review the build-time tooling chapter and its native examples D — **IMPLEMENTED ON DEVELOP; BLOCKED — package check needs libgit2/pkg-config; Windows AWS-LC blocks book build; details are on the issue**
- [#922](https://github.com/john-cd/rust_howto/issues/922) — **P3** — Review Cargo auditing tools — **IMPLEMENTED ON DEVELOP; BLOCKED — direct mdBook and documentation checks passed; Windows book wrapper remains; details are on the issue**
- [#925](https://github.com/john-cd/rust_howto/issues/925) — **P3** — Review logging configuration — **IMPLEMENTED ON DEVELOP; BLOCKED — focused appender test/Clippy passed; full logger suite and Windows book gate remain; details are on the issue**
- [#926](https://github.com/john-cd/rust_howto/issues/926) — **P3** — Review diagnostic functions — **IMPLEMENTED ON DEVELOP; BLOCKED — focused diagnostic test/Clippy passed; Windows book gate remains; details are on the issue**
- [#927](https://github.com/john-cd/rust_howto/issues/927) — **P3** — Review the logging chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — focused tests/Clippy and book checks passed; full-workspace Clippy remains; details are on the issue**
- [#329](https://github.com/john-cd/rust_howto/issues/329) — **P4** — Write the procedural macro tools chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — direct mdBook and documentation checks passed; Windows book wrapper remains; details are on the issue**
- [#332](https://github.com/john-cd/rust_howto/issues/332) — **P4** — Complete the procedural-macro helper index — **IMPLEMENTED ON DEVELOP; BLOCKED — documentation checks passed; Windows AWS-LC book wrapper remains; details are on the issue**
- [#604](https://github.com/john-cd/rust_howto/issues/604) — **P4** — Write the release automation chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — direct mdBook and documentation checks passed; Windows book wrapper remains; details are on the issue**
- [#605](https://github.com/john-cd/rust_howto/issues/605) — **P4** — Write the DevOps index — **IMPLEMENTED ON DEVELOP; BLOCKED — direct mdBook and documentation checks passed; Windows book wrapper remains; details are on the issue**

#### VALIDATE-PROSE-04 — Encoding, parsing and text processing

- [#354](https://github.com/john-cd/rust_howto/issues/354) — **P2** — `encoding/typecasts.md`: typecasts: write NOW; reorganize — **IMPLEMENTED ON DEVELOP; BLOCKED — prior formatting, focused Clippy, refdefs, and book build passed; example/spelling checks and linked PR remain; details are on the issue**
- [#930](https://github.com/john-cd/rust_howto/issues/930) — **P2** — `encoding/string_encoding.md`: review — **IMPLEMENTED ON DEVELOP; OPEN — chapter guidance and related links are present; reference includes are outside the topic list; formatted; example checks, spelling, and required final gates remain**
- [#350](https://github.com/john-cd/rust_howto/issues/350) — **P3** — Clean up structured-data TOML guidance — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#440](https://github.com/john-cd/rust_howto/issues/440) — **P3** — Complete the JSON guide — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#442](https://github.com/john-cd/rust_howto/issues/442) — **P3** — Complete the Markdown parser guide — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#444](https://github.com/john-cd/rust_howto/issues/444) — **P3** — Complete TOML parser guidance — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#446](https://github.com/john-cd/rust_howto/issues/446) — **P3** — Complete the XML guide — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#448](https://github.com/john-cd/rust_howto/issues/448) — **P3** — Complete parser utility guidance — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#951](https://github.com/john-cd/rust_howto/issues/951) — **P3** — Add programming-language parser guidance and a syn example — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#952](https://github.com/john-cd/rust_howto/issues/952) — **P3** — Review the parsing index — **IMPLEMENTED ON DEVELOP; BLOCKED — parser all-target checks stop in Windows onig_sys; strict Clippy and book build remain; details are on the issue**
- [#1078](https://github.com/john-cd/rust_howto/issues/1078) — **P3** — `encoding/no_external_schema.md`: write — **PARTIAL ON DEVELOP; BLOCKED — guide is present; stale issue self-reference, full checks, Unix spelling, and linked PR remain; details are on the issue**
- [#1186](https://github.com/john-cd/rust_howto/issues/1186) — **P3** — `parser-implementations/yaml.md`: write — **IMPLEMENTED ON DEVELOP; BLOCKED — YAML guide and example are present; focused/full gates, Unix spelling, and linked PR remain; details are on the issue**
- [#1187](https://github.com/john-cd/rust_howto/issues/1187) — **P3** — `parser-implementations/html.md`: write — **IMPLEMENTED ON DEVELOP; BLOCKED — HTML parser guide is present; focused/full gates, Unix spelling, and linked PR remain; details are on the issue**
- [#1190](https://github.com/john-cd/rust_howto/issues/1190) — **P3** — `text-processing/string_manipulation.md`: write — **IMPLEMENTED ON DEVELOP; BLOCKED — string manipulation recipes are present; full gates, Unix spelling, and linked PR remain; details are on the issue**

#### VALIDATE-PROSE-05 — Web, networking and databases

- [#944](https://github.com/john-cd/rust_howto/issues/944) — **P2** — `network-programming/index.md`: write; add cross-links; review; review `email_address`, `fast_chemail` — **IMPLEMENTED ON DEVELOP; BLOCKED — network index and cross-links are present; crate release and repository activity reviewed on 2026-10-27; refdefs/spelling/book gates and linked PR remain; details are on the issue**
- [#974](https://github.com/john-cd/rust_howto/issues/974) — **P2** — `web-programming_http-client/apis.md`: review / fix — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused Clippy, refdefs, and book build passed; example/spelling and all-features checks remain; details are on the issue**
- [#977](https://github.com/john-cd/rust_howto/issues/977) — **P2** — `web-programming_http-server/index.md`: write / review in depth — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#238](https://github.com/john-cd/rust_howto/issues/238) — **P3** — Networking CLI chapter E — **IMPLEMENTED ON DEVELOP; BLOCKED — prior spelling/refdefs/Markdown checks passed; Windows AWS-LC book wrapper remains; details are on the issue**
- [#284](https://github.com/john-cd/rust_howto/issues/284) — **P3** — Expand and verify the connection-pool examples B — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused and live PostgreSQL checks passed; nextest selector and full gates remain; details are on the issue**
- [#286](https://github.com/john-cd/rust_howto/issues/286) — **P3** — Expand PostgreSQL and Cornucopia guidance B — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused Clippy and live PostgreSQL checks passed; nextest selector, full gates, and repository-wide refdefs remain; details are on the issue**
- [#291](https://github.com/john-cd/rust_howto/issues/291) — **P3** — — Search engines chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — search overview and comparison are present; Meilisearch service verification and refdefs/book/spelling gates remain; details are on the issue**
- [#971](https://github.com/john-cd/rust_howto/issues/971) — **P3** — `web-programming/mime.md`: review — **IMPLEMENTED ON DEVELOP; BLOCKED — MIME guidance and examples are present; native-build, focused tests, full gates, and linked PR remain; details are on the issue**
- [#972](https://github.com/john-cd/rust_howto/issues/972) — **P3** — `web-programming/scraping.md`: review — **IMPLEMENTED ON DEVELOP; BLOCKED — scraping guidance and offline fixtures are present; native-build, focused tests, full gates, and linked PR remain; details are on the issue**
- [#978](https://github.com/john-cd/rust_howto/issues/978) — **P3** — `web-programming_http-server/middleware.md`: review — **IMPLEMENTED ON DEVELOP; BLOCKED — prior middleware tests/Clippy and direct mdBook build passed; book wrapper, Unix spelling, and linked PR remain; details are on the issue**
- [#1064](https://github.com/john-cd/rust_howto/issues/1064) — **P3** — `database/amqp.md`: finish — **IMPLEMENTED ON DEVELOP; BLOCKED — AMQP guide and example are present; RabbitMQ service and native-build/full gates remain; details are on the issue**
- [#979](https://github.com/john-cd/rust_howto/issues/979) — **P4** — Review the WebSocket index — **IMPLEMENTED ON DEVELOP; BLOCKED — WebSocket index and direct reference checks are present; book/refdefs/spelling gates remain; details are on the issue**

#### VALIDATE-PROSE-06 — Filesystem utilities, shells and compression

- [#237](https://github.com/john-cd/rust_howto/issues/237) — **P3** — `command-line-utilities/filesystem_cli.md`: filesystem: organize — **IMPLEMENTED; OPEN — task-based tools and rename guidance; repository activity reviewed; refdefs, book, spelling, and linked PR remain**
- [#239](https://github.com/john-cd/rust_howto/issues/239) — **P3** — `command-line-utilities/shells.md`: shells: expand — **IMPLEMENTED; OPEN — shell and completion guidance, with repository activity reviewed; refdefs, book, spelling, and linked PR remain**
- [#253](https://github.com/john-cd/rust_howto/issues/253) — **P3** — `compression/tar.md`: tar: review — **IMPLEMENTED; OPEN — archive and compression guidance reviewed; #254/#255 acceptance, refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-07 — Security and cryptocurrencies

- [#1180](https://github.com/john-cd/rust_howto/issues/1180) — **P3** — Write the cryptography utilities chapter — **IMPLEMENTED; OPEN — helper recipes and security limitations documented; tests, Clippy, refdefs, book, spelling, and linked PR remain**
- [#278](https://github.com/john-cd/rust_howto/issues/278) — **P4** — Complete the cryptocurrency category overview D — **IMPLEMENTED; OPEN — crate release activity reviewed and deprecation linked; refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-08 — Systems, embedded, WebAssembly and virtualization

- [#946](https://github.com/john-cd/rust_howto/issues/946) — **P3** — Write and review the external commands chapter — **PARTIAL; OPEN — stdin description corrected; platform-dependent examples, full checks, and linked PR remain**
- [#1177](https://github.com/john-cd/rust_howto/issues/1177) — **P3** — `hardware-support/peripherals.md`: write — **IMPLEMENTED; OPEN — peripheral guide and overview table present; refdefs, book, spelling, and linked PR remain**
- [#427](https://github.com/john-cd/rust_howto/issues/427) — **P4** — Complete the no-std index — **IMPLEMENTED; OPEN — index and crate table clarified; crate review, refdefs, book, spelling, and linked PR remain**
- [#968](https://github.com/john-cd/rust_howto/issues/968) — **P4** — Complete the virtualization index — **PARTIAL; OPEN — table links repaired and vm-memory added; remaining review, refdefs, book, spelling, and linked PR remain**
- [#970](https://github.com/john-cd/rust_howto/issues/970) — **P4** — Complete the WebAssembly index — **IMPLEMENTED; OPEN — host guidance and tooling index present; refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-09 — UI, accessibility, media and rendering

- [#187](https://github.com/john-cd/rust_howto/issues/187) — **P4** — Complete the screen-reader and AccessKit chapter D — **PARTIAL; BLOCKED — desktop accessibility verification and deferred checks remain; details are on the issue**
- [#422](https://github.com/john-cd/rust_howto/issues/422) — **P4** — Video chapter and example F — **PARTIAL; BLOCKED — native media environment and deferred checks remain; details are on the issue**
- [#453](https://github.com/john-cd/rust_howto/issues/453) — **P4** — Finish and validate the glTF example D — **IMPLEMENTED; OPEN — model-format chapter and metadata-count example present; tests, Clippy, refdefs, book, spelling, and linked PR remain**
- [#638](https://github.com/john-cd/rust_howto/issues/638) — **P4** — Clipboard chapter F — **IMPLEMENTED; OPEN — text/image and platform guidance present; desktop checks, refdefs, book, spelling, and linked PR remain**
- [#940](https://github.com/john-cd/rust_howto/issues/940) — **P4** — Complete the multimedia index — **IMPLEMENTED; OPEN — subcategory and streaming links present; refdefs, book, spelling, and linked PR remain**
- [#942](https://github.com/john-cd/rust_howto/issues/942) — **P4** — Complete the images index — **PARTIAL; OPEN — task table links repaired and image recipes present; example checks, refdefs, book, spelling, and linked PR remain**
- [#943](https://github.com/john-cd/rust_howto/issues/943) — **P4** — Complete the video index — **PARTIAL; OPEN — GStreamer and FFmpeg release status reviewed; external media setup, refdefs, book, spelling, and linked PR remain**
- [#953](https://github.com/john-cd/rust_howto/issues/953) — **P4** — Complete the rendering index — **PARTIAL; OPEN — subpages nested and recipe tables grouped; content review, refdefs, book, spelling, and linked PR remain**
- [#955](https://github.com/john-cd/rust_howto/issues/955) — **P4** — Complete the rendering-engine index — **IMPLEMENTED; OPEN — pointer and generated table retained; refdefs, book, spelling, and linked PR remain**
- [#956](https://github.com/john-cd/rust_howto/issues/956) — **P4** — Complete the graphics-API index — **PARTIAL; OPEN — related-topic table repaired; platform review, refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-10 — Science, finance, data processing and visualization

- [#585](https://github.com/john-cd/rust_howto/issues/585) — **P4** — — CSV chapter — **IMPLEMENTED; OPEN — existing encoding draft linked and streaming guidance corrected; refdefs, book, spelling, and linked PR remain**
- [#589](https://github.com/john-cd/rust_howto/issues/589) — **P4** — Organize the data-engineering chapter D — **PARTIAL; OPEN — shared Polars recipe deduplicated; external runtime review, refdefs, book, spelling, and linked PR remain**
- [#961](https://github.com/john-cd/rust_howto/issues/961) — **P4** — Complete the simulation index — **PARTIAL; OPEN — malformed domain and related-topic tables repaired; crate review, refdefs, book, spelling, and linked PR remain**
- [#969](https://github.com/john-cd/rust_howto/issues/969) — **P4** — Review the visualization index — **PARTIAL; OPEN — GUI table and three-d backend corrected; crate review, refdefs, book, spelling, and linked PR remain**
- [#1205](https://github.com/john-cd/rust_howto/issues/1205) — **P4** — — Deep learning chapter — **PARTIAL; OPEN — CPU example and framework guidance present; feature build, tests, Clippy, refdefs, book, spelling, and linked PR remain**
- [#1208](https://github.com/john-cd/rust_howto/issues/1208) — **P4** — — Control systems chapter — **PARTIAL; OPEN — PID simulation and sample-period guidance present; crate review, tests, Clippy, refdefs, book, spelling, and linked PR remain**
- [#1209](https://github.com/john-cd/rust_howto/issues/1209) — **P4** — — Robotics AI chapter — **IMPLEMENTED; OPEN — A-star example and dataflow guidance present; tests, Clippy, refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-11 — Architecture and scripting

- [#610](https://github.com/john-cd/rust_howto/issues/610) — **P4** — — Rhai chapter — **IMPLEMENTED; OPEN — script errors propagate and sandbox assertions added; tests, refdefs, book, spelling, and linked PR remain**
- [#1230](https://github.com/john-cd/rust_howto/issues/1230) — **P4** — — Architecture chapters — **PARTIAL; OPEN — event, actor, and pipeline guarantees clarified; full chapter review, tests, book checks, and linked PR remain**

#### VALIDATE-PROSE-12 — Foreign function interfaces

- [#210](https://github.com/john-cd/rust_howto/issues/210) — **P4** — Finish the Python example and validate its platform requirements D — **PARTIAL; BLOCKED — Linux Python and extension environment verification, full checks, and linked PR remain**

### Tools and infrastructure

#### VALIDATE-INFRA-01 — Dependencies, CI and containers

- [#1699](https://github.com/john-cd/rust_howto/issues/1699) — **P1** — Reconcile seven conflicted PRs against main and the local example changes — **PARTIAL**
- [#1719](https://github.com/john-cd/rust_howto/issues/1719) — **P1** — Repair and document the dev container setup and compiler caching — **PARTIAL**
- [#51](https://github.com/john-cd/rust_howto/issues/51) — **P2** — Audit dependencies during CI — **PARTIAL**
- [#1727](https://github.com/john-cd/rust_howto/issues/1727) — **P2** — Benchmark container build storage and select persistent cache mounts — **NOT STARTED**
- [#12](https://github.com/john-cd/rust_howto/issues/12) — **P3** — review deny.toml — **PARTIAL**
- [#53](https://github.com/john-cd/rust_howto/issues/53) — **P4** — Evaluate whether a slim or Alpine CI image improves the supported builds I — **NOT STARTED**

#### VALIDATE-INFRA-02 — Book and workspace validation trackers

- [#1871](https://github.com/john-cd/rust_howto/issues/1871) — **P2** — Run Windows-blocked validation in a Linux environment D — **PARTIAL**
- [#1876](https://github.com/john-cd/rust_howto/issues/1876) — **P2** — Validate development-tool, encoding, and HTTP documentation changes A — **PARTIAL**
- [#1877](https://github.com/john-cd/rust_howto/issues/1877) — **P2** — Validate networking, CLI, and tar changes A — **PARTIAL**
- [#1886](https://github.com/john-cd/rust_howto/issues/1886) — **P2** — Complete remaining published-chapter accuracy validation — **PARTIAL**
- [#1900](https://github.com/john-cd/rust_howto/issues/1900) — **P3** — Validate actors, build scripts, and web documentation changes A — **PARTIAL**
- [#1959](https://github.com/john-cd/rust_howto/issues/1959) — **P3** — Verify the encoding chapters and Cap'n Proto regeneration B — **PARTIAL** — human editorial review also required
- [#1978](https://github.com/john-cd/rust_howto/issues/1978) — **P3** — Validate the remaining book and example batch — **PARTIAL**
- [#1979](https://github.com/john-cd/rust_howto/issues/1979) — **P3** — Validate concurrency, memory, hashing, and enumeration recipe pages — **PARTIAL** — human editorial review also required

### Roadmaps and epics

#### VALIDATE-EPIC-01 — Validation and acceptance tracking

- [#1996](https://github.com/john-cd/rust_howto/issues/1996) — **P2** — Coordinate validation gates and verification passes — **OPEN TRACKER**

## Deferred scope and dependencies

### Code examples

#### DEFERRED-CODE-01 — Desktop UI, accessibility, games and GPU

- [#828](https://github.com/john-cd/rust_howto/issues/828) — **P4** — 2d_raster_graphics/tiny_skia.rs: draw and encode a headless scene — **IMPLEMENTED ON DEVELOP; OPEN — existing commit `8e13180f3`; tests and lints skipped in this pass by request; PR link pending**
- [#829](https://github.com/john-cd/rust_howto/issues/829) — **P4** — data_formats/inline_gltf_triangle.rs: parse a model and report geometry counts — **IMPLEMENTED ON DEVELOP; OPEN — existing commit `3d020a6b7`; tests and lints skipped in this pass by request; PR link pending**
- [#830](https://github.com/john-cd/rust_howto/issues/830) — **P4** — rendering_engines/render.rs: resolve the placeholder — **IMPLEMENTED ON DEVELOP; OPEN — `00f43662a` removes the empty example crate per #955; tests and lints skipped by request; book validation and PR link pending**
- [#831](https://github.com/john-cd/rust_howto/issues/831) — **P4** — native_graphics_apis/wgpu_adapter_info.rs: report a GPU adapter's native backend — **IMPLEMENTED ON DEVELOP; OPEN — `1896f5110` adds a feature-gated headless query with errors; tests and lints skipped by request; build/GPU/book validation and PR link pending**
- [#886](https://github.com/john-cd/rust_howto/issues/886) — **P4** — gpu/rust_gpu.rs: resolve the shader example — **IMPLEMENTED ON DEVELOP; OPEN — `2e0fbca70` documents a separate shader setup and removes the no-op host target; tests and lints skipped by request; book validation and PR link pending**

DEFERRED-CODE-01 is integrated on local `develop`. The #828 and #829 implementations were already present; #830, #831, and #886 were rebased and integrated as focused commits. All five issues have delivery comments and remain open. Formatting and `git diff --check` passed; no builds, tests, or lints were run for this batch.

#### DEFERRED-CODE-02 — Machine learning, data and robotics

- [#844](https://github.com/john-cd/rust_howto/issues/844) — **P4** — robot_operating_systems/ros2_pubsub.rs: write — **IMPLEMENTED ON DEVELOP; OPEN — ROS 2 publisher/subscriber and optional feature complete; tests and lints skipped by request; ROS environment, book validation, and PR link pending**
- [#846](https://github.com/john-cd/rust_howto/issues/846) — **P4** — simulation/discrete_event.rs: write — **IMPLEMENTED ON DEVELOP (`c4ae5379a`); OPEN — named deterministic queue example, runner, chapter include, and assertions already present; tests and lints skipped by request; book validation and PR link pending**

#### DEFERRED-CODE-03 — Operating-system APIs, emulation and no-std

- [#752](https://github.com/john-cd/rust_howto/issues/752) — **P4** — emulators/register_vm.rs: bounded fetch/decode/execute VM; preserved PolkaVM host — **IMPLEMENTED ON DEVELOP (`b389cb8f1`); tests and lints skipped by request; book build pending**
- [#814](https://github.com/john-cd/rust_howto/issues/814) — **P4** — no_std/checksum.rs: core-only library and host runner; feature and bare-metal guidance — **IMPLEMENTED ON DEVELOP (`e8d37d62e`); tests and lints skipped by request; target and book builds pending**
- [#815](https://github.com/john-cd/rust_howto/issues/815) — **P4** — no_alloc/no_alloc.rs: heapless capacity handling, core-only snippet, and fallible formatting — **IMPLEMENTED ON DEVELOP (`99f8a11f3`); tests and lints skipped by request; book build pending**
- [#817](https://github.com/john-cd/rust_howto/issues/817) — **P4** — freebsd/freebsd.rs: read-only sysctl query and target-gated dependency — **IMPLEMENTED ON DEVELOP (`23f5eeebb`); tests and lints skipped by request; FreeBSD and book builds pending**
- [#818](https://github.com/john-cd/rust_howto/issues/818) — **P4** — linux/statx_metadata.rs: temporary-file size and optional birth time through rustix — **IMPLEMENTED ON DEVELOP (`090e6a956`); tests and lints skipped by request; Linux and book builds pending**
- [#819](https://github.com/john-cd/rust_howto/issues/819) — **P4** — macos/macos.rs: safe objc2 Foundation system-version query and target gating — **IMPLEMENTED ON DEVELOP (`6677bdc9f`); tests and lints skipped by request; macOS and book builds pending**

#### DEFERRED-CODE-04 — Media encoding, images and video

- [#805](https://github.com/john-cd/rust_howto/issues/805) — **P4** — multimedia/multimedia1.rs: write — **PARTIAL; BLOCKED — audio-device validation and device-independent source test remain; details are on the issue**
- [#808](https://github.com/john-cd/rust_howto/issues/808) — **P4** — images/images.rs: write — **IMPLEMENTED; OPEN — gradient, resize, in-memory PNG/JPEG conversion, and blur examples; tests, lints, book validation, and linked PR remain**
- [#809](https://github.com/john-cd/rust_howto/issues/809) — **P4** — video/video.rs: write — **IMPLEMENTED ON DEVELOP; OPEN — synthetic AV1 encoder and packet assertion present; tests, lints, book validation, issue note removal, and linked PR remain**

#### DEFERRED-CODE-05 — Localization and internationalization

- [#796](https://github.com/john-cd/rust_howto/issues/796) — **P4** — internationalization/internationalization1.rs: write — **IMPLEMENTED; OPEN — ICU number and date formatting in two locales; tests, lints, book validation, and linked PR remain**
- [#797](https://github.com/john-cd/rust_howto/issues/797) — **P4** — localization/localization1.rs: write — **PARTIAL; DEFERRED — Fluent dependencies and translation/plural example remain; details are on the issue**

#### DEFERRED-CODE-06 — Aerospace and device protocols

- [#60](https://github.com/john-cd/rust_howto/issues/60) — **P4** — aerospace/aero.rs: write — **IMPLEMENTED; OPEN — sample an ISS orbit and assert orbital radii; tests, Clippy, book validation, and linked PR remain**
- [#61](https://github.com/john-cd/rust_howto/issues/61) — **P4** — drones/drones1.rs: write — **BLOCKED — flight-plan work and real-drone validation remain; details are on the issue**
- [#62](https://github.com/john-cd/rust_howto/issues/62) — **P4** — aerospace_protocols/aero_protocols.rs: write — **IMPLEMENTED; OPEN — in-memory MAVLink heartbeat round trip and field assertions; tests, Clippy, book validation, and linked PR remain**
- [#63](https://github.com/john-cd/rust_howto/issues/63) — **P4** — aerospace_simulation/aero_simulation.rs: write — **IMPLEMENTED; OPEN — simplify launch output and assert ascent, burnout, descent, and ground boundary; tests, Clippy, book validation, and linked PR remain**
- [#64](https://github.com/john-cd/rust_howto/issues/64) — **P4** — space_protocols/space_protocols.rs: write — **IMPLEMENTED; OPEN — CCSDS and PUS telecommand/telemetry examples; tests, Clippy, book validation, and linked PR remain**
- [#65](https://github.com/john-cd/rust_howto/issues/65) — **P4** — uavs/uavs.rs: write — **BLOCKED — telemetry decoding and real-drone validation remain; details are on the issue**

### Tools and infrastructure

#### DEFERRED-INFRA-01 — Dependencies, CI and containers

- [#1283](https://github.com/john-cd/rust_howto/issues/1283) — **P4** — finance/Cargo.toml: RustQuant: heavy build (polars, nalgebra, etc) — **IMPLEMENTED; OPEN — keep RustQuant out, document the existing standalone example; build timing, tests, Clippy, and linked PR remain**

### Roadmaps and epics

#### DEFERRED-EPIC-01 — Later-edition chapters and examples

- [#1994](https://github.com/john-cd/rust_howto/issues/1994) — **P4** — Plan later-edition chapters and example stubs — **OPEN TRACKER; DEFERRED**

## Human review required

### Code examples

#### HUMAN-CODE-01 — Networking, authentication and email

- [#1144](https://github.com/john-cd/rust_howto/issues/1144) — **P2** — `examples/lettre.rs`: review; Requires valid SMTP credentials to run — **HUMAN REVIEW**

#### HUMAN-CODE-02 — Databases and service clients

- [#46](https://github.com/john-cd/rust_howto/issues/46) — **P2** — Finish the deadpool2 example; human review remains required — **HUMAN REVIEW**

#### HUMAN-CODE-03 — Build scripts, Cargo and procedural macros

- [#1158](https://github.com/john-cd/rust_howto/issues/1158) — **P3** — `write_proc_macros/main.rs`: finish; review the following; decide what examples are needed — **HUMAN REVIEW**

#### HUMAN-CODE-04 — Desktop UI, accessibility, games and GPU

- [#1752](https://github.com/john-cd/rust_howto/issues/1752) — **P3** — Verify game engine examples on Windows (review) — **HUMAN REVIEW**
- [#1753](https://github.com/john-cd/rust_howto/issues/1753) — **P3** — Review and complete the staging games example (review) — **HUMAN REVIEW**
- [#1755](https://github.com/john-cd/rust_howto/issues/1755) — **P3** — Review the minifb renderer example (review) — **HUMAN REVIEW**
- [#1858](https://github.com/john-cd/rust_howto/issues/1858) — **P3** — Review the earlier additions and Linux Relm4 evidence — **HUMAN REVIEW**

#### HUMAN-CODE-05 — Compression and archives

- [#1875](https://github.com/john-cd/rust_howto/issues/1875) — **Unprioritized** — Review the flate2 file-I/O example — **HUMAN REVIEW**

### Prose and book structure

#### HUMAN-PROSE-01 — Language, mathematics, date and time

- [#1750](https://github.com/john-cd/rust_howto/issues/1750) — **P1** — Review the dynamic typing chapter (review) — **PARTIAL; HUMAN REVIEW**
- [#935](https://github.com/john-cd/rust_howto/issues/935) — **P2** — `mathematics/complex_numbers.md`: final review — **PARTIAL; HUMAN REVIEW**

#### HUMAN-PROSE-02 — Systems, embedded, WebAssembly and virtualization

- [#639](https://github.com/john-cd/rust_howto/issues/639) — **P3** — os/rust_os.md: write; move to written-in-rust? — **HUMAN REVIEW**
- [#346](https://github.com/john-cd/rust_howto/issues/346) — **P4** — Review the staged embedded promotion scope C — **PARTIAL; HUMAN REVIEW** — Reviewer: john-cd

#### HUMAN-PROSE-03 — Publishing, curation and book coverage

- [#1725](https://github.com/john-cd/rust_howto/issues/1725) — **P2** — Review the Pandoc and Typst export plan (review) — **HUMAN REVIEW**
- [#1748](https://github.com/john-cd/rust_howto/issues/1748) — **P2** — Review draft and staging crate inventory for book coverage (review) — **HUMAN REVIEW**

#### HUMAN-PROSE-04 — UI, accessibility, media and rendering

- [#366](https://github.com/john-cd/rust_howto/issues/366) — **P4** — game-development/game_development.md: game_development: identify crates; write — **HUMAN REVIEW**
- [#1212](https://github.com/john-cd/rust_howto/issues/1212) — **P4** — rendering/text_rendering.md: write / decide what to cover — **HUMAN REVIEW**
- [#1213](https://github.com/john-cd/rust_howto/issues/1213) — **P4** — rendering/3d_renderers.md: write. decide what to cover — **HUMAN REVIEW**
- [#1215](https://github.com/john-cd/rust_howto/issues/1215) — **P4** — rendering/2d_raster_graphics.md: write. decide what to cover. — **HUMAN REVIEW**
- [#1227](https://github.com/john-cd/rust_howto/issues/1227) — **P4** — rendering_graphics-api/vulkan.md: decide what to cover / write — **HUMAN REVIEW**
- [#1228](https://github.com/john-cd/rust_howto/issues/1228) — **P4** — rendering_graphics-api/shaders.md: decide what to cover, write — **HUMAN REVIEW**

#### HUMAN-PROSE-05 — Science, finance, data processing and visualization

- [#963](https://github.com/john-cd/rust_howto/issues/963) — **P4** — Review natural-language-processing coverage — **HUMAN REVIEW**
- [#1206](https://github.com/john-cd/rust_howto/issues/1206) — **P4** — simulation/physics_engines.md: write / decide what to cover — **HUMAN REVIEW**

#### HUMAN-PROSE-06 — Language and domain-specific languages

- [#251](https://github.com/john-cd/rust_howto/issues/251) — **P4** — cover DSLs in Rust — **HUMAN REVIEW**

### Tools and infrastructure

#### HUMAN-INFRA-01 — Dependencies, CI and containers

- [#1850](https://github.com/john-cd/rust_howto/issues/1850) — **P1** — Re-enable GitHub Actions CI and verify the refactored workflows (review) — **HUMAN REVIEW**
- [#1740](https://github.com/john-cd/rust_howto/issues/1740) — **P2** — Review remaining published-example dependency pins and upgrade notes (review) — **PARTIAL; HUMAN REVIEW**

#### HUMAN-INFRA-02 — Repository metadata and analytics

- [#19](https://github.com/john-cd/rust_howto/issues/19) — **P3** — review Google Analytics setup — **HUMAN REVIEW**
- [#1872](https://github.com/john-cd/rust_howto/issues/1872) — **P3** — Document the issue label and priority scheme (review) — **PARTIAL; HUMAN REVIEW**

#### HUMAN-INFRA-03 — Markdown linting and spelling

- [#1862](https://github.com/john-cd/rust_howto/issues/1862) — **P3** — Add CSpell for cross-platform Markdown spell checking — **HUMAN REVIEW**
- [#1863](https://github.com/john-cd/rust_howto/issues/1863) — **P3** — Add markdownlint-cli2 for Markdown linting — **HUMAN REVIEW**

#### HUMAN-INFRA-04 — Contributor hooks and container setup

- [#1245](https://github.com/john-cd/rust_howto/issues/1245) — **P3** — precommit/mod.just: finish - install pre-commit or use cargo-husky? — **HUMAN REVIEW**

#### HUMAN-INFRA-05 — Project policy, publication and outreach

- [#16](https://github.com/john-cd/rust_howto/issues/16) — **P3** — review license — **HUMAN REVIEW**
- [#17](https://github.com/john-cd/rust_howto/issues/17) — **P4** — send a pull request to little book of rust books — **HUMAN REVIEW**
- [#18](https://github.com/john-cd/rust_howto/issues/18) — **P4** — Consider adding project to goodfirstissue.dev — **HUMAN REVIEW**

### Roadmaps and epics

#### HUMAN-EPIC-01 — Human decisions and review

- [#1849](https://github.com/john-cd/rust_howto/issues/1849) — **Unprioritized** — Coordinate tasks requiring human decisions — **OPEN TRACKER; HUMAN REVIEW**
