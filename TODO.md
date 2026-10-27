<!-- TODO Reviewed 10-25-2026 -->

# TO DO List

- [ ] JOHN review tools/
- [ ] JOHN review .github\actions
- [ ] JOHN review / rewrite the .py scripts
- [ ] JOHN Review .pre-commit-config.yaml vs prek.toml
- [ ] JOHN review LICENSE
- [ ] JOHN bookmarks.json

## Tasks

### Urgent

- [/] review dirty worktrees, create commits, integrate useful branches, discard stale work, and clean up topic refs
  - [x] review `.worktrees\d`: 890 uncommitted deletions were accidental removals under `later/`; restored them
  - [x] commit the main-worktree example cleanup, entry-point test coverage, and formatting in separate commits
  - [/] `.worktrees\b` is active in another task; keep `.worktrees\c` untouched while its owner is working
- [/] integrate all useful topic branches into develop and discard the rest
- [x] format all dprint-included Markdown and configuration files
  - [x] `dprint fmt` formatted 54 files; `dprint check` passed

### Scripts, Tools

- [x] add a comment section at the beginning of all mod.just files and scripts with usage and purpose (after shebang line if there is one)
- [x] move # TODO Reviewed ??-??-2026 after a comment blank line at the end of a comment section after the shebang line in all .sh files
- [/] verify that all .sh files can run in git bash
  - [x] all 60 repository shell scripts passed `bash -n`; runtime-dependent commands were not executed
- [/] just files: update all `rust_howto`-owned `windows-shell` settings to Git Bash
  - [x] Updated and parser-checked the repository-owned justfiles; `mdbook-utils` remains a separately versioned submodule and was left unchanged.
- [x] mod.just: deduplicate tasks for unix / windows if they are the same. see e.g. bk\scripts\anchors\mod.just
- [x] use `_typos.toml` and the `typos` recipe as the cross-platform spelling check; keep the Aspell script and dictionary as an optional interactive legacy workflow
- [x] dprint.json expand to YAML, JSON, TOML, and Dockerfile; format tracked Markdown and configuration files
- [x] keep `.vscode/tasks.json` for the one-click default `just bk book serve` build task

### Docker

- [x] no raw passwords in compose*.yaml; service credentials use required environment-variable placeholders
- [x] Dockerfile: fix comments; remove the unused toolchain-file rewrite
  - [x] keep `cargo install --git --rev --locked` for the pinned, in-development `mdbook-utils` revision
- [x] keep both installers: PowerShell installs native Windows dependencies; the shell script installs Debian/Ubuntu dependencies and must run in Linux, WSL, or a container, not Git Bash
- [x] update .devcontainer\WINDOWS_BUILD.md

### Miscellaneous

- [x] in all files: remove markdown link to a GitHub issue if it has been fully resolved and closed
- [x] review and remove playground\drafts\workflow.yml
- [x] tools/ xmpl/ Cargo.toml - move dependencies to workspace

### Code Examples

- [x] update to syn 3.x; maintained direct dependencies use syn 3.0.9
- [x] review playground\bits_and_pieces and determine if any content should be moved or removed
- [x] later\crates\gui_vizia\Cargo.toml move to proper location
- [x] integrate embedded see: exclude = ["cats/embedded"] ; see later\crates\cats\embedded\Cargo.toml
- [x] no unwrap() except in places where we demonstrate unwrap(); preserved dedicated demonstrations, generated code, and archival scratch files
- [x] all maintained examples with a `main()` or equivalent entry point have test coverage; interactive and platform-dependent tests are ignored by default
- [x] give descriptive names to test functions; generic example test names were replaced, with trait methods excluded
- [/] improve comments within examples. add context and explanations where necessary. end with a period.
  - [x] added behavior-specific doc comments to new example entry-point tests; a broad editorial comment review remains
- [x] move playground\examples_wip examples to bk/ if ready; reviewed remaining examples and kept them in scratch because each needs design or portability work
- [x] make all Cargo.toml consistent; sort workspace manifests and centralize shared dependencies where workspaces support it
- [/] move code examples from later/ to bk/ when code + text ready
  - [x] retain the separate LATER text and staging workspaces; no additional matching chapter paths or ready package pairs were identified in this pass

---

## In progress

B1 implementation commits are integrated into local `develop` through `7761c5e6c` (not pushed). Seven B1 issues are closed: #1941, #1940, #939, #1252, #1281, #1912, and #1950. #912 and #1058 remain open because their done criteria require PR links; #864 remains open because its human-review marker is preserved. Formatting, focused tests/Clippy, reference checks, Markdown lint, mdBook build/linkcheck, sitemap generation, static asset copy, and `git diff --check` passed. `just bb` passed in the elevated run; mdBook reported an indexing-plugin version warning and the optional Pandoc backend was unavailable.

Selected topic branches are integrated linearly into local `develop` through `fa58e2dba` (not pushed). This includes the async/concurrency examples and the science changes for #835, #837, #473, #839, and #840; the existing NIfTI example was retained alongside the new neuron simulation. GitHub marks #473 closed. Issues #835, #837, #839, and #840 remain open because their done criteria require PR links. Focused Clippy and tests passed for the changed science packages (including Candle), async/concurrency packages, MIME and GraphQL examples, and link-generation fixture. The full all-features Clippy gate is blocked by the missing Oracle `OCI_LIB_DIR`; the broad `development_tools_debugging` test run still fails in existing log examples that initialize the process-wide logger together.

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

#### ACTIVE-CODE-08 — Formatting, encoding, paths and global state

- #939 — **P3** — Modernize lazy initialization and synchronized global-state examples — Everyday examples and concrete fixes chat — **CLOSED; INTEGRATED ON LOCAL DEVELOP**
- #1940 — **P3** — Add portable path, file-signature, and argument-quoting examples — Everyday examples and concrete fixes chat — **CLOSED; INTEGRATED ON LOCAL DEVELOP**
- #1941 — **P3** — Add small formatting and encoding recipes — Everyday examples and concrete fixes chat — **CLOSED; INTEGRATED ON LOCAL DEVELOP**

#### ACTIVE-CODE-09 — ORMs, GraphQL, metrics and decision tables

- [#864](https://github.com/john-cd/rust_howto/issues/864) — **P3** — Finish Async-GraphQL with deterministic schema/query tests — Everyday examples and concrete fixes chat — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — HUMAN REVIEW MARKER PRESERVED**
- [#912](https://github.com/john-cd/rust_howto/issues/912) — **P3** — Extract the Toasty example and finish ORM guidance using in-memory SQLite — Everyday examples and concrete fixes chat — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- #1912 — **P4** — Demonstrate metrics and Prometheus output without an HTTP server — Everyday examples and concrete fixes chat — **CLOSED; INTEGRATED ON LOCAL DEVELOP**
- #1950 — **P4** — Finish the decision-table missing-input error case and validation — Everyday examples and concrete fixes chat — **CLOSED; INTEGRATED ON LOCAL DEVELOP**

### Prose and book structure

#### ACTIVE-PROSE-01 — Foreign function interfaces

- [#1073](https://github.com/john-cd/rust_howto/issues/1073) — **P4** — Complete Lua integration recipes and tested includes — Text processing, automation, and local runtimes chat — **PARTIAL; OPEN — sandbox guidance is documented, but a hostile-script sandbox and external module runtime test remain**

### Tools and infrastructure

#### ACTIVE-INFRA-01 — Preprocessors, references and recipe portability

- [#1253](https://github.com/john-cd/rust_howto/issues/1253) — **P3** — Verify the supported `just` version and modular recipe behavior — **PARTIAL; OPEN — Windows just 1.59.0 verified; Linux entry-point verification unavailable**
- #1281 — **P3** — Update category-link generation to use landing pages and preprocessor directives / Codex — Everyday examples and concrete fixes chat — **CLOSED; LOCAL CATEGORY LINKS INTEGRATED; #248 TRACKS BADGE DIRECTIVES**

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

- [#676](https://github.com/john-cd/rust_howto/issues/676) — **P4** — Finish the AccessKit screen-reader example I — **NOT STARTED**
- [#767](https://github.com/john-cd/rust_howto/issues/767) — **P4** — Finish the Bevy example G — **NOT STARTED**
- [#768](https://github.com/john-cd/rust_howto/issues/768) — **P4** — game_engines/fyrox.rs: finish — **NOT STARTED**
- [#769](https://github.com/john-cd/rust_howto/issues/769) — **P4** — Finish or remove the ggez example G — **NOT STARTED**
- [#771](https://github.com/john-cd/rust_howto/issues/771) — **P4** — Finish the macroquad example G — **NOT STARTED**
- [#772](https://github.com/john-cd/rust_howto/issues/772) — **P4** — Finish the wgpu compute example G — **NOT STARTED**
- [#774](https://github.com/john-cd/rust_howto/issues/774) — **P4** — text_layout/cosmic_text.rs: finish — **NOT STARTED**
- [#775](https://github.com/john-cd/rust_howto/issues/775) — **P4** — Finish the Dioxus example G — **NOT STARTED**
- [#777](https://github.com/john-cd/rust_howto/issues/777) — **P4** — Finish the femtovg example G — **NOT STARTED**
- [#778](https://github.com/john-cd/rust_howto/issues/778) — **P4** — Finish the Floem example G — **NOT STARTED**
- [#780](https://github.com/john-cd/rust_howto/issues/780) — **P4** — Finish the GTK4 example G — **NOT STARTED**
- [#781](https://github.com/john-cd/rust_howto/issues/781) — **P4** — Finish the Iced example G — **NOT STARTED**
- [#782](https://github.com/john-cd/rust_howto/issues/782) — **P4** — ui_layout/morphorm.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#783](https://github.com/john-cd/rust_howto/issues/783) — **P4** — text_layout/parley.rs: finish — **NOT STARTED**
- [#785](https://github.com/john-cd/rust_howto/issues/785) — **P4** — file_dialogs/rfd.rs: how to test — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#786](https://github.com/john-cd/rust_howto/issues/786) — **P4** — 2d_renderers/skia_safe.rs: finish — **NOT STARTED**
- [#788](https://github.com/john-cd/rust_howto/issues/788) — **P4** — ui_layout/taffy.rs: finish — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#789](https://github.com/john-cd/rust_howto/issues/789) — **P4** — window_creation/tao.rs: finish — **NOT STARTED**
- [#790](https://github.com/john-cd/rust_howto/issues/790) — **P4** — tauri/mod.rs: finish — **NOT STARTED**
- [#791](https://github.com/john-cd/rust_howto/issues/791) — **P4** — 2d_renderers/vello.rs: finish — **NOT STARTED**
- [#792](https://github.com/john-cd/rust_howto/issues/792) — **P4** — 2d_renderers/vger.rs: finish — **NOT STARTED**
- [#793](https://github.com/john-cd/rust_howto/issues/793) — **P4** — 2d_renderers/webrender.rs: finish — **NOT STARTED**
- [#794](https://github.com/john-cd/rust_howto/issues/794) — **P4** — window_creation/winit.rs: review — **IMPLEMENTED ON TOPIC BRANCH; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#880](https://github.com/john-cd/rust_howto/issues/880) — **P4** — Finish the Crux cross-platform example I — **NOT STARTED**
- [#1047](https://github.com/john-cd/rust_howto/issues/1047) — **P4** — 2d_renderers/main.rs: review — **NOT STARTED**
- [#1048](https://github.com/john-cd/rust_howto/issues/1048) — **P4** — clipboard/main.rs: review — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — focused tests, Clippy, book build, and PR link pending**
- [#1052](https://github.com/john-cd/rust_howto/issues/1052) — **P4** — retained_mode_gui/vizia.rs: finish — **NOT STARTED**
- [#1056](https://github.com/john-cd/rust_howto/issues/1056) — **P4** — window_creation/baseview.rs: write; review — **NOT STARTED**

#### NOW-CODE-04 — Machine learning, data and robotics

- [#764](https://github.com/john-cd/rust_howto/issues/764) — **P4** — quant/rustquant.rs: finish; review — **NOT STARTED**
- [#835](https://github.com/john-cd/rust_howto/issues/835) — **P4** — Finish the Candle machine-learning example I — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#837](https://github.com/john-cd/rust_howto/issues/837) — **P4** — Finish the SmartCore machine-learning example I — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#839](https://github.com/john-cd/rust_howto/issues/839) — **P4** — geo/geo.rs: write — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#840](https://github.com/john-cd/rust_howto/issues/840) — **P4** — neuroscience/neuro.rs: write — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — PR LINK REQUIRED**
- [#843](https://github.com/john-cd/rust_howto/issues/843) — **P4** — robotics_frameworks/openrr.rs: write — **NOT STARTED**
- [#845](https://github.com/john-cd/rust_howto/issues/845) — **P4** — Add the Zenoh robotics example I — **NOT STARTED**
- [#885](https://github.com/john-cd/rust_howto/issues/885) — **P4** — Finish the Polars example I — **NOT STARTED**
- [#1929](https://github.com/john-cd/rust_howto/issues/1929) — **P4** — Validate the ONNX inference example with ort A — **PARTIAL**

#### NOW-CODE-05 — Recipe batches with accompanying prose

- [#1970](https://github.com/john-cd/rust_howto/issues/1970) — **P4** — Follow up on the web examples and pages — **PARTIAL**
- [#1972](https://github.com/john-cd/rust_howto/issues/1972) — **P4** — Validate the Arrow IPC, optimization, and AWS SDK examples — **PARTIAL** — human acceptance also required
- [#1974](https://github.com/john-cd/rust_howto/issues/1974) — **P4** — Follow up on Excel, profiling, geo, audio, wry, and egui examples — **PARTIAL**
- [#1977](https://github.com/john-cd/rust_howto/issues/1977) — **P4** — Finish the remaining dev-tools, parsing, text, and testing examples — **PARTIAL** — unclaimed subtasks only; coordinate with active chats
- [#1986](https://github.com/john-cd/rust_howto/issues/1986) — **P4** — Finish remaining cloud recipe batches — **PARTIAL** — unclaimed subtasks only; coordinate with active chats
- [#1991](https://github.com/john-cd/rust_howto/issues/1991) — **Unprioritized** — Resolve local recipe-batch follow-ups — **PARTIAL** — unclaimed subtasks only; coordinate with active chats

#### NOW-CODE-06 — Examples derived from repository tools

- [#41](https://github.com/john-cd/rust_howto/issues/41) — **P4** — Make additional examples out of mdbook-utils B — **PARTIAL**
- [#43](https://github.com/john-cd/rust_howto/issues/43) — **P4** — Make additional examples out of the templ tool B — **PARTIAL**

#### NOW-CODE-07 — WebAssembly and browser applications

- [#856](https://github.com/john-cd/rust_howto/issues/856) — **P4** — yew/yew.rs: review — **NOT STARTED**
- [#1944](https://github.com/john-cd/rust_howto/issues/1944) — **P4** — Add examples: Leptos web app building blocks (leptos_router, gloo-net, gloo-storage, icons, gRPC-Web client) — **PARTIAL**

#### NOW-CODE-08 — Media encoding, images and video

- [#1943](https://github.com/john-cd/rust_howto/issues/1943) — **P4** — Add examples: audio files, playback and MIDI (hound, rodio with a custom Source, midir, hrtf) — **PARTIAL**

#### NOW-CODE-09 — Networking, authentication and email

- [#867](https://github.com/john-cd/rust_howto/issues/867) — **P3** — examples/leptos.rs: finish — **NOT STARTED**
- [#1939](https://github.com/john-cd/rust_howto/issues/1939) — **P3** — Add examples: axum add-ons (axum-extra Host and TypedHeader, axum-server TLS, flash messages, #[debug_handler], tower-cookies) — **PARTIAL**

#### NOW-CODE-10 — Cloud services and SDKs

- [#1913](https://github.com/john-cd/rust_howto/issues/1913) — **P4** — Add examples: message bus over AWS SQS/SNS with Protobuf payloads (aws-sdk, prost) — **PARTIAL**

#### NOW-CODE-11 — Tabular data and finance

- [#1909](https://github.com/john-cd/rust_howto/issues/1909) — **P3** — Add examples: read and write Excel files (calamine, rust_xlsxwriter) — **PARTIAL**

#### NOW-CODE-12 — Embedded systems and hardware interfaces

- [#751](https://github.com/john-cd/rust_howto/issues/751) — **P4** — embassy/embassy.rs: finish — **NOT STARTED**

### Prose and book structure

#### NOW-PROSE-01 — Systems, embedded, WebAssembly and virtualization

- [#348](https://github.com/john-cd/rust_howto/issues/348) — **P4** — emulators/emulators.md: write — **NOT STARTED**
- [#349](https://github.com/john-cd/rust_howto/issues/349) — **P4** — emulators/index.md: write — **NOT STARTED**
- [#426](https://github.com/john-cd/rust_howto/issues/426) — **P4** — no-std/no_std.md: no_std: write — **NOT STARTED**
- [#428](https://github.com/john-cd/rust_howto/issues/428) — **P4** — no-std_no-alloc/no_alloc.md: no_alloc: write — **NOT STARTED**
- [#433](https://github.com/john-cd/rust_howto/issues/433) — **P4** — os_freebsd-apis/freebsd.md: freebsd: write — **NOT STARTED**
- [#434](https://github.com/john-cd/rust_howto/issues/434) — **P4** — os_linux-apis/linux.md: linux: write — **NOT STARTED**
- [#435](https://github.com/john-cd/rust_howto/issues/435) — **P4** — os_macos-apis/macos.md: macos: write — **NOT STARTED**
- [#496](https://github.com/john-cd/rust_howto/issues/496) — **P4** — wasm/wasm_standalone_runtimes.md: others: organize/write — **NOT STARTED**
- [#498](https://github.com/john-cd/rust_howto/issues/498) — **P4** — wasm/yew.md: yew: write / organize — **NOT STARTED**
- [#1217](https://github.com/john-cd/rust_howto/issues/1217) — **P4** — `wasm/wasm_development.md`: write — **NOT STARTED**
- [#1218](https://github.com/john-cd/rust_howto/issues/1218) — **P4** — `wasm/interfacing_with_javascript.md`: write — **PARTIAL**

#### NOW-PROSE-02 — UI, accessibility, media and rendering

- [#367](https://github.com/john-cd/rust_howto/issues/367) — **P4** — game-development/index.md: expand — **NOT STARTED**
- [#369](https://github.com/john-cd/rust_howto/issues/369) — **P4** — game-engines/game_engines.md: game_engines: write — **NOT STARTED**
- [#375](https://github.com/john-cd/rust_howto/issues/375) — **P4** — rendering_graphics-api/gpu_abstraction_layers.md: webgpu: write — **NOT STARTED**
- [#377](https://github.com/john-cd/rust_howto/issues/377) — **P4** — rendering/2d_renderers.md: 2d_renderers: write; titles — **NOT STARTED**
- [#381](https://github.com/john-cd/rust_howto/issues/381) — **P4** — gui/file_dialogs.md: file_dialogs: write — **NOT STARTED**
- [#383](https://github.com/john-cd/rust_howto/issues/383) — **P4** — gui/gtk.md: gtk: write — **NOT STARTED**
- [#385](https://github.com/john-cd/rust_howto/issues/385) — **P4** — gui/immediate_mode_gui.md: immediate_mode_gui: write — **NOT STARTED**
- [#389](https://github.com/john-cd/rust_howto/issues/389) — **P4** — gui/retained_mode_gui.md: retained_mode_gui: organize/write — **NOT STARTED**
- [#391](https://github.com/john-cd/rust_howto/issues/391) — **P4** — gui/text_layout.md: text_layout: write — **NOT STARTED**
- [#394](https://github.com/john-cd/rust_howto/issues/394) — **P4** — gui/web_based_gui.md: web_based_gui: write — **NOT STARTED**
- [#396](https://github.com/john-cd/rust_howto/issues/396) — **P4** — Write the window creation chapter G — **NOT STARTED**
- [#413](https://github.com/john-cd/rust_howto/issues/413) — **P4** — multimedia/multimedia.md: multimedia: write — **NOT STARTED**
- [#417](https://github.com/john-cd/rust_howto/issues/417) — **P4** — multimedia_encoding/encoding.md: encoding: write — **NOT STARTED**
- [#418](https://github.com/john-cd/rust_howto/issues/418) — **P4** — multimedia_encoding/index.md: write — **NOT STARTED**
- [#420](https://github.com/john-cd/rust_howto/issues/420) — **P4** — multimedia_images/images.md: images: write — **NOT STARTED**
- [#455](https://github.com/john-cd/rust_howto/issues/455) — **P4** — rendering_engine/rendering_engines.md: rendering_engines: write — **NOT STARTED**
- [#457](https://github.com/john-cd/rust_howto/issues/457) — **P4** — rendering_graphics-api/native_graphics_apis.md: graphics_apis: write — **NOT STARTED**
- [#608](https://github.com/john-cd/rust_howto/issues/608) — **P4** — gpu/index.md: write — **NOT STARTED**
- [#934](https://github.com/john-cd/rust_howto/issues/934) — **P4** — gui/ui_layout.md: review — **NOT STARTED**
- [#954](https://github.com/john-cd/rust_howto/issues/954) — **P4** — rendering_data-formats/index.md: review — **NOT STARTED**
- [#1229](https://github.com/john-cd/rust_howto/issues/1229) — **P4** — rendering_graphics-api/opengl.md: write — **NOT STARTED**
- [#1999](https://github.com/john-cd/rust_howto/issues/1999) — **Unprioritized** — categories/multimedia: write a streaming media page (RTSP, WebRTC, RTP) — **NOT STARTED**

#### NOW-PROSE-03 — Science, finance, data processing and visualization

- [#257](https://github.com/john-cd/rust_howto/issues/257) — **P4** — computer-vision/opencv.md: opencv: expand, add example — **NOT STARTED**
- [#364](https://github.com/john-cd/rust_howto/issues/364) — **P4** — finance/quant.md: quant: write — **NOT STARTED**
- [#365](https://github.com/john-cd/rust_howto/issues/365) — **P4** — `finance/index.md`: write — **NOT STARTED**
- [#472](https://github.com/john-cd/rust_howto/issues/472) — **P4** — science/index.md: write — **NOT STARTED**
- #473 — **P4** — science/classical_machine_learning.md: organize / write — **CLOSED; IMPLEMENTED AND INTEGRATED ON LOCAL DEVELOP**
- [#474](https://github.com/john-cd/rust_howto/issues/474) — **P4** — science_geo/geo.md: geo: write — **NOT STARTED**
- [#476](https://github.com/john-cd/rust_howto/issues/476) — **P4** — science_neuroscience/neuroscience.md: neuro: write — **NOT STARTED**
- [#477](https://github.com/john-cd/rust_howto/issues/477) — **P4** — science_robotics/robot_operating_systems.md: robotics: organize — **NOT STARTED**
- [#479](https://github.com/john-cd/rust_howto/issues/479) — **P4** — science_robotics/robotics_frameworks.md: useful_robotics_tools_and_libs: locate libs, organize, write — **NOT STARTED**
- [#480](https://github.com/john-cd/rust_howto/issues/480) — **P4** — science_robotics/index.md: organize — **NOT STARTED**
- [#481](https://github.com/john-cd/rust_howto/issues/481) — **P4** — simulation/simulation.md: simulation: write — **NOT STARTED**
- [#494](https://github.com/john-cd/rust_howto/issues/494) — **P4** — `visualization/visualization.md`: visualization: write — **NOT STARTED**
- [#587](https://github.com/john-cd/rust_howto/issues/587) — **P4** — data-processing/dataframes.md: dataframes: organize / edit — **NOT STARTED**
- [#1207](https://github.com/john-cd/rust_howto/issues/1207) — **P4** — science_robotics/perception_and_sensors.md: write — **NOT STARTED**
- [#1210](https://github.com/john-cd/rust_howto/issues/1210) — **P4** — science_robotics/simulation_visualization.md: write — **NOT STARTED**
- [#1211](https://github.com/john-cd/rust_howto/issues/1211) — **P4** — science_robotics/hardware_integration.md: write — **NOT STARTED**

#### NOW-PROSE-04 — Publishing, curation and book coverage

- [#36](https://github.com/john-cd/rust_howto/issues/36) — **P3** — Triage relevant pending Rust Cookbook issues and PRs I — **NOT STARTED**
- [#246](https://github.com/john-cd/rust_howto/issues/246) — **P3** — Review `blessed.rs` recommendations against the book I — **NOT STARTED**
- [#1720](https://github.com/john-cd/rust_howto/issues/1720) — **P3** — Curate remaining VS Code, browser and TOREVIEW bookmarks into book content — **PARTIAL**
- [#1957](https://github.com/john-cd/rust_howto/issues/1957) — **Unprioritized** — Audit references and links in restored book pages A — **PARTIAL** — human scope decision also required

#### NOW-PROSE-05 — Cloud platforms and containers

- [#574](https://github.com/john-cd/rust_howto/issues/574) — **P4** — cloud/aws.md: aws: write — **NOT STARTED**
- [#576](https://github.com/john-cd/rust_howto/issues/576) — **P4** — cloud/rust_native_cloud_development.md: write — **NOT STARTED**
- [#988](https://github.com/john-cd/rust_howto/issues/988) — **P4** — virtualization/containers.md: review/write — **NOT STARTED**
- [#1224](https://github.com/john-cd/rust_howto/issues/1224) — **P4** — virtualization/using_containers.md: write / fix — **NOT STARTED**
- [#1225](https://github.com/john-cd/rust_howto/issues/1225) — **P4** — virtualization/containerization.md: write — **NOT STARTED**

#### NOW-PROSE-06 — Security and cryptocurrencies

- [#1179](https://github.com/john-cd/rust_howto/issues/1179) — **P3** — cryptography/certificates.md: write — **NOT STARTED**

#### NOW-PROSE-07 — Foreign function interfaces

- [#355](https://github.com/john-cd/rust_howto/issues/355) — **P4** — external-ffi-bindings/external_ffi_bindings.md: external_ffi_bindings: write — **NOT STARTED**
- [#1070](https://github.com/john-cd/rust_howto/issues/1070) — **P4** — development-tools_ffi/erlang_elixir.md: write — **NOT STARTED**
- [#1071](https://github.com/john-cd/rust_howto/issues/1071) — **P4** — development-tools_ffi/flutter.md: write; review in depth — **NOT STARTED**
- [#1072](https://github.com/john-cd/rust_howto/issues/1072) — **P4** — development-tools_ffi/java.md: write — **NOT STARTED**
- [#1074](https://github.com/john-cd/rust_howto/issues/1074) — **P4** — development-tools_ffi/node.md: write — **NOT STARTED**
- [#1075](https://github.com/john-cd/rust_howto/issues/1075) — **P4** — development-tools_ffi/objc.md: write — **NOT STARTED**
- [#1076](https://github.com/john-cd/rust_howto/issues/1076) — **P4** — development-tools_ffi/ruby.md: write — **NOT STARTED**

#### NOW-PROSE-08 — Web, networking and databases

- [#288](https://github.com/john-cd/rust_howto/issues/288) — **P3** — database/search.md: expand database search guidance — **NOT STARTED**
- [#290](https://github.com/john-cd/rust_howto/issues/290) — **P3** — database-implementations/databases.md: databases: expand / write — **NOT STARTED**
- [#292](https://github.com/john-cd/rust_howto/issues/292) — **P3** — database-implementations/index.md: database-implementations: expand — **NOT STARTED**
- [#342](https://github.com/john-cd/rust_howto/issues/342) — **P3** — email/send_emails.md: send_emails: write — **NOT STARTED**
- [#343](https://github.com/john-cd/rust_howto/issues/343) — **P3** — email/index.md: write — **NOT STARTED**
- [#509](https://github.com/john-cd/rust_howto/issues/509) — **P3** — web-programming_http-server/_batteries-included_frameworks.md: batteries-included_frameworks: write — **NOT STARTED**
- [#518](https://github.com/john-cd/rust_howto/issues/518) — **P3** — web-programming_http-server/other_frameworks.md: other_frameworks: organize — **NOT STARTED**
- [#1065](https://github.com/john-cd/rust_howto/issues/1065) — **P3** — database/index.md: organize — **NOT STARTED**
- [#1067](https://github.com/john-cd/rust_howto/issues/1067) — **P3** — database/mssql.md: organize / write — **NOT STARTED**
- [#1068](https://github.com/john-cd/rust_howto/issues/1068) — **P3** — database/nosql.md: review — **NOT STARTED**
- [#1069](https://github.com/john-cd/rust_howto/issues/1069) — **P3** — database/oracle.md: write — **NOT STARTED**

#### NOW-PROSE-09 — Development tools, build systems and editors

- [#600](https://github.com/john-cd/rust_howto/issues/600) — **P4** — devops/github_actions.md: github_actions - see blessed.rs — **NOT STARTED**
- [#615](https://github.com/john-cd/rust_howto/issues/615) — **P4** — written-in-rust/other_tools.md: others: review/add — **NOT STARTED**

#### NOW-PROSE-10 — Localization and internationalization

- [#404](https://github.com/john-cd/rust_howto/issues/404) — **P4** — localization/localization.md: localization: write — **NOT STARTED**
- [#405](https://github.com/john-cd/rust_howto/issues/405) — **P4** — localization/index.md: write — **NOT STARTED**

#### NOW-PROSE-11 — Asynchronous programming and caching

- [#269](https://github.com/john-cd/rust_howto/issues/269) — **P3** — concurrency/_actors.md: _actors: organize — **NOT STARTED**

#### NOW-PROSE-12 — Aerospace and device protocols

- [#190](https://github.com/john-cd/rust_howto/issues/190) — **P4** — aerospace/index.md: review — **NOT STARTED**
- [#193](https://github.com/john-cd/rust_howto/issues/193) — **P4** — aerospace_drones/index.md: write — **NOT STARTED**
- [#196](https://github.com/john-cd/rust_howto/issues/196) — **P4** — aerospace_protocols/index.md: aerospace protocols: write — **NOT STARTED**
- [#199](https://github.com/john-cd/rust_howto/issues/199) — **P4** — aerospace_simulation/aerospace_simulation.md: aerospace_simulation: write — **NOT STARTED**
- [#201](https://github.com/john-cd/rust_howto/issues/201) — **P4** — aerospace_space-protocols/space_protocols.md: space_protocols: write — **NOT STARTED**
- [#202](https://github.com/john-cd/rust_howto/issues/202) — **P4** — aerospace_space-protocols/index.md: write — **NOT STARTED**

### Tools and infrastructure

#### NOW-INFRA-01 — Preprocessors, references and recipe portability

- [#248](https://github.com/john-cd/rust_howto/issues/248) — **P1** — replace current badge markdown copy-pasted text by a preprocessor command {#crate ...} {#categories... } — **PARTIAL**

#### NOW-INFRA-02 — Dependencies, CI and containers

- [#1998](https://github.com/john-cd/rust_howto/issues/1998) — **P1** — Port remaining examples across major dependency upgrades — **PARTIAL** — upstream-blocked subset

#### NOW-INFRA-03 — Branch integration and retained work

- [#1992](https://github.com/john-cd/rust_howto/issues/1992) — **P1** — Triage and integrate unique work from retained example branches — **PARTIAL** — preserve branch ownership

#### NOW-INFRA-04 — Windows validation and tooling repairs

- [#1983](https://github.com/john-cd/rust_howto/issues/1983) — **P3** — Finish Windows validation and reference-generator repairs — **PARTIAL**

#### NOW-INFRA-05 — Contributor hooks and container setup

- [#1247](https://github.com/john-cd/rust_howto/issues/1247) — **P3** — precommit/mod.just: move to Dockerfile — **NOT STARTED**

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

- [#225](https://github.com/john-cd/rust_howto/issues/225) — **P3** — Offline Basic authentication example F — **PARTIAL**
- [#810](https://github.com/john-cd/rust_howto/issues/810) — **P3** — Run native Linux tests and strict Clippy for the Glommio example — **PARTIAL**
- [#1145](https://github.com/john-cd/rust_howto/issues/1145) — **P3** — `websocket/tokio_tungstenite.rs`: review https://github.com/snapview/tokio-tungstenite/tree/master/examples — **PARTIAL**
- [#1910](https://github.com/john-cd/rust_howto/issues/1910) — **P3** — Add examples: web login with axum-login, tower-sessions and Argon2 password hashing — **PARTIAL**
- [#1911](https://github.com/john-cd/rust_howto/issues/1911) — **P3** — Add examples: client-side rate limiting and retries (governor, backon/backoff) — **PARTIAL**
- [#1945](https://github.com/john-cd/rust_howto/issues/1945) — **P3** — Add examples: gRPC-Web with tonic and protobuf codegen without a system protoc (tonic-web, tonic-prost-build, protobuf-src) — **PARTIAL**
- [#1946](https://github.com/john-cd/rust_howto/issues/1946) — **P3** — Add examples: read mail over IMAP and filter disposable addresses before sending (imap, mailchecker) — **PARTIAL**
- [#1916](https://github.com/john-cd/rust_howto/issues/1916) — **P4** — Add examples: OpenAPI docs with utoipa and JSON Schema generation with schemars — **PARTIAL**

#### VALIDATE-CODE-02 — Databases and service clients

- [#1017](https://github.com/john-cd/rust_howto/issues/1017) — **P2** — Finish the cdrs_tokio example — **PARTIAL**
- [#1022](https://github.com/john-cd/rust_howto/issues/1022) — **P2** — Finish the Sibyl example and review installation steps — **PARTIAL**
- [#1148](https://github.com/john-cd/rust_howto/issues/1148) — **P2** — Review the SurrealDB example — **PARTIAL**
- [#1162](https://github.com/john-cd/rust_howto/issues/1162) — **P2** — Review the PostgreSQL aggregate_data example — **PARTIAL**
- [#708](https://github.com/john-cd/rust_howto/issues/708) — **P3** — Finish the Cornucopia example B — **PARTIAL**

#### VALIDATE-CODE-03 — Compression and archives

- [#254](https://github.com/john-cd/rust_howto/issues/254) — **P3** — `tar/tar_decompress.rs`: review; tar_decompress.rs is noplayground - fix? — **PARTIAL**
- [#255](https://github.com/john-cd/rust_howto/issues/255) — **P3** — `tar/tar_compress.rs`: review; tar_compress.rs is noplayground - fix? — **PARTIAL**

#### VALIDATE-CODE-04 — Build scripts, Cargo and procedural macros

- [#897](https://github.com/john-cd/rust_howto/issues/897) — **P3** — Complete the C++ linkage example — **PARTIAL**
- [#898](https://github.com/john-cd/rust_howto/issues/898) — **P3** — Complete the C++ Rust wrapper example — **PARTIAL**
- [#899](https://github.com/john-cd/rust_howto/issues/899) — **P3** — Complete the C static-library example — **PARTIAL**
- [#900](https://github.com/john-cd/rust_howto/issues/900) — **P3** — Complete the C Rust wrapper example — **PARTIAL**
- [#1000](https://github.com/john-cd/rust_howto/issues/1000) — **P3** — `build_time_tooling/cc_defines.rs`: finish; deal wth cc — **PARTIAL**
- [#1060](https://github.com/john-cd/rust_howto/issues/1060) — **P3** — `buildinfo/build-info.rs`: review / convert into example — **PARTIAL**
- [#1951](https://github.com/john-cd/rust_howto/issues/1951) — **P3** — Cargo subcommand and coverage examples — **PARTIAL**
- [#739](https://github.com/john-cd/rust_howto/issues/739) — **P4** — Finish the `darling` procedural-macro helper example J — **PARTIAL**
- [#741](https://github.com/john-cd/rust_howto/issues/741) — **P4** — Finish the `proc_macro2` helper example J — **PARTIAL**
- [#742](https://github.com/john-cd/rust_howto/issues/742) — **P4** — Review and expand the `quote` helper example J — **PARTIAL**
- [#743](https://github.com/john-cd/rust_howto/issues/743) — **P4** — Finish the `syn` procedural-macro helper example J — **PARTIAL**

#### VALIDATE-CODE-05 — Foreign function interfaces

- [#738](https://github.com/john-cd/rust_howto/issues/738) — **P4** — CXX bridge F — **PARTIAL**

#### VALIDATE-CODE-06 — Desktop UI, accessibility, games and GPU

- [#1155](https://github.com/john-cd/rust_howto/issues/1155) — **P3** — `clipboard/arboard.rs`: review — **PARTIAL**
- [#1933](https://github.com/john-cd/rust_howto/issues/1933) — **P4** — Add examples: egui add-on crates (tables, plots, docking tiles, toasts, file dialog, graphs, Markdown, icons, log viewer) — **PARTIAL**
- [#1952](https://github.com/john-cd/rust_howto/issues/1952) — **P4** — Add example: show a local web page in a native window (wry, tao) — **PARTIAL**

#### VALIDATE-CODE-07 — Machine learning, data and robotics

- [#1079](https://github.com/john-cd/rust_howto/issues/1079) — **P4** — Review and expand the OpenCV example — **PARTIAL**
- [#1915](https://github.com/john-cd/rust_howto/issues/1915) — **P4** — Add examples: numerical optimization (argmin, CMA-ES, particle swarm, genetic algorithms) — **PARTIAL**
- [#1942](https://github.com/john-cd/rust_howto/issues/1942) — **P4** — Add examples: geocode addresses and compute distances (geocoding, haversine, us-state-info) — **PARTIAL**

#### VALIDATE-CODE-08 — Concurrency

- [#268](https://github.com/john-cd/rust_howto/issues/268) — **P3** — Fractal/threadpool example E — **PARTIAL**
- [#1011](https://github.com/john-cd/rust_howto/issues/1011) — **P3** — `actors/actors.rs`: finish — **PARTIAL**
- [#1907](https://github.com/john-cd/rust_howto/issues/1907) — **P3** — Add example: graceful shutdown on Ctrl-C (ctrlc, tokio::signal) — **PARTIAL**
- [#1919](https://github.com/john-cd/rust_howto/issues/1919) — **P3** — Add examples: async functions in traits (native AFIT/RPITIT with Send bounds, trait_variant, async_trait for dyn) — **PARTIAL**
- [#1920](https://github.com/john-cd/rust_howto/issues/1920) — **P3** — Add a "Cancellation and Structured Concurrency" topic: JoinSet worker pools, CancellationToken trees, watch/broadcast state — **PARTIAL**
- [#1908](https://github.com/john-cd/rust_howto/issues/1908) — **P4** — Add examples: real-time-safe communication between threads (rtrb, ringbuf, flume) — **PARTIAL**

#### VALIDATE-CODE-09 — Tabular data and finance

- [#1949](https://github.com/john-cd/rust_howto/issues/1949) — **P4** — Add examples: finance building blocks (option pricing, rolling statistics, trading calendars, market-data and broker clients) — **PARTIAL**

#### VALIDATE-CODE-10 — Cloud services and SDKs

- [#1928](https://github.com/john-cd/rust_howto/issues/1928) — **P4** — Add example: write a Kubernetes operator with kube-rs (CRD, reconcile, error policy, status patch) — **PARTIAL**
- [#1938](https://github.com/john-cd/rust_howto/issues/1938) — **P4** — Add examples: AWS SDK for Rust basics (shared config, SES v2 email, CloudWatch metrics, invoke Lambda, S3-compatible endpoints) — **PARTIAL**

#### VALIDATE-CODE-11 — Algorithms, data structures and memory

- [#1936](https://github.com/john-cd/rust_howto/issues/1936) — **P3** — Add examples: fast non-cryptographic hashing (ahash, rustc-hash, xxhash-rust) and bit sets (fixedbitset) — **PARTIAL**
- [#1914](https://github.com/john-cd/rust_howto/issues/1914) — **P4** — Add examples: SIMD vector math with std::arch (and portable alternatives) — **PARTIAL**
- [#1925](https://github.com/john-cd/rust_howto/issues/1925) — **P4** — Add examples: memory techniques: arena allocation (bumpalo), string interning (ustr), feature-selected global allocator — **PARTIAL**

#### VALIDATE-CODE-12 — Language and domain models

- [#1923](https://github.com/john-cd/rust_howto/issues/1923) — **P3** — Add a "Domain Newtypes" recipe: validated value types with operator overloading, Display/FromStr and serde — **PARTIAL**
- [#1935](https://github.com/john-cd/rust_howto/issues/1935) — **P3** — Add examples: more strum derives (EnumMessage, EnumProperty, serialize_all, case-insensitive parsing, variant lists for UIs) — **PARTIAL**

#### VALIDATE-CODE-13 — Testing and observability

- [#1918](https://github.com/john-cd/rust_howto/issues/1918) — **P3** — Add testing recipes: snapshots, parameterized cases, env-var isolation, HTTP mocks, paused time, heap assertions — **PARTIAL**
- [#1934](https://github.com/john-cd/rust_howto/issues/1934) — **P3** — Add examples: instrument code for frame profiling (profiling, puffin, puffin_http) — **PARTIAL**

#### VALIDATE-CODE-14 — Text processing and HTML

- [#1932](https://github.com/john-cd/rust_howto/issues/1932) — **P3** — Add examples: sanitize untrusted HTML (ammonia) and convert HTML to plain text (html2text) — **PARTIAL**

#### VALIDATE-CODE-15 — Parsing and XML

- [#1924](https://github.com/john-cd/rust_howto/issues/1924) — **P3** — Add example: write an expression parser (winnow lexer + precedence climbing), and validate SQL with sqlparser — **PARTIAL**

#### VALIDATE-CODE-16 — Filesystem and process management

- [#1906](https://github.com/john-cd/rust_howto/issues/1906) — **P3** — Add example: crash-safe atomic file replacement (temp file + fsync + rename) — **PARTIAL**
- [#1922](https://github.com/john-cd/rust_howto/issues/1922) — **P3** — Add examples: manage child processes (kill a process tree, resource limits, sandboxed workers) — **PARTIAL**

### Prose and book structure

#### VALIDATE-PROSE-01 — Language, mathematics, date and time

- [#407](https://github.com/john-cd/rust_howto/issues/407) — **P2** — `mathematics/additional_numeric_types.md`: additional_numeric_types: write — **PARTIAL**
- [#938](https://github.com/john-cd/rust_howto/issues/938) — **P2** — Final review of trigonometry — **PARTIAL**
- [#1188](https://github.com/john-cd/rust_howto/issues/1188) — **P2** — `date-and-time/index.md`: review in depth — **PARTIAL**
- [#293](https://github.com/john-cd/rust_howto/issues/293) — **P3** — Write the time-crate chapter — **PARTIAL**
- [#913](https://github.com/john-cd/rust_howto/issues/913) — **P3** — Review the duration chapter — **PARTIAL**
- [#1890](https://github.com/john-cd/rust_howto/issues/1890) — **P3** — Fix Code Organization chapters and visibility examples — **PARTIAL**
- [#401](https://github.com/john-cd/rust_howto/issues/401) — **P4** — Complete the internationalization chapter D — **PARTIAL**
- [#490](https://github.com/john-cd/rust_howto/issues/490) — **P4** — Write the number-formatting chapter — **PARTIAL**

#### VALIDATE-PROSE-02 — Asynchronous programming and caching

- [#633](https://github.com/john-cd/rust_howto/issues/633) — **P2** — `asynchronous/async.md`: write; review in depth — **PARTIAL**
- [#1173](https://github.com/john-cd/rust_howto/issues/1173) — **P3** — `caching/index.md`: write — **PARTIAL**

#### VALIDATE-PROSE-03 — Development tools, build systems and editors

- [#294](https://github.com/john-cd/rust_howto/issues/294) — **P2** — `cargo/crate_registries.md`: write; expand — **PARTIAL**
- [#299](https://github.com/john-cd/rust_howto/issues/299) — **P2** — `documentation/mdbook.md`: mdbook: organize, expand — **PARTIAL**
- [#305](https://github.com/john-cd/rust_howto/issues/305) — **P2** — `other/other.md`: other: expand; revise refs.incl.md — **PARTIAL**
- [#311](https://github.com/john-cd/rust_howto/issues/311) — **P2** — `development-tools_cargo-plugins/index.md`: review in depth, reorg table — **PARTIAL**
- [#919](https://github.com/john-cd/rust_howto/issues/919) — **P2** — `other/code_build.md`: review; dedupe this page and cargo plugins > building; move from other section — **PARTIAL**
- [#962](https://github.com/john-cd/rust_howto/issues/962) — **P2** — `text-editors/index.md`: write; review in depth finish the table — **PARTIAL**
- [#240](https://github.com/john-cd/rust_howto/issues/240) — **P3** — Cross-compilation chapter E — **PARTIAL**
- [#242](https://github.com/john-cd/rust_howto/issues/242) — **P3** — Faster-linking review E — **PARTIAL**
- [#245](https://github.com/john-cd/rust_howto/issues/245) — **P3** — Compilation-duration chapter E — **PARTIAL**
- [#296](https://github.com/john-cd/rust_howto/issues/296) — **P3** — Expand badge documentation — **PARTIAL**
- [#921](https://github.com/john-cd/rust_howto/issues/921) — **P3** — Review the build-time tooling chapter and its native examples D — **PARTIAL**
- [#922](https://github.com/john-cd/rust_howto/issues/922) — **P3** — Review Cargo auditing tools — **PARTIAL**
- [#925](https://github.com/john-cd/rust_howto/issues/925) — **P3** — Review logging configuration — **PARTIAL**
- [#926](https://github.com/john-cd/rust_howto/issues/926) — **P3** — Review diagnostic functions — **PARTIAL**
- [#927](https://github.com/john-cd/rust_howto/issues/927) — **P3** — Review the logging chapter — **PARTIAL**
- [#329](https://github.com/john-cd/rust_howto/issues/329) — **P4** — Write the procedural macro tools chapter — **PARTIAL**
- [#332](https://github.com/john-cd/rust_howto/issues/332) — **P4** — Complete the procedural-macro helper index — **PARTIAL**
- [#604](https://github.com/john-cd/rust_howto/issues/604) — **P4** — Write the release automation chapter — **PARTIAL**
- [#605](https://github.com/john-cd/rust_howto/issues/605) — **P4** — Write the DevOps index — **PARTIAL**

#### VALIDATE-PROSE-04 — Encoding, parsing and text processing

- [#354](https://github.com/john-cd/rust_howto/issues/354) — **P2** — `encoding/typecasts.md`: typecasts: write NOW; reorganize — **PARTIAL**
- [#930](https://github.com/john-cd/rust_howto/issues/930) — **P2** — `encoding/string_encoding.md`: review — **PARTIAL**
- [#350](https://github.com/john-cd/rust_howto/issues/350) — **P3** — Clean up structured-data TOML guidance — **PARTIAL**
- [#440](https://github.com/john-cd/rust_howto/issues/440) — **P3** — Complete the JSON guide — **PARTIAL**
- [#442](https://github.com/john-cd/rust_howto/issues/442) — **P3** — Complete the Markdown parser guide — **PARTIAL**
- [#444](https://github.com/john-cd/rust_howto/issues/444) — **P3** — Complete TOML parser guidance — **PARTIAL**
- [#446](https://github.com/john-cd/rust_howto/issues/446) — **P3** — Complete the XML guide — **PARTIAL**
- [#448](https://github.com/john-cd/rust_howto/issues/448) — **P3** — Complete parser utility guidance — **PARTIAL**
- [#951](https://github.com/john-cd/rust_howto/issues/951) — **P3** — Add programming-language parser guidance and a syn example — **PARTIAL**
- [#952](https://github.com/john-cd/rust_howto/issues/952) — **P3** — Review the parsing index — **PARTIAL**
- [#1078](https://github.com/john-cd/rust_howto/issues/1078) — **P3** — `encoding/no_external_schema.md`: write — **PARTIAL**
- [#1186](https://github.com/john-cd/rust_howto/issues/1186) — **P3** — `parser-implementations/yaml.md`: write — **PARTIAL**
- [#1187](https://github.com/john-cd/rust_howto/issues/1187) — **P3** — `parser-implementations/html.md`: write — **PARTIAL**
- [#1190](https://github.com/john-cd/rust_howto/issues/1190) — **P3** — `text-processing/string_manipulation.md`: write — **PARTIAL**

#### VALIDATE-PROSE-05 — Web, networking and databases

- [#944](https://github.com/john-cd/rust_howto/issues/944) — **P2** — `network-programming/index.md`: write; add cross-links; review; review `email_address`, `fast_chemail` — **PARTIAL**
- [#974](https://github.com/john-cd/rust_howto/issues/974) — **P2** — `web-programming_http-client/apis.md`: review / fix — **PARTIAL**
- [#977](https://github.com/john-cd/rust_howto/issues/977) — **P2** — `web-programming_http-server/index.md`: write / review in depth — **PARTIAL**
- [#238](https://github.com/john-cd/rust_howto/issues/238) — **P3** — Networking CLI chapter E — **PARTIAL**
- [#284](https://github.com/john-cd/rust_howto/issues/284) — **P3** — Expand and verify the connection-pool examples B — **PARTIAL**
- [#286](https://github.com/john-cd/rust_howto/issues/286) — **P3** — Expand PostgreSQL and Cornucopia guidance B — **PARTIAL**
- [#291](https://github.com/john-cd/rust_howto/issues/291) — **P3** — — Search engines chapter — **PARTIAL**
- [#971](https://github.com/john-cd/rust_howto/issues/971) — **P3** — `web-programming/mime.md`: review — **PARTIAL**
- [#972](https://github.com/john-cd/rust_howto/issues/972) — **P3** — `web-programming/scraping.md`: review — **PARTIAL**
- [#978](https://github.com/john-cd/rust_howto/issues/978) — **P3** — `web-programming_http-server/middleware.md`: review — **PARTIAL**
- [#1064](https://github.com/john-cd/rust_howto/issues/1064) — **P3** — `database/amqp.md`: finish — **PARTIAL**
- [#979](https://github.com/john-cd/rust_howto/issues/979) — **P4** — Review the WebSocket index — **PARTIAL**

#### VALIDATE-PROSE-06 — Filesystem utilities, shells and compression

- [#237](https://github.com/john-cd/rust_howto/issues/237) — **P3** — `command-line-utilities/filesystem_cli.md`: filesystem: organize — **PARTIAL**
- [#239](https://github.com/john-cd/rust_howto/issues/239) — **P3** — `command-line-utilities/shells.md`: shells: expand — **PARTIAL**
- [#253](https://github.com/john-cd/rust_howto/issues/253) — **P3** — `compression/tar.md`: tar: review — **PARTIAL**

#### VALIDATE-PROSE-07 — Security and cryptocurrencies

- [#1180](https://github.com/john-cd/rust_howto/issues/1180) — **P3** — Write the cryptography utilities chapter — **PARTIAL**
- [#278](https://github.com/john-cd/rust_howto/issues/278) — **P4** — Complete the cryptocurrency category overview D — **PARTIAL**

#### VALIDATE-PROSE-08 — Systems, embedded, WebAssembly and virtualization

- [#946](https://github.com/john-cd/rust_howto/issues/946) — **P3** — Write and review the external commands chapter — **PARTIAL**
- [#1177](https://github.com/john-cd/rust_howto/issues/1177) — **P3** — `hardware-support/peripherals.md`: write — **PARTIAL**
- [#427](https://github.com/john-cd/rust_howto/issues/427) — **P4** — Complete the no-std index — **PARTIAL**
- [#968](https://github.com/john-cd/rust_howto/issues/968) — **P4** — Complete the virtualization index — **PARTIAL**
- [#970](https://github.com/john-cd/rust_howto/issues/970) — **P4** — Complete the WebAssembly index — **PARTIAL**

#### VALIDATE-PROSE-09 — UI, accessibility, media and rendering

- [#187](https://github.com/john-cd/rust_howto/issues/187) — **P4** — Complete the screen-reader and AccessKit chapter D — **PARTIAL**
- [#422](https://github.com/john-cd/rust_howto/issues/422) — **P4** — Video chapter and example F — **PARTIAL**
- [#453](https://github.com/john-cd/rust_howto/issues/453) — **P4** — Finish and validate the glTF example D — **PARTIAL**
- [#638](https://github.com/john-cd/rust_howto/issues/638) — **P4** — Clipboard chapter F — **PARTIAL**
- [#940](https://github.com/john-cd/rust_howto/issues/940) — **P4** — Complete the multimedia index — **PARTIAL**
- [#942](https://github.com/john-cd/rust_howto/issues/942) — **P4** — Complete the images index — **PARTIAL**
- [#943](https://github.com/john-cd/rust_howto/issues/943) — **P4** — Complete the video index — **PARTIAL**
- [#953](https://github.com/john-cd/rust_howto/issues/953) — **P4** — Complete the rendering index — **PARTIAL**
- [#955](https://github.com/john-cd/rust_howto/issues/955) — **P4** — Complete the rendering-engine index — **PARTIAL**
- [#956](https://github.com/john-cd/rust_howto/issues/956) — **P4** — Complete the graphics-API index — **PARTIAL**

#### VALIDATE-PROSE-10 — Science, finance, data processing and visualization

- [#585](https://github.com/john-cd/rust_howto/issues/585) — **P4** — — CSV chapter — **PARTIAL**
- [#589](https://github.com/john-cd/rust_howto/issues/589) — **P4** — Organize the data-engineering chapter D — **PARTIAL**
- [#961](https://github.com/john-cd/rust_howto/issues/961) — **P4** — Complete the simulation index — **PARTIAL**
- [#969](https://github.com/john-cd/rust_howto/issues/969) — **P4** — Review the visualization index — **PARTIAL**
- [#1205](https://github.com/john-cd/rust_howto/issues/1205) — **P4** — — Deep learning chapter — **PARTIAL**
- [#1208](https://github.com/john-cd/rust_howto/issues/1208) — **P4** — — Control systems chapter — **PARTIAL**
- [#1209](https://github.com/john-cd/rust_howto/issues/1209) — **P4** — — Robotics AI chapter — **PARTIAL**

#### VALIDATE-PROSE-11 — Architecture and scripting

- [#610](https://github.com/john-cd/rust_howto/issues/610) — **P4** — — Rhai chapter — **PARTIAL**
- [#1230](https://github.com/john-cd/rust_howto/issues/1230) — **P4** — — Architecture chapters — **PARTIAL**

#### VALIDATE-PROSE-12 — Foreign function interfaces

- [#210](https://github.com/john-cd/rust_howto/issues/210) — **P4** — Finish the Python example and validate its platform requirements D — **PARTIAL**

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

- [#828](https://github.com/john-cd/rust_howto/issues/828) — **P4** — 2d_raster_graphics/render.rs: write — **NOT STARTED; DEFERRED**
- [#829](https://github.com/john-cd/rust_howto/issues/829) — **P4** — data_formats/data_formats.rs: write — **NOT STARTED; DEFERRED**
- [#830](https://github.com/john-cd/rust_howto/issues/830) — **P4** — rendering_engines/render.rs: write — **NOT STARTED; DEFERRED**
- [#831](https://github.com/john-cd/rust_howto/issues/831) — **P4** — native_graphics_apis/graphics.rs: write — **NOT STARTED; DEFERRED**
- [#886](https://github.com/john-cd/rust_howto/issues/886) — **P4** — gpu/rust_gpu.rs: write — **NOT STARTED; DEFERRED**

#### DEFERRED-CODE-02 — Machine learning, data and robotics

- [#844](https://github.com/john-cd/rust_howto/issues/844) — **P4** — robot_operating_systems/robotics.rs: write — **NOT STARTED; DEFERRED**
- [#846](https://github.com/john-cd/rust_howto/issues/846) — **P4** — simulation/simulation1.rs: write — **NOT STARTED; DEFERRED**

#### DEFERRED-CODE-03 — Operating-system APIs, emulation and no-std

- [#752](https://github.com/john-cd/rust_howto/issues/752) — **P4** — emulators/emulator.rs: write — **NOT STARTED; DEFERRED**
- [#814](https://github.com/john-cd/rust_howto/issues/814) — **P4** — no_std/no_std1.rs: write — **NOT STARTED; DEFERRED**
- [#815](https://github.com/john-cd/rust_howto/issues/815) — **P4** — no_alloc/no_alloc.rs: write — **NOT STARTED; DEFERRED**
- [#817](https://github.com/john-cd/rust_howto/issues/817) — **P4** — freebsd/freebsd.rs: write — **NOT STARTED; DEFERRED**
- [#818](https://github.com/john-cd/rust_howto/issues/818) — **P4** — linux/linux.rs: write — **NOT STARTED; DEFERRED**
- [#819](https://github.com/john-cd/rust_howto/issues/819) — **P4** — macos/macos.rs: write — **NOT STARTED; DEFERRED**

#### DEFERRED-CODE-04 — Media encoding, images and video

- [#805](https://github.com/john-cd/rust_howto/issues/805) — **P4** — multimedia/multimedia1.rs: write — **NOT STARTED; DEFERRED**
- [#808](https://github.com/john-cd/rust_howto/issues/808) — **P4** — images/images.rs: write — **NOT STARTED; DEFERRED**
- [#809](https://github.com/john-cd/rust_howto/issues/809) — **P4** — video/video.rs: write — **NOT STARTED; DEFERRED**

#### DEFERRED-CODE-05 — Localization and internationalization

- [#796](https://github.com/john-cd/rust_howto/issues/796) — **P4** — internationalization/internationalization1.rs: write — **NOT STARTED; DEFERRED**
- [#797](https://github.com/john-cd/rust_howto/issues/797) — **P4** — localization/localization1.rs: write — **NOT STARTED; DEFERRED**

#### DEFERRED-CODE-06 — Aerospace and device protocols

- [#60](https://github.com/john-cd/rust_howto/issues/60) — **P4** — aerospace/aero.rs: write — **NOT STARTED; DEFERRED**
- [#61](https://github.com/john-cd/rust_howto/issues/61) — **P4** — drones/drones1.rs: write — **NOT STARTED; DEFERRED**
- [#62](https://github.com/john-cd/rust_howto/issues/62) — **P4** — aerospace_protocols/aero_protocols.rs: write — **NOT STARTED; DEFERRED**
- [#63](https://github.com/john-cd/rust_howto/issues/63) — **P4** — aerospace_simulation/aero_simulation.rs: write — **NOT STARTED; DEFERRED**
- [#64](https://github.com/john-cd/rust_howto/issues/64) — **P4** — space_protocols/space_protocols.rs: write — **NOT STARTED; DEFERRED**
- [#65](https://github.com/john-cd/rust_howto/issues/65) — **P4** — uavs/uavs.rs: write — **NOT STARTED; DEFERRED**

### Tools and infrastructure

#### DEFERRED-INFRA-01 — Dependencies, CI and containers

- [#1283](https://github.com/john-cd/rust_howto/issues/1283) — **P4** — finance/Cargo.toml: RustQuant: heavy build (polars, nalgebra, etc) — **NOT STARTED; DEFERRED**

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
