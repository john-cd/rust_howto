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

- [x] REVIEW meaningful, not generic, README.md - see for example bk\crates\cats\asynchronous\README.md
- [/] review all TODO comments in the codebase and fix
- [/] fix "function xyz is never used". most book examples should have a private main() function and a pub fn run() function + a test that call main(). run() should be called by the main() function of the example.
- [/] all book examples should have a test that calls the private main() function and verifies the expected behavior. also add tests to verify other functions / code items within the exanple. ensure coverage for edge cases and error handling.
- [ ] review dirty worktrees, create commits, integrate useful branches, discard stale work, and clean up topic refs
- [ ] integrate all useful topic branches into develop and discard the rest

### Scripts, Tools

- [x] add just command for cargo fmt-toml; update Dockerfile and install scripts
- [/] verify that all .sh files can run in Git Bash (all 60 passed `bash -n`; runtime-dependent commands are deferred).

### Code Examples

- [/] improve comments within examples. add context and explanations where necessary. end with a period.
- [ ] WAIT move code examples from later/ to bk/ when code + text ready

---

## In progress

Linux tooling validation (2026-10-27): the previously unavailable `book_tooling` revision `404e8fe0d4adc40517b0b40a2536cc6833b29059` is published on `codex/rust-howto-pin-404e8fe`. In native Linux slot `a`, `tools` all-features Clippy passed and `just nta` passed 159 tests and doctests after isolating the mdBook fixture from the optional Pandoc renderer, surfacing doctest failures, and refreshing the alphabetical index. Full repository and container gates remain outstanding.

Windows tooling validation (2026-10-27): `tools` all-features Clippy and `just nta` passed 159 tests and doctests with `RUSTC_WRAPPER=''` and `CARGO_INCREMENTAL=1`. The first test attempt failed because the sccache server could not exchange data; the shared cache still needs investigation.

Tooling validation (2026-10-27), issue [#1997](https://github.com/john-cd/rust_howto/issues/1997): slot `e` contains in-progress fixes for optional submodule parsing, aggregate recipe failure reporting, Windows recipe-list path quoting, and Linux native prerequisites. Windows and WSL workspace listings run except for the uninitialized `mdbook-utils` submodule, and deliberately failing aggregate runs return nonzero on both platforms. NASM was installed on Windows; NASM and Ninja were installed in Ubuntu. Windows Docker Compose and Dockerfile checks pass; Bash syntax checks pass on Windows and WSL. Docker Desktop does not expose a Docker CLI in this WSL distro. Windows `just bb` passed with NASM on PATH; linkcheck produced warnings, indexing reported a version mismatch, and the optional Pandoc backend was unavailable. Today, cargo-fmt-toml 1.0.5 is installed on Windows and Ubuntu; `just fmt-toml` and `just fmt-toml-check` cover 11 independent workspaces and pass on both platforms. Root `just fmt-toml-check` runs in devcontainer CI; dprint excludes Cargo manifests and passes, and rumdl passes all 1,123 book/staging Markdown files on both platforms. All repository justfiles parse, list, and pass format checks on both platforms; PowerShell and Bash syntax checks pass on Windows, and Bash syntax checks pass in WSL. The full container build remains blocked by invalid GitHub authentication for private `book_tooling`; broader runtime/full-workspace validation remains.

B1 implementation commits are integrated into local `develop` through `7761c5e6c` (not pushed). Seven B1 issues are closed: #1941, #1940, #939, #1252, #1281, #1912, and #1950. #912 and #1058 remain open because their done criteria require PR links; #864 remains open because its human-review marker is preserved. Formatting, focused tests/Clippy, reference checks, Markdown lint, mdBook build/linkcheck, sitemap generation, static asset copy, and `git diff --check` passed. `just bb` passed in the elevated run; mdBook reported an indexing-plugin version warning and the optional Pandoc backend was unavailable.

Selected topic branches are integrated linearly into local `develop` through `fa58e2dba` (not pushed). This includes the async/concurrency examples and the science changes for #835, #837, #473, #839, and #840; the existing NIfTI example was retained alongside the new neuron simulation. GitHub marks #473 closed. Issues #835, #837, #839, and #840 remain open because their done criteria require PR links. Focused Clippy and tests passed for the changed science packages (including Candle), async/concurrency packages, MIME and GraphQL examples, and link-generation fixture. The full all-features Clippy gate is blocked by the missing Oracle `OCI_LIB_DIR`; the broad `development_tools_debugging` test run still fails in existing log examples that initialize the process-wide logger together.

Validation (2026-10-27): `dprint check` and `rumdl` across `bk/src`, `bk/drafts`, and `later/src` pass. All-features Clippy passes for `tools`, `xmpl`, `playground`, `publish`, the isolated WASM, Vizia, Python-extension, and embedded workspaces; the embedded all-target tests pass. The PostgreSQL-feature database package also passes Clippy with the Cornucopia client pinned to 0.4.0. GUI all-target Clippy passes with default features after disabling the stale Tauri build hook, keeping the disabled Tao source out of auto-discovered targets, and fixing Taffy `f32` literals. Full all-features Clippy for `bk` and the main `later` workspace remains blocked by the Windows `aws-lc-sys` 0.46.0 MSVC archive failure (LNK1114, error 5). The `just` entry point cannot start Git Bash in this session because MSYS cannot create its `BaseNamedObjects` directory.

### Code examples

#### ACTIVE-CODE-01 — Networking, authentication and email

- [#812](https://github.com/john-cd/rust_howto/issues/812) — **P2** — Finish and locally verify the Pingora reverse-proxy example — **PARTIAL; OPEN — isolated test, strict Clippy, and localhost forwarding pass with the pinned Pingora Tokio 1.54 fix; workspace Cargo gates blocked by #1998; PR link remains required**

#### ACTIVE-CODE-02 — Databases and service clients

- [#1021](https://github.com/john-cd/rust_howto/issues/1021) — **P3** — Finish the Oracle example and document its optional Instant Client test — **PARTIAL; OPEN — Oracle test cannot link; live service validation pending**

#### ACTIVE-CODE-04 — Cryptography and Windows APIs

- [#705](https://github.com/john-cd/rust_howto/issues/705) — **P3** — Finish the X.509 certificate example — **PARTIAL; OPEN — book build blocked; human review pending**
- [#822](https://github.com/john-cd/rust_howto/issues/822) — **P3** — Review and test the Windows API example — **PARTIAL; OPEN — book build blocked; human review pending**

#### ACTIVE-CODE-05 — Parsing and XML

- [#826](https://github.com/john-cd/rust_howto/issues/826) — **P3** — Finish and test the Pest arithmetic example — **PARTIAL; OPEN — book build blocked; human review pending**
- [#1102](https://github.com/john-cd/rust_howto/issues/1102) — **P3** — Finish the XML parser example — **PARTIAL; OPEN — XML test blocked by onig_sys LNK1114; human review pending**

#### ACTIVE-CODE-07 — Scheduling and sandboxed runtimes


#### ACTIVE-CODE-09 — ORMs, GraphQL, metrics and decision tables

- [#864](https://github.com/john-cd/rust_howto/issues/864) — **P3** — Finish Async-GraphQL with deterministic schema/query tests — Everyday examples and concrete fixes chat — **IMPLEMENTED ON LOCAL DEVELOP; OPEN — HUMAN REVIEW MARKER PRESERVED**

### Prose and book structure

#### ACTIVE-PROSE-01 — Foreign function interfaces


### Tools and infrastructure

#### ACTIVE-INFRA-01 — Preprocessors, references and recipe portability

- [#1253](https://github.com/john-cd/rust_howto/issues/1253) — **P3** — Verify the supported `just` version and modular recipe behavior — **VERIFIED AND COMPLETE — Windows and Linux just 1.59.0 verified across all workspace justfiles and script modules**

## Available for Codex now

### Code examples

#### NOW-CODE-01 — Build scripts, Cargo and procedural macros

- [#901](https://github.com/john-cd/rust_howto/issues/901) — **P3** — build_time_tooling/cc_defines1.rs: finish; deal with extern — **NOT STARTED, BLOCKED**

#### NOW-CODE-02 — Foreign function interfaces

- [#1751](https://github.com/john-cd/rust_howto/issues/1751) — **P3** — Verify FFI platform prerequisites and demonstrate Python module invocation — **PARTIAL; OPEN — Windows JDK, Ruby DevKit, Python, and Maturin installed; docs and ignored integration test added; isolated Python extension, Java JNI, and Ruby runs pass; package checks await #1998 lock integration**
- [#1032](https://github.com/john-cd/rust_howto/issues/1032) — **P4** — Finish the Node N-API example J — **IN PROGRESS — napi-derive 3 and N-API async exports added; Node 24/MSVC available;book-gate pending**
- [#1033](https://github.com/john-cd/rust_howto/issues/1033) — **P4** — node/neon.rs: finish how to test — **IN PROGRESS — host-independent Neon path now returns anyhow::Result and has non-ignored tests; chapter documents npm init neon and an npm test JavaScript addon check; book-gate pending**
- [#1034](https://github.com/john-cd/rust_howto/issues/1034) — **P4** — objc/objc2.rs: finish; fix — **DEFERRED — macOS validation unavailable; blocker recorded on the issue**
- [#1035](https://github.com/john-cd/rust_howto/issues/1035) — **P4** — ruby/magnus.rs: finish — **IN PROGRESS — Magnus embedding now registers a Rust function, calls it from Ruby, and checks the returned string; extension-mode guidance and ignore-test commands documented; final Ruby package/book-gate pending**
- [#1037](https://github.com/john-cd/rust_howto/issues/1037) — **P4** — examples/uniffi.rs: finish — **DEFERRED — native compiler/linker unavailable; blocker recorded on the issue**

#### NOW-CODE-03 — Desktop UI, accessibility, games and GPU

- [#768](https://github.com/john-cd/rust_howto/issues/768) — **P4** — game_engines/fyrox.rs: finish — **BLOCKED — feature build cost and native platform dependencies need verification**
- [#772](https://github.com/john-cd/rust_howto/issues/772) — **P4** — Finish the wgpu compute example G — **BLOCKED — GPU adapter or configured software adapter and Naga validation dependency needed**
- [#775](https://github.com/john-cd/rust_howto/issues/775) — **P4** — Finish the Dioxus example G — **BLOCKED — SSR test requires the undeclared `dioxus-ssr` crate and feature validation**
- [#777](https://github.com/john-cd/rust_howto/issues/777) — **P4** — Finish the femtovg example G — **BLOCKED — OpenGL context and GPU/window validation needed**
- [#780](https://github.com/john-cd/rust_howto/issues/780) — **P4** — Finish the GTK4 example G — **BLOCKED — GTK4 development SDK is unavailable on the current Windows host**
- [#786](https://github.com/john-cd/rust_howto/issues/786) — **P4** — 2d_renderers/skia_safe.rs: finish — **BLOCKED — Skia requires external binary downloads or a C++ source build**
- [#792](https://github.com/john-cd/rust_howto/issues/792) — **P4** — 2d_renderers/vger.rs: finish — **BLOCKED — upstream maintenance and WGPU API compatibility review needed**
- [#793](https://github.com/john-cd/rust_howto/issues/793) — **P4** — 2d_renderers/webrender.rs: finish — **BLOCKED — required file deletion would remove its human-only TODO Reviewed marker**
- [#1047](https://github.com/john-cd/rust_howto/issues/1047) — **P4** — 2d_renderers/main.rs: review — **BLOCKED — WebRender removal is held by the human-only marker constraint in #793**
- [#1048](https://github.com/john-cd/rust_howto/issues/1048) — **P4** — clipboard/main.rs: review — **CLIPPY CLEAN; OPEN — entry-point test ignored without a desktop clipboard; run it in a desktop session**
- [#1052](https://github.com/john-cd/rust_howto/issues/1052) — **P4** — retained_mode_gui/vizia.rs: finish — **BLOCKED — current Vizia API differs from the requested model/lens design; feature validation needed**
- [#1056](https://github.com/john-cd/rust_howto/issues/1056) — **P4** — window_creation/baseview.rs: write; review — **BLOCKED — unpinned external Git dependency needs upstream review and feature-build validation**

#### NOW-CODE-04 — Machine learning, data and robotics

- [#764](https://github.com/john-cd/rust_howto/issues/764) — **P4** — quant/rustquant.rs: finish; review — **DEFERRED — #1283 keeps RustQuant out, and the specified standalone rewrite requires adding the external `statrs` crate; resume when new dependency work is in scope**
- [#843](https://github.com/john-cd/rust_howto/issues/843) — **P4** — robotics_frameworks/openrr.rs: write — **BLOCKED — upstream describes OpenRR as heavily under development and ROS 2 support as experimental; the current dependency graph includes affected `fast-float` and `serde_with` releases; confirm a portable feature path and native prerequisites**
- [#845](https://github.com/john-cd/rust_howto/issues/845) — **P4** — Add the Zenoh robotics example I — **BLOCKED — the current workspace dependency graph includes affected `lz4_flex 0.10`; feature/example registration is disabled until the advisory is resolved**
- [#1929](https://github.com/john-cd/rust_howto/issues/1929) — **P4** — Validate the ONNX inference example with ort A — **BLOCKED — ONNX Runtime binary download prerequisite and multi-example chapter work remain**

#### NOW-CODE-05 — Recipe batches with accompanying prose

- [#1970](https://github.com/john-cd/rust_howto/issues/1970) — **P4** — Follow up on the web examples and pages — **BLOCKED — dedicated localization crate, Leptos wasm32 dependency split, and full book/toolchain gates remain**
- [#1972](https://github.com/john-cd/rust_howto/issues/1972) — **P4** — Validate the Arrow IPC, optimization, and AWS SDK examples — **BLOCKED — MinIO/LocalStack, Windows NASM, full validation, and human review remain**
- [#1974](https://github.com/john-cd/rust_howto/issues/1974) — **P4** — Follow up on Excel, profiling, geo, audio, wry, and egui examples — **PARTIAL — egui_kittest example already exists; HRTF fixture, native audio alignment, Wry display validation, and full gates remain**
- [#1977](https://github.com/john-cd/rust_howto/issues/1977) — **P4** — Finish the remaining dev-tools, parsing, text, and testing examples — **BLOCKED — external crate integrations, rust-analyzer, and full book/toolchain validation remain; unclaimed subtasks only**
- [#1986](https://github.com/john-cd/rust_howto/issues/1986) — **P4** — Finish remaining cloud recipe batches — **BLOCKED — external services, native GUI validation, dependency work, and parent-issue coordination remain; unclaimed subtasks only**
- [#1991](https://github.com/john-cd/rust_howto/issues/1991) — **Unprioritized** — Resolve local recipe-batch follow-ups — **BLOCKED — crypto dependency mismatch, external databases, Linux, Cap'n Proto tooling, and native compilers remain**

#### NOW-CODE-06 — Examples derived from repository tools

- [#41](https://github.com/john-cd/rust_howto/issues/41) — **P4** — Make additional examples out of mdbook-utils B — **IMPLEMENTED ON DEVELOP; OPEN — offline feature scripts documented; Cargo example-crate acceptance and live broken-link check omitted; focused checks, book build, and PR link pending**
- [#43](https://github.com/john-cd/rust_howto/issues/43) — **P4** — Make additional examples out of the templ tool B — **PARTIAL ON DEVELOP — offline help, Rust-by-Example, and known-category badge examples added; crates.io-backed commands and final gates remain**

#### NOW-CODE-07 — WebAssembly and browser applications

- [#1944](https://github.com/john-cd/rust_howto/issues/1944) — **P4** — Add examples: Leptos web app building blocks (leptos_router, gloo-net, gloo-storage, icons, gRPC-Web client) — **BLOCKED — route/fetch/storage/gRPC examples exist; icondata example, Leptos browser dependency split tracked by #1970, and wasm/book validation remain**

#### NOW-CODE-08 — Media encoding, images and video

#### NOW-CODE-09 — Networking, authentication and email


#### NOW-CODE-10 — Cloud services and SDKs

#### NOW-CODE-11 — Tabular data and finance

#### NOW-CODE-12 — Embedded systems and hardware interfaces

### Prose and book structure

#### NOW-PROSE-01 — Systems, embedded, WebAssembly and virtualization

- [#348](https://github.com/john-cd/rust_howto/issues/348) — **P4** — emulators/emulators.md: write — **IMPLEMENTED ON DEVELOP; OPEN — bounded register VM, Intel HEX Reader example, PolkaVM guidance, and references present; final Cargo/book/reference/spelling checks and PR link pending**
- [#349](https://github.com/john-cd/rust_howto/issues/349) — **P4** — emulators/index.md: write — **IMPLEMENTED ON DEVELOP; OPEN — short landing page and recipe table integrated; reference, book, spelling checks and PR link pending**
- [#426](https://github.com/john-cd/rust_howto/issues/426) — **P4** — no-std/no_std.md: no_std: write — **IMPLEMENTED ON DEVELOP; OPEN — allocator, panic-handler, target-build, and checksum guidance present; final reference/book/spelling checks and issue-link removal pending**
- [#428](https://github.com/john-cd/rust_howto/issues/428) — **P4** — no-std_no-alloc/no_alloc.md: no_alloc: write — **IMPLEMENTED ON DEVELOP; OPEN — fixed-capacity collections, formatting, SPSC queues, static pools, and slice serialization documented; final reference/book/spelling checks and issue-link removal pending**
- [#433](https://github.com/john-cd/rust_howto/issues/433) — **P4** — os_freebsd-apis/freebsd.md: freebsd: write — **PARTIAL ON DEVELOP; OPEN — chapter and #817 sysctl example present; FreeBSD CI, final target/book/reference/spelling validation, and PR link remain**
- [#434](https://github.com/john-cd/rust_howto/issues/434) — **P4** — os_linux-apis/linux.md: linux: write — **IMPLEMENTED ON DEVELOP; OPEN — chapter and #818 statx example present; Linux test, final book/reference/spelling checks, and PR link pending**
- [#435](https://github.com/john-cd/rust_howto/issues/435) — **P4** — os_macos-apis/macos.md: macos: write — **PARTIAL ON DEVELOP; OPEN — chapter and safe #819 objc2 example present; later/crates macOS CI inclusion, final native/book/reference/spelling validation, and PR link remain**
- [#496](https://github.com/john-cd/rust_howto/issues/496) — **P4** — wasm/wasm_standalone_runtimes.md: others: organize/write — **DEFERRED — current support checks for six runtimes and Wasmer/Wasmtime example builds/tests are held for the final validation pass; blocker details are on the issue**

#### NOW-PROSE-02 — UI, accessibility, media and rendering

- [#369](https://github.com/john-cd/rust_howto/issues/369) — **P4** — game-engines/game_engines.md: game_engines: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#375](https://github.com/john-cd/rust_howto/issues/375) — **P4** — rendering_graphics-api/gpu_abstraction_layers.md: webgpu: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#377](https://github.com/john-cd/rust_howto/issues/377) — **P4** — rendering/2d_renderers.md: 2d_renderers: write; titles — **BLOCKED — #792 Vger status/compatibility and #793 WebRender disposition unresolved**
- [#381](https://github.com/john-cd/rust_howto/issues/381) — **P4** — gui/file_dialogs.md: file_dialogs: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#383](https://github.com/john-cd/rust_howto/issues/383) — **P4** — gui/gtk.md: gtk: write — **BLOCKED — GTK 4 development SDK and pkg-config metadata unavailable**
- [#385](https://github.com/john-cd/rust_howto/issues/385) — **P4** — gui/immediate_mode_gui.md: immediate_mode_gui: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#389](https://github.com/john-cd/rust_howto/issues/389) — **P4** — gui/retained_mode_gui.md: retained_mode_gui: organize/write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — book and reference checks and the Slint-feature check remain; stale undeclared Tauri build-hook reference removed**
- [#394](https://github.com/john-cd/rust_howto/issues/394) — **P4** — gui/web_based_gui.md: web_based_gui: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#396](https://github.com/john-cd/rust_howto/issues/396) — **P4** — Write the window creation chapter G — **BLOCKED — #1056 baseview disposition and new softbuffer dependency review pending**
- [#413](https://github.com/john-cd/rust_howto/issues/413) — **P4** — multimedia/multimedia.md: multimedia: write — **BLOCKED — #805 playback example is unfinished; a distinct cross-media scope remains**
- [#417](https://github.com/john-cd/rust_howto/issues/417) — **P4** — multimedia_encoding/encoding.md: encoding: write — **BLOCKED — symphonia, hound, and mp4 dependencies are not in the workspace**
- [#420](https://github.com/john-cd/rust_howto/issues/420) — **P4** — multimedia_images/images.md: images: write — **BLOCKED — required example split and EXIF/oxipng dependency review in #808 are incomplete**
- [#455](https://github.com/john-cd/rust_howto/issues/455) — **P4** — rendering_engine/rendering_engines.md: rendering_engines: write — **BLOCKED — #955 category pointer decision and #830 example disposition remain unresolved**
- [#457](https://github.com/john-cd/rust_howto/issues/457) — **P4** — rendering_graphics-api/native_graphics_apis.md: graphics_apis: write — **BLOCKED — required wgpu adapter-info example in #831 remains a stub**
- [#608](https://github.com/john-cd/rust_howto/issues/608) — **P4** — gpu/index.md: write — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — Cargo lockfile reconciled; book and reference checks pending**
- [#934](https://github.com/john-cd/rust_howto/issues/934) — **P4** — gui/ui_layout.md: review — **IMPLEMENTED ON DEVELOP; VALIDATION PENDING — focused GUI all-target Clippy passes; book and reference checks remain**

#### NOW-PROSE-03 — Science, finance, data processing and visualization

- [#257](https://github.com/john-cd/rust_howto/issues/257) — **P4** — computer-vision/opencv.md: opencv: expand, add example — **BLOCKED — OpenCV/libclang SDKs unavailable in this environment**
- [#364](https://github.com/john-cd/rust_howto/issues/364) — **P4** — finance/quant.md: quant: write — **DEFERRED — requested `financial`, `rand`, and `rand_distr` examples require new dependencies; skipped under the no-external-dependency constraint**
- [#474](https://github.com/john-cd/rust_howto/issues/474) — **P4** — science_geo/geo.md: geo: write — **BLOCKED — GeoJSON dependencies and PROJ C library required**
- [#477](https://github.com/john-cd/rust_howto/issues/477) — **P4** — science_robotics/robot_operating_systems.md: robotics: organize — **BLOCKED — ROS 2 client/example issue #844 pending**
- [#494](https://github.com/john-cd/rust_howto/issues/494) — **P4** — `visualization/visualization.md`: visualization: write — **BLOCKED — Plotters dependency required for SVG example**
- [#1207](https://github.com/john-cd/rust_howto/issues/1207) — **P4** — science_robotics/perception_and_sensors.md: write — **PARTIAL ON DEVELOP; BLOCKED — `kiddo` example dependency unavailable**
- [#1211](https://github.com/john-cd/rust_howto/issues/1211) — **P4** — science_robotics/hardware_integration.md: write — **PARTIAL ON DEVELOP; BLOCKED — `embedded-hal-mock` example dependency unavailable**

#### NOW-PROSE-04 — Publishing, curation and book coverage

- [#1720](https://github.com/john-cd/rust_howto/issues/1720) — **P3** — Curate remaining VS Code, browser and TOREVIEW bookmarks into book content — **BLOCKED — the Chrome bookmarks export (`bookmarks.json`) is absent from this checkout; request the source before further curation**
- [#1957](https://github.com/john-cd/rust_howto/issues/1957) — **Unprioritized** — Audit references and links in restored book pages A — **PARTIAL — regenerated restored-page references and verified the wikilink targets exist; four graphics aliases match both an index and a detail page, and separate draft/later builds remain blocked by missing book configurations**

#### NOW-PROSE-05 — Cloud platforms and containers

- [#576](https://github.com/john-cd/rust_howto/issues/576) — **P4** — cloud/rust_native_cloud_development.md: write — **BLOCKED — Shuttle's official notice says hosted operations are ceasing and its old pricing is stale; the Shuttle-specific example needs a replacement target decision**
- [#988](https://github.com/john-cd/rust_howto/issues/988) — **P4** — virtualization/containers.md: review/write — **PARTIAL — `containerization.md` is nested under Containers; blocked on #1224's missing Docker client/test dependencies before adding the thin `using_containers.md` sibling and finalizing the section structure**
- [#1224](https://github.com/john-cd/rust_howto/issues/1224) — **P4** — virtualization/using_containers.md: write / fix — **BLOCKED — required `bollard` and `testcontainers` dependencies are absent; the integration also requires a Docker daemon, as noted in the issue comment**

#### NOW-PROSE-06 — Security and cryptocurrencies

#### NOW-PROSE-07 — Foreign function interfaces - IN PROGRESS

- [#1071](https://github.com/john-cd/rust_howto/issues/1071) — **P4** — development-tools_ffi/flutter.md: write; review in depth — **IMPLEMENTED AND REVIEWED; OPEN — focused checks passed; full book gate and PR link pending**
- [#1075](https://github.com/john-cd/rust_howto/issues/1075) — **P4** — development-tools_ffi/objc.md: write — **IMPLEMENTED; OPEN — Windows checks passed; native runtime, full book gate and PR link pending**

#### NOW-PROSE-08 — Web, networking and databases

- [#290](https://github.com/john-cd/rust_howto/issues/290) — **P3** — database-implementations/databases.md: databases: expand / write — **BLOCKED — requires adding and validating a new `redb` or `fjall` dependency and tempfile-backed example; deferred per request**
- [#1067](https://github.com/john-cd/rust_howto/issues/1067) — **P3** — database/mssql.md: organize / write — **BLOCKED — Docker CLI is installed, but Docker API access fails with `permission denied`; SQL Server service validation is unavailable**
- [#1068](https://github.com/john-cd/rust_howto/issues/1068) — **P3** — database/nosql.md: review — **IMPLEMENTED ON LOCAL DEVELOP — focused database Clippy and 9/9 portable tests passed (4 external-service tests skipped); draft lint/spelling and generated crate refs passed; full book gate and PR link pending**
- [#1069](https://github.com/john-cd/rust_howto/issues/1069) — **P3** — database/oracle.md: write — **BLOCKED — Oracle Instant Client is unavailable (`OCI_LIB_DIR` unset and `sqlplus` absent); Docker API access is denied, so the required local Oracle service cannot be validated**

Validation (2026-10-27, NOW-PROSE-08): focused database Clippy and nextest passed; the test run passed 9 tests and skipped four ignored external-service tests. `dprint check`, `rumdl` across `bk/drafts`, `typos` across the book and future content, and the generated dependency-reference check passed. The local draft book gate remains blocked: Git Bash cannot create its MSYS `BaseNamedObjects` directory, the direct tools release build fails in `aws-lc-sys` with MSVC LNK1114/error 5, and Docker API access is denied. The Windows `just refdefs check` recipe is unavailable; its Linux-only script could not run. Issues remain open because full book/reference gates and PR links are not complete.

#### NOW-PROSE-09 — Development tools, build systems and editors — IMPLEMENTED; FULL VALIDATION BLOCKED

Validation (2026-10-26): dprint, rumdl, `just spell`, changed-page references and local fragments, focused mdBook rendering, rendered tables/anchors, and `git diff --check` passed. All three YAML workflows passed Zizmor's offline audit. `just bb` is blocked before rendering because `cargo tree --locked` needs to update the existing `bk/crates/Cargo.lock`; no Cargo inputs changed in this batch. `just refdefs check` is Unix-only; running its script through Git Bash reports existing unresolved global labels, while all labels used by the two completed chapters resolve. Issues remain open pending the full book gate. Commits are local; no push was requested.

#### NOW-PROSE-10 — Localization and internationalization - IN PROGRESS

- [#404](https://github.com/john-cd/rust_howto/issues/404) — **P4** — localization/localization.md: localization: write — **IMPLEMENTED ON DEVELOP; OPEN — PR link pending**

#### NOW-PROSE-11 — Asynchronous programming and caching - IN PROGRESS

- Focused actor Clippy, four actor tests, dprint, rumdl, chapter spelling, and direct refdefs check pass. `just ca`, `just ntt actors`, `just bb`, and `just spell` stop at the Git Bash `NtCreateDirectoryObject` access error; `just refdefs check` is not a defined recipe. Direct all-target Clippy also reports a pre-existing semicolon lint in `message_passing_crossbeam_channel.rs`.

#### NOW-PROSE-12 — Aerospace and device protocols — IMPLEMENTED; FULL VALIDATION BLOCKED

Validation (2026-10-26): Markdown formatting, lint, spelling, and whitespace checks passed. Focused tests and all-target Clippy passed for the aerospace simulation and space-protocol examples. The complete book/reference gates remain blocked: the direct reference check reaches `bk/crates` but `cargo tree --locked` requires an existing lockfile update; Git Bash-backed `just bb` cannot start because of the `NtCreateDirectoryObject` access denial. A direct mdBook build reports repository-wide unresolved links, and optional `mdbook-pandoc` is unavailable. Issues remain open pending complete book gates and PR links.

### Tools and infrastructure

#### NOW-INFRA-01 — Preprocessors, references and recipe portability

- [#248](https://github.com/john-cd/rust_howto/issues/248) — **P1** — replace current badge markdown copy-pasted text by a preprocessor command {#crate ...} {#categories... } — **BLOCKED — waiting on full-page parser fixes in external book_tooling#34/#47 and rendered publication validation**
  - The sorting-table migration and representative rendering are integrated; broad filesystem-page conversion was reverted after raw badge directives remained in output. Resume after the parser fix is available, then verify directive expansion, rendered output parity, index anchors, and idempotence before closing.

#### NOW-INFRA-02 — Dependencies, CI and containers

  — **P1** — Port remaining examples across major dependency upgrades — **PARTIAL**
  - Resolved the `libsqlite3-sys` links collision and `smallvec` 1.16/Kube
    resolver conflict; corrected Naga 26/27 aliases and aligned tested SeaORM,
    media, and Linfa examples. Windows checks pass; Ubuntu database tests pass
    (8/8) with strict Clippy, and Kube/cloud plus GUI/Parley examples compile.
    Remaining ports and full workspace gates still need completion.

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

- [#810](https://github.com/john-cd/rust_howto/issues/810) — **P3** — Run native Linux tests and strict Clippy for the Glommio example — **IMPLEMENTED ON DEVELOP; BLOCKED — Linux-only tests/Clippy cannot run here; WSL startup returns `Wsl/E_ACCESSDENIED`; the issue comment records the native Linux follow-up**
- [#1946](https://github.com/john-cd/rust_howto/issues/1946) — **P3** — Add examples: read mail over IMAP and filter disposable addresses before sending (imap, mailchecker) — **IMPLEMENTED ON DEVELOP; OPEN — IMAP/parser and mailchecker examples, pages, tables, and refdefs are present; live IMAP requires a server and credentials; offline tests, Clippy, refdefs, and book build deferred to final validation; details are on the issue**

#### VALIDATE-CODE-02 — Databases and service clients

- [#1022](https://github.com/john-cd/rust_howto/issues/1022) — **P2** — Finish the Sibyl example and review installation steps — **IMPLEMENTED ON DEVELOP; BLOCKED — Oracle Instant Client is unavailable (`OCI_LIB_DIR` required); Linux/Windows installation and Oracle service checks remain; details are on the issue**
- [#1162](https://github.com/john-cd/rust_howto/issues/1162) — **P2** — Review the PostgreSQL aggregate_data example — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused target tests/Clippy passed; the aggregate query test needs configured PostgreSQL; full book/native-dependency gates remain; details are on the issue**
- [#708](https://github.com/john-cd/rust_howto/issues/708) — **P3** — Finish the Cornucopia example B — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused build/tests/Clippy and live PostgreSQL checks passed; full Clippy/book gates remain unresolved; details are on the issue**

#### VALIDATE-CODE-03 — Compression and archives

#### VALIDATE-CODE-04 — Build scripts, Cargo and procedural macros

#### VALIDATE-CODE-05 — Foreign function interfaces

#### VALIDATE-CODE-06 — Desktop UI, accessibility, games and GPU

- [#1155](https://github.com/john-cd/rust_howto/issues/1155) — **P3** — `clipboard/arboard.rs`: review — **IMPLEMENTED ON DEVELOP; BLOCKED — prior GUI package tests passed; ignored desktop clipboard run, full Clippy, and issue-linked PR remain; details are on the issue**
- [#1952](https://github.com/john-cd/rust_howto/issues/1952) — **P4** — Add example: show a local web page in a native window (wry, tao) — **IMPLEMENTED ON DEVELOP; OPEN — native webview example, URL allow-list coverage, and platform prose are present; manual window run and refdefs/book checks remain; details are on the issue**

#### VALIDATE-CODE-07 — Machine learning, data and robotics

- [#1079](https://github.com/john-cd/rust_howto/issues/1079) — **P4** — Review and expand the OpenCV example — **IMPLEMENTED ON DEVELOP; BLOCKED — Linux/OpenCV/libclang tests and Clippy require a Linux host; WSL access and Docker Linux engine are unavailable; details are on the issue**
- [#1942](https://github.com/john-cd/rust_howto/issues/1942) — **P4** — Add examples: geocode addresses and compute distances (geocoding, haversine, us-state-info) — **IMPLEMENTED ON DEVELOP; OPEN — distance/state/cache examples and fake-geocoder coverage are present; live provider access is deliberately excluded; required refdefs/book checks deferred to final validation**

#### VALIDATE-CODE-08 — Concurrency

#### VALIDATE-CODE-09 — Tabular data and finance

- [#1949](https://github.com/john-cd/rust_howto/issues/1949) — **P4** — Add examples: finance building blocks (option pricing, rolling statistics, trading calendars, market-data and broker clients) — **PARTIAL ON DEVELOP; BLOCKED — pricing, statistics, Yahoo fixture, and pages are present; calendar code uses supplied holidays rather than the requested exchange-calendar integration; provider access and final checks deferred; remaining work is on the issue**

#### VALIDATE-CODE-10 — Cloud services and SDKs

- [#1938](https://github.com/john-cd/rust_howto/issues/1938) — **P4** — Add examples: AWS SDK for Rust basics (shared config, SES v2 email, CloudWatch metrics, invoke Lambda, S3-compatible endpoints) — **PARTIAL ON DEVELOP; BLOCKED — SDK examples/pages are present; noted CloudWatch/Prometheus comparison and custom TLS guidance remain; live AWS/S3-compatible checks need credentials/services; final checks deferred; remaining work is on the issue**

#### VALIDATE-CODE-11 — Algorithms, data structures and memory

- [#1925](https://github.com/john-cd/rust_howto/issues/1925) — **P4** — Add examples: memory techniques: arena allocation (bumpalo), string interning (ustr), feature-selected global allocator — **PARTIAL ON DEVELOP; BLOCKED — arena/interner/allocator examples and pages are present; allocation measurement and strict mutually exclusive feature selection remain; selection conflicts with the current all-features gate, and jemalloc needs a non-MSVC host; details are on the issue**

#### VALIDATE-CODE-12 — Language and domain models

#### VALIDATE-CODE-13 — Testing and observability

#### VALIDATE-CODE-14 — Text processing and HTML

#### VALIDATE-CODE-15 — Parsing and XML

#### VALIDATE-CODE-16 — Filesystem and process management

- [#1922](https://github.com/john-cd/rust_howto/issues/1922) — **P3** — Add examples: manage child processes (kill a process tree, resource limits, sandboxed workers) — **IMPLEMENTED ON DEVELOP; BLOCKED — process groups, Windows Job Objects, timeout and kill guard examples, confinement guidance, and book page are present; Linux/Windows and required full gates remain; details are on the issue**

### Prose and book structure

#### VALIDATE-PROSE-01 — Language, mathematics, date and time

- [#1188](https://github.com/john-cd/rust_howto/issues/1188) — **P2** — `date-and-time/index.md`: review in depth — **IMPLEMENTED ON DEVELOP; BLOCKED — humantime and guidance are present; book validation and human review/promotion remain; details are on the issue**
- [#490](https://github.com/john-cd/rust_howto/issues/490) — **P4** — Write the number-formatting chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused tests/Clippy and documentation checks passed; full book build remains; details are on the issue**

#### VALIDATE-PROSE-02 — Asynchronous programming and caching

- [#633](https://github.com/john-cd/rust_howto/issues/633) — **P2** — `asynchronous/async.md`: write; review in depth — **IMPLEMENTED ON DEVELOP; BLOCKED — async fundamentals and runtime guidance are present; prior focused tests/Clippy and book build passed; full Clippy and Windows spelling gate remain; details are on the issue**

#### VALIDATE-PROSE-03 — Development tools, build systems and editors

- [#311](https://github.com/john-cd/rust_howto/issues/311) — **P2** — `development-tools_cargo-plugins/index.md`: review in depth, reorg table — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#962](https://github.com/john-cd/rust_howto/issues/962) — **P2** — `text-editors/index.md`: write; review in depth finish the table — **IMPLEMENTED ON DEVELOP; BLOCKED — reference/spelling and Windows AWS-LC book gates remain; details are on the issue**
- [#604](https://github.com/john-cd/rust_howto/issues/604) — **P4** — Write the release automation chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — direct mdBook and documentation checks passed; Windows book wrapper remains; details are on the issue**

#### VALIDATE-PROSE-04 — Encoding, parsing and text processing

- [#930](https://github.com/john-cd/rust_howto/issues/930) — **P2** — `encoding/string_encoding.md`: review — **IMPLEMENTED ON DEVELOP; OPEN — chapter guidance and related links are present; reference includes are outside the topic list; formatted; example checks, spelling, and required final gates remain**
- [#1078](https://github.com/john-cd/rust_howto/issues/1078) — **P3** — `encoding/no_external_schema.md`: write — **PARTIAL ON DEVELOP; BLOCKED — guide is present; stale issue self-reference, full checks, Unix spelling, and linked PR remain; details are on the issue**

#### VALIDATE-PROSE-05 — Web, networking and databases

- [#944](https://github.com/john-cd/rust_howto/issues/944) — **P2** — `network-programming/index.md`: write; add cross-links; review; review `email_address`, `fast_chemail` — **IMPLEMENTED ON DEVELOP; BLOCKED — network index and cross-links are present; crate release and repository activity reviewed on 2026-10-27; refdefs/spelling/book gates and linked PR remain; details are on the issue**
- [#974](https://github.com/john-cd/rust_howto/issues/974) — **P2** — `web-programming_http-client/apis.md`: review / fix — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused Clippy, refdefs, and book build passed; example/spelling and all-features checks remain; details are on the issue**
- [#977](https://github.com/john-cd/rust_howto/issues/977) — **P2** — `web-programming_http-server/index.md`: write / review in depth — **IMPLEMENTED ON DEVELOP; BLOCKED — prior Markdown/refdefs/book checks passed; Unix spelling and linked PR remain; details are on the issue**
- [#284](https://github.com/john-cd/rust_howto/issues/284) — **P3** — Expand and verify the connection-pool examples B — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused and live PostgreSQL checks passed; nextest selector and full gates remain; details are on the issue**
- [#286](https://github.com/john-cd/rust_howto/issues/286) — **P3** — Expand PostgreSQL and Cornucopia guidance B — **IMPLEMENTED ON DEVELOP; BLOCKED — prior focused Clippy and live PostgreSQL checks passed; nextest selector, full gates, and repository-wide refdefs remain; details are on the issue**
- [#291](https://github.com/john-cd/rust_howto/issues/291) — **P3** — — Search engines chapter — **IMPLEMENTED ON DEVELOP; BLOCKED — search overview and comparison are present; Meilisearch service verification and refdefs/book/spelling gates remain; details are on the issue**
- [#1064](https://github.com/john-cd/rust_howto/issues/1064) — **P3** — `database/amqp.md`: finish — **IMPLEMENTED ON DEVELOP; BLOCKED — AMQP guide and example are present; RabbitMQ service and native-build/full gates remain; details are on the issue**

#### VALIDATE-PROSE-06 — Filesystem utilities, shells and compression

- [#239](https://github.com/john-cd/rust_howto/issues/239) — **P3** — `command-line-utilities/shells.md`: shells: expand — **IMPLEMENTED; OPEN — shell and completion guidance, with repository activity reviewed; refdefs, book, spelling, and linked PR remain**
- [#253](https://github.com/john-cd/rust_howto/issues/253) — **P3** — `compression/tar.md`: tar: review — **IMPLEMENTED; OPEN — archive and compression guidance reviewed; #254/#255 acceptance, refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-07 — Security and cryptocurrencies

- [#278](https://github.com/john-cd/rust_howto/issues/278) — **P4** — Complete the cryptocurrency category overview D — **IMPLEMENTED; OPEN — crate release activity reviewed and deprecation linked; refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-08 — Systems, embedded, WebAssembly and virtualization

- [#946](https://github.com/john-cd/rust_howto/issues/946) — **P3** — Write and review the external commands chapter — **PARTIAL; OPEN — stdin description corrected; platform-dependent examples, full checks, and linked PR remain**
- [#1177](https://github.com/john-cd/rust_howto/issues/1177) — **P3** — `hardware-support/peripherals.md`: write — **IMPLEMENTED; OPEN — peripheral guide and overview table present; refdefs, book, spelling, and linked PR remain**
- [#968](https://github.com/john-cd/rust_howto/issues/968) — **P4** — Complete the virtualization index — **PARTIAL; OPEN — table links repaired and vm-memory added; remaining review, refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-09 — UI, accessibility, media and rendering

- [#187](https://github.com/john-cd/rust_howto/issues/187) — **P4** — Complete the screen-reader and AccessKit chapter D — **PARTIAL; BLOCKED — desktop accessibility verification and deferred checks remain; details are on the issue**
- [#422](https://github.com/john-cd/rust_howto/issues/422) — **P4** — Video chapter and example F — **PARTIAL; BLOCKED — native media environment and deferred checks remain; details are on the issue**
- [#638](https://github.com/john-cd/rust_howto/issues/638) — **P4** — Clipboard chapter F — **IMPLEMENTED; OPEN — text/image and platform guidance present; desktop checks, refdefs, book, spelling, and linked PR remain**
- [#942](https://github.com/john-cd/rust_howto/issues/942) — **P4** — Complete the images index — **PARTIAL; OPEN — task table links repaired and image recipes present; example checks, refdefs, book, spelling, and linked PR remain**
- [#943](https://github.com/john-cd/rust_howto/issues/943) — **P4** — Complete the video index — **PARTIAL; OPEN — GStreamer and FFmpeg release status reviewed; external media setup, refdefs, book, spelling, and linked PR remain**
- [#953](https://github.com/john-cd/rust_howto/issues/953) — **P4** — Complete the rendering index — **PARTIAL; OPEN — subpages nested and recipe tables grouped; content review, refdefs, book, spelling, and linked PR remain**
- [#956](https://github.com/john-cd/rust_howto/issues/956) — **P4** — Complete the graphics-API index — **PARTIAL; OPEN — related-topic table repaired; platform review, refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-10 — Science, finance, data processing and visualization

- [#589](https://github.com/john-cd/rust_howto/issues/589) — **P4** — Organize the data-engineering chapter D — **PARTIAL; OPEN — shared Polars recipe deduplicated; external runtime review, refdefs, book, spelling, and linked PR remain**
- [#961](https://github.com/john-cd/rust_howto/issues/961) — **P4** — Complete the simulation index — **PARTIAL; OPEN — malformed domain and related-topic tables repaired; crate review, refdefs, book, spelling, and linked PR remain**
- [#969](https://github.com/john-cd/rust_howto/issues/969) — **P4** — Review the visualization index — **PARTIAL; OPEN — GUI table and three-d backend corrected; crate review, refdefs, book, spelling, and linked PR remain**
- [#1205](https://github.com/john-cd/rust_howto/issues/1205) — **P4** — — Deep learning chapter — **PARTIAL; OPEN — CPU example and framework guidance present; feature build, tests, Clippy, refdefs, book, spelling, and linked PR remain**
- [#1208](https://github.com/john-cd/rust_howto/issues/1208) — **P4** — — Control systems chapter — **PARTIAL; OPEN — PID simulation and sample-period guidance present; crate review, tests, Clippy, refdefs, book, spelling, and linked PR remain**

#### VALIDATE-PROSE-11 — Architecture and scripting

- [#1230](https://github.com/john-cd/rust_howto/issues/1230) — **P4** — — Architecture chapters — **IMPLEMENTATION COMPLETE ON DEVELOP; OPEN — chapter review and formatting complete; tests, book checks, and linked PR deferred**

#### VALIDATE-PROSE-12 — Foreign function interfaces

- [#210](https://github.com/john-cd/rust_howto/issues/210) — **P4** — Finish the Python example and validate its platform requirements D — **PARTIAL; BLOCKED — Linux Python and extension environment verification, full checks, and linked PR remain**

### Tools and infrastructure

#### VALIDATE-INFRA-01 — Dependencies, CI and containers

- [#1699](https://github.com/john-cd/rust_howto/issues/1699) — **P1** — Reconcile seven conflicted PRs against main and the local example changes — **PARTIAL**
- [#1719](https://github.com/john-cd/rust_howto/issues/1719) — **P1** — Repair and document the dev container setup and compiler caching — **PARTIAL — Windows install script and WSL tool inventory pass; Compose, Dockerfile, and prose checks pass; full image build needs a valid GH_TOKEN for private book_tooling. BuildKit secret support is documented; container startup/restart, cache persistence, contributor gates, and cold/warm sccache measurements remain.**
- [#51](https://github.com/john-cd/rust_howto/issues/51) — **P2** — Audit dependencies during CI — **PARTIAL**
- [#1727](https://github.com/john-cd/rust_howto/issues/1727) — **P2** — Benchmark container build storage and select persistent cache mounts — **IMPLEMENTED** — Task 1 done: persistent named volumes (`cargo_home_cache`, `cargo_target_cache`, `cargo_build_cache`) implemented in `compose.override.yaml` for Dev, and `tmpfs` (5G in-RAM mount for `/code/target/`) implemented in `compose-ci.yaml` for CI. Task 2 & 3 done: Linux container benchmarks (`dd if=/dev/zero ... bs=1024 count=100000 conv=fsync`) measured 218 MB/s on bind mount (`/code`), 190 MB/s on container overlay FS (`/tmp`), and 399 MB/s on `tmpfs` (`/dev/shm`), confirming bind mounts on Linux have no virtualization bottleneck (unlike Windows host bind mounts), while named persistent volumes and tmpfs provide optimal cache isolation and compilation speeds. Dev and CI compose files pass `docker compose config --quiet`.
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

- [#831](https://github.com/john-cd/rust_howto/issues/831) — **P4** — native_graphics_apis/wgpu_adapter_info.rs: report a GPU adapter's native backend — **IMPLEMENTED ON DEVELOP; OPEN — `1896f5110` adds a feature-gated headless query with errors; tests and lints skipped by request; build/GPU/book validation and PR link pending**
- [#886](https://github.com/john-cd/rust_howto/issues/886) — **P4** — gpu/rust_gpu.rs: resolve the shader example — **IMPLEMENTED ON DEVELOP; OPEN — `2e0fbca70` documents a separate shader setup and removes the no-op host target; tests and lints skipped by request; book validation and PR link pending**

DEFERRED-CODE-01 is integrated on local `develop`. The #828 and #829 implementations were already present; #830, #831, and #886 were rebased and integrated as focused commits. All five issues have delivery comments and remain open. Formatting and `git diff --check` passed; no builds, tests, or lints were run for this batch.

#### DEFERRED-CODE-02 — Machine learning, data and robotics

#### DEFERRED-CODE-03 — Operating-system APIs, emulation and no-std

- [#818](https://github.com/john-cd/rust_howto/issues/818) — **P4** — linux/statx_metadata.rs: temporary-file size and optional birth time through rustix — **IMPLEMENTED ON DEVELOP (`090e6a956`); tests and lints skipped by request; Linux and book builds pending**
- [#819](https://github.com/john-cd/rust_howto/issues/819) — **P4** — macos/macos.rs: safe objc2 Foundation system-version query and target gating — **IMPLEMENTED ON DEVELOP (`6677bdc9f`); tests and lints skipped by request; macOS and book builds pending**

#### DEFERRED-CODE-04 — Media encoding, images and video

- [#805](https://github.com/john-cd/rust_howto/issues/805) — **P4** — multimedia/multimedia1.rs: write — **PARTIAL; BLOCKED — audio-device validation and device-independent source test remain; details are on the issue**

#### DEFERRED-CODE-05 — Localization and internationalization

- [#796](https://github.com/john-cd/rust_howto/issues/796) — **P4** — internationalization/internationalization1.rs: write — **IMPLEMENTED; OPEN — ICU number and date formatting in two locales; tests, lints, book validation, and linked PR remain**
- [#797](https://github.com/john-cd/rust_howto/issues/797) — **P4** — localization/fluent.rs: write — **IMPLEMENTED; BLOCKED — English/French Fluent example, tests, and chapter updates are drafted; later workspace lock resolution is blocked by #1998's `smallvec` conflict; tests, Clippy, book build, and PR link remain**

#### DEFERRED-CODE-06 — Aerospace and device protocols

- [#61](https://github.com/john-cd/rust_howto/issues/61) — **P4** — drones/drones1.rs: write — **BLOCKED — flight-plan work and real-drone validation remain; details are on the issue**
- [#65](https://github.com/john-cd/rust_howto/issues/65) — **P4** — uavs/uavs.rs: write — **BLOCKED — telemetry decoding and real-drone validation remain; details are on the issue**

### Tools and infrastructure

#### DEFERRED-INFRA-01 — Dependencies, CI and containers

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

## Review: `dead_code` allowances and ignored tests

Added by the repo-wide Clippy pass (declaration-only snippets) and the blocked-issue isolation pass. For human review: each entry should either gain real use in the example, be removed, or keep a documented reason. Regenerate by searching for `dead_code` and `#[ignore`.

### `dead_code` allowances (121)

#### `algorithms`

- [bk/crates/cats/algorithms/examples/rand/rand_custom.rs:13](bk/crates/cats/algorithms/examples/rand/rand_custom.rs#L13)

#### `asynchronous`

- [bk/crates/cats/asynchronous/examples/async_traits/async_traits2.rs:12](bk/crates/cats/asynchronous/examples/async_traits/async_traits2.rs#L12)
- [bk/crates/cats/asynchronous/examples/async_traits/rpitit_send.rs:4](bk/crates/cats/asynchronous/examples/async_traits/rpitit_send.rs#L4)
- [bk/crates/cats/asynchronous/examples/joinset_pool.rs:4](bk/crates/cats/asynchronous/examples/joinset_pool.rs#L4)
- [bk/crates/cats/asynchronous/examples/rpitit_send.rs:4](bk/crates/cats/asynchronous/examples/rpitit_send.rs#L4)

#### `authentication`

- [bk/crates/cats/authentication/examples/oauth_client_credentials/client_credentials.rs:4](bk/crates/cats/authentication/examples/oauth_client_credentials/client_credentials.rs#L4)
- [bk/crates/cats/authentication/examples/session_login/axum_login.rs:4](bk/crates/cats/authentication/examples/session_login/axum_login.rs#L4)
- [bk/crates/cats/authentication/examples/session_login/hashing.rs:4](bk/crates/cats/authentication/examples/session_login/hashing.rs#L4)

#### `code_organization`

- [bk/crates/code_organization/examples/modules/module_visibility.rs:4](bk/crates/code_organization/examples/modules/module_visibility.rs#L4)
- [bk/crates/code_organization/examples/modules/module_visibility.rs:49](bk/crates/code_organization/examples/modules/module_visibility.rs#L49)
- [bk/crates/code_organization/examples/visibility/private_access.rs:4](bk/crates/code_organization/examples/visibility/private_access.rs#L4)
- [bk/crates/code_organization/examples/visibility/pub_keyword.rs:4](bk/crates/code_organization/examples/visibility/pub_keyword.rs#L4)
- [bk/crates/code_organization/examples/visibility/public_access.rs:4](bk/crates/code_organization/examples/visibility/public_access.rs#L4)
- [bk/crates/code_organization/examples/visibility/public_by_default.rs:4](bk/crates/code_organization/examples/visibility/public_by_default.rs#L4)
- [bk/crates/code_organization/examples/visibility/visibility_external_code.rs:4](bk/crates/code_organization/examples/visibility/visibility_external_code.rs#L4)
- [bk/crates/code_organization/examples/visibility/visibility_scope.rs:25](bk/crates/code_organization/examples/visibility/visibility_scope.rs#L25)
- [bk/crates/code_organization/examples/visibility/visibility_scope.rs:36](bk/crates/code_organization/examples/visibility/visibility_scope.rs#L36)

#### `concurrency`

- [bk/crates/cats/concurrency/examples/message_passing/flume_commands.rs:4](bk/crates/cats/concurrency/examples/message_passing/flume_commands.rs#L4)
- [bk/crates/cats/concurrency/examples/message_passing/flume_tick.rs:4](bk/crates/cats/concurrency/examples/message_passing/flume_tick.rs#L4)
- [bk/crates/cats/concurrency/examples/send_sync/send_sync_impl.rs:4](bk/crates/cats/concurrency/examples/send_sync/send_sync_impl.rs#L4)

#### `config`

- [bk/crates/cats/config/examples/env/envy.rs:4](bk/crates/cats/config/examples/env/envy.rs#L4)

#### `cryptography`

- [bk/crates/cats/cryptography/examples/password_hashing/scrypt.rs:4](bk/crates/cats/cryptography/examples/password_hashing/scrypt.rs#L4)
- [bk/crates/cats/cryptography/examples/secrets/redacted_secret.rs:4](bk/crates/cats/cryptography/examples/secrets/redacted_secret.rs#L4)

#### `data_structures`

- [bk/crates/cats/data_structures/examples/strings/string_format.rs:92](bk/crates/cats/data_structures/examples/strings/string_format.rs#L92)

#### `database`

- [bk/crates/cats/database/examples/connection_pool/deadpool.rs:4](bk/crates/cats/database/examples/connection_pool/deadpool.rs#L4)
- [bk/crates/cats/database/examples/postgres/cornucopia/generated.rs:6](bk/crates/cats/database/examples/postgres/cornucopia/generated.rs#L6)
- [bk/crates/cats/database/examples/postgres/cornucopia/generated.rs:11](bk/crates/cats/database/examples/postgres/cornucopia/generated.rs#L11)
- [bk/crates/cats/database/examples/sqlite/insert_select.rs:16](bk/crates/cats/database/examples/sqlite/insert_select.rs#L16)
- [bk/crates/cats/database/examples/sqlite/transactions.rs:101](bk/crates/cats/database/examples/sqlite/transactions.rs#L101)

#### `development_tools_debugging`

- [bk/crates/cats/development_tools_debugging/examples/log/log_custom.rs:4](bk/crates/cats/development_tools_debugging/examples/log/log_custom.rs#L4)
- [bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs:4](bk/crates/cats/development_tools_debugging/examples/tracing/tracing_subscriber_layers.rs#L4)

#### `development_tools_ffi`

- [later/crates/cats/development_tools_ffi/examples/node/napi.rs:8](later/crates/cats/development_tools_ffi/examples/node/napi.rs#L8)

#### `development_tools_profiling`

- [bk/crates/cats/development_tools_profiling/examples/puffin_server.rs:4](bk/crates/cats/development_tools_profiling/examples/puffin_server.rs#L4)

#### `development_tools_testing`

- [bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs:4](bk/crates/cats/development_tools_testing/examples/property_based_testing/fake.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/env_vars.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/env_vars.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/paused_time.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/paused_time.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/proptest_strategy.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/proptest_strategy.rs#L4)
- [bk/crates/cats/development_tools_testing/examples/test_techniques/rstest_cases.rs:4](bk/crates/cats/development_tools_testing/examples/test_techniques/rstest_cases.rs#L4)

#### `email`

- [bk/crates/cats/email/examples/imap/mailchecker.rs:4](bk/crates/cats/email/examples/imap/mailchecker.rs#L4)

#### `encoding`

- [bk/crates/cats/encoding/examples/binary_encoders/my_game_generated.rs:6](bk/crates/cats/encoding/examples/binary_encoders/my_game_generated.rs#L6)
- [bk/crates/cats/encoding/examples/serde/serde_ignored.rs:22](bk/crates/cats/encoding/examples/serde/serde_ignored.rs#L22)
- [bk/crates/cats/encoding/examples/typecasts/zerocopy.rs:192](bk/crates/cats/encoding/examples/typecasts/zerocopy.rs#L192)

#### `finance`

- [later/crates/cats/finance/examples/market_data/yahoo_history.rs:4](later/crates/cats/finance/examples/market_data/yahoo_history.rs#L4)
- [later/crates/cats/finance/examples/quant/rustquant.rs:10](later/crates/cats/finance/examples/quant/rustquant.rs#L10)
- [later/crates/cats/finance/examples/quant/rustquant.rs:34](later/crates/cats/finance/examples/quant/rustquant.rs#L34)

#### `gui`

- [later/crates/cats/gui/examples/immediate_mode_gui/egui_headless.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_headless.rs#L4)
- [later/crates/cats/gui/examples/immediate_mode_gui/egui_kittest.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_kittest.rs#L4)
- [later/crates/cats/gui/examples/immediate_mode_gui/egui_plot.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_plot.rs#L4)
- [later/crates/cats/gui/examples/immediate_mode_gui/egui_table.rs:4](later/crates/cats/gui/examples/immediate_mode_gui/egui_table.rs#L4)

#### `language`

- [bk/crates/language/examples/attributes/allow_dead_code.rs:4](bk/crates/language/examples/attributes/allow_dead_code.rs#L4)
- [bk/crates/language/examples/attributes/attributes_derive.rs:4](bk/crates/language/examples/attributes/attributes_derive.rs#L4)
- [bk/crates/language/examples/attributes/attributes_early_development.rs:4](bk/crates/language/examples/attributes/attributes_early_development.rs#L4)
- [bk/crates/language/examples/attributes/attributes_early_development.rs:29](bk/crates/language/examples/attributes/attributes_early_development.rs#L29)
- [bk/crates/language/examples/constants_and_statics/constants.rs:4](bk/crates/language/examples/constants_and_statics/constants.rs#L4)
- [bk/crates/language/examples/constants_and_statics/statics.rs:4](bk/crates/language/examples/constants_and_statics/statics.rs#L4)
- [bk/crates/language/examples/data_types/unit_never.rs:4](bk/crates/language/examples/data_types/unit_never.rs#L4)
- [bk/crates/language/examples/functions/diverging_functions.rs:14](bk/crates/language/examples/functions/diverging_functions.rs#L14)
- [bk/crates/language/examples/functions/diverging_functions.rs:22](bk/crates/language/examples/functions/diverging_functions.rs#L22)
- [bk/crates/language/examples/functions/generic_functions2.rs:4](bk/crates/language/examples/functions/generic_functions2.rs#L4)
- [bk/crates/language/examples/functions/generic_functions2.rs:55](bk/crates/language/examples/functions/generic_functions2.rs#L55)
- [bk/crates/language/examples/generics/generic_type_parameter.rs:4](bk/crates/language/examples/generics/generic_type_parameter.rs#L4)
- [bk/crates/language/examples/lifetimes/lifetime_parameters.rs:4](bk/crates/language/examples/lifetimes/lifetime_parameters.rs#L4)
- [bk/crates/language/examples/lifetimes/self_referential_struct.rs:6](bk/crates/language/examples/lifetimes/self_referential_struct.rs#L6)
- [bk/crates/language/examples/macros/macros.rs:69](bk/crates/language/examples/macros/macros.rs#L69)
- [bk/crates/language/examples/match/destructure_enum.rs:4](bk/crates/language/examples/match/destructure_enum.rs#L4)
- [bk/crates/language/examples/match/if_let.rs:4](bk/crates/language/examples/match/if_let.rs#L4)
- [bk/crates/language/examples/match/match1.rs:4](bk/crates/language/examples/match/match1.rs#L4)
- [bk/crates/language/examples/ownership_borrowing/borrowing_function.rs:1](bk/crates/language/examples/ownership_borrowing/borrowing_function.rs#L1)
- [bk/crates/language/examples/ownership_borrowing/copy.rs:4](bk/crates/language/examples/ownership_borrowing/copy.rs#L4)
- [bk/crates/language/examples/structs/generic_structs.rs:4](bk/crates/language/examples/structs/generic_structs.rs#L4)
- [bk/crates/language/examples/structs/struct_fields.rs:4](bk/crates/language/examples/structs/struct_fields.rs#L4)
- [bk/crates/language/examples/structs/struct_init.rs:4](bk/crates/language/examples/structs/struct_init.rs#L4)
- [bk/crates/language/examples/structs/struct_update.rs:4](bk/crates/language/examples/structs/struct_update.rs#L4)
- [bk/crates/language/examples/structs/structs.rs:4](bk/crates/language/examples/structs/structs.rs#L4)
- [bk/crates/language/examples/structs/tuple_structs.rs:4](bk/crates/language/examples/structs/tuple_structs.rs#L4)
- [bk/crates/language/examples/trait_objects/dyn_compat.rs:4](bk/crates/language/examples/trait_objects/dyn_compat.rs#L4)
- [bk/crates/language/examples/traits/trait_types.rs:4](bk/crates/language/examples/traits/trait_types.rs#L4)
- [bk/crates/language/examples/traits/traits.rs:4](bk/crates/language/examples/traits/traits.rs#L4)

#### `mathematics`

- [bk/crates/cats/mathematics/examples/additional_numeric_types/typenum.rs:4](bk/crates/cats/mathematics/examples/additional_numeric_types/typenum.rs#L4)

#### `multimedia_audio`

- [later/crates/cats/multimedia_audio/examples/audio/rodio_file.rs:43](later/crates/cats/multimedia_audio/examples/audio/rodio_file.rs#L43)
- [later/crates/cats/multimedia_audio/examples/audio/rodio_source.rs:76](later/crates/cats/multimedia_audio/examples/audio/rodio_source.rs#L76)

#### `os`

- [bk/crates/cats/os/examples/process_management/kill_guard.rs:4](bk/crates/cats/os/examples/process_management/kill_guard.rs#L4)
- [bk/crates/cats/os/examples/signals/tokio_signal.rs:4](bk/crates/cats/os/examples/signals/tokio_signal.rs#L4)

#### `other`

- [later/crates/other/examples/architecture/cqrs.rs:4](later/crates/other/examples/architecture/cqrs.rs#L4)
- [later/crates/other/examples/architecture/layered_architecture.rs:4](later/crates/other/examples/architecture/layered_architecture.rs#L4)
- [later/crates/other/examples/cloud/aws_cloudwatch.rs:4](later/crates/other/examples/cloud/aws_cloudwatch.rs#L4)
- [later/crates/other/examples/cloud/aws_lambda_invoke.rs:4](later/crates/other/examples/cloud/aws_lambda_invoke.rs#L4)
- [later/crates/other/examples/cloud/aws_message_bus.rs:4](later/crates/other/examples/cloud/aws_message_bus.rs#L4)
- [later/crates/other/examples/cloud/aws_s3_compatible.rs:4](later/crates/other/examples/cloud/aws_s3_compatible.rs#L4)
- [later/crates/other/examples/cloud/aws_ses.rs:4](later/crates/other/examples/cloud/aws_ses.rs#L4)
- [later/crates/other/examples/cloud/kubernetes.rs:4](later/crates/other/examples/cloud/kubernetes.rs#L4)
- [later/crates/other/examples/cloud/shuttle.rs:44](later/crates/other/examples/cloud/shuttle.rs#L44)

#### `parser_implementations`

- [bk/crates/cats/parser_implementations/examples/toml/toml.rs:4](bk/crates/cats/parser_implementations/examples/toml/toml.rs#L4)
- [bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs:4](bk/crates/cats/parser_implementations/examples/xml/xml5ever.rs#L4)

#### `rust_patterns`

- [bk/crates/cats/rust_patterns/examples/builder_pattern/builder_pattern.rs:4](bk/crates/cats/rust_patterns/examples/builder_pattern/builder_pattern.rs#L4)
- [bk/crates/cats/rust_patterns/examples/error/thiserror.rs:4](bk/crates/cats/rust_patterns/examples/error/thiserror.rs#L4)
- [bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs:50](bk/crates/cats/rust_patterns/examples/rust_specific_patterns/pin_project.rs#L50)

#### `science_geo`

- [later/crates/cats/science_geo/examples/geo/geocode.rs:4](later/crates/cats/science_geo/examples/geo/geocode.rs#L4)

#### `standard_library`

- [bk/crates/standard_library/examples/box/box_basics.rs:4](bk/crates/standard_library/examples/box/box_basics.rs#L4)
- [bk/crates/standard_library/examples/conversion_traits/from3.rs:4](bk/crates/standard_library/examples/conversion_traits/from3.rs#L4)
- [bk/crates/standard_library/examples/default/default.rs:4](bk/crates/standard_library/examples/default/default.rs#L4)
- [bk/crates/standard_library/examples/derive/derive_more.rs:4](bk/crates/standard_library/examples/derive/derive_more.rs#L4)
- [bk/crates/standard_library/examples/result/result.rs:77](bk/crates/standard_library/examples/result/result.rs#L77)
- [bk/crates/standard_library/examples/result/result.rs:118](bk/crates/standard_library/examples/result/result.rs#L118)

#### `wasm`

- [later/crates/cats/wasm/examples/interfacing_with_javascript/promises.rs:2](later/crates/cats/wasm/examples/interfacing_with_javascript/promises.rs#L2)
- [later/crates/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs:2](later/crates/cats/wasm/examples/interfacing_with_javascript/serde_wasm_bindgen.rs#L2)
- [later/crates/cats/wasm/examples/leptos_client/gloo_fetch.rs:4](later/crates/cats/wasm/examples/leptos_client/gloo_fetch.rs#L4)
- [later/crates/cats/wasm/examples/leptos_client/gloo_storage.rs:4](later/crates/cats/wasm/examples/leptos_client/gloo_storage.rs#L4)
- [later/crates/cats/wasm/examples/leptos_client/grpc_web_client.rs:4](later/crates/cats/wasm/examples/leptos_client/grpc_web_client.rs#L4)

#### `web_programming_http_client`

- [bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs:4](bk/crates/cats/web_programming_http_client/examples/apis/paginated.rs#L4)

#### `web_programming_http_server`

- [bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/cookies.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/cookies.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/debug_handler.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/debug_handler.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/flash_messages.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/flash_messages.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/https_redirect.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/https_redirect.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/axum_extras/typed_header.rs:4](bk/crates/cats/web_programming_http_server/examples/axum_extras/typed_header.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/grpc/grpc_web.rs:4](bk/crates/cats/web_programming_http_server/examples/grpc/grpc_web.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs:4](bk/crates/cats/web_programming_http_server/examples/grpc/tonic_client.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs:4](bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs#L4)
- [bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs:16](bk/crates/cats/web_programming_http_server/examples/openapi/schemars.rs#L16)
- [bk/crates/cats/web_programming_http_server/examples/openapi/utoipa.rs:4](bk/crates/cats/web_programming_http_server/examples/openapi/utoipa.rs#L4)

### Ignored tests (75)

#### `aerospace_drones`

- [later/crates/cats/aerospace_drones/examples/drones/drones1.rs:33](later/crates/cats/aerospace_drones/examples/drones/drones1.rs#L33) - requires real drone

#### `aerospace_unmanned_aerial_vehicles`

- [later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs:52](later/crates/cats/aerospace_unmanned_aerial_vehicles/examples/uavs/uavs.rs#L52) - requires real drone

#### `command_line_interface`

- [bk/crates/cats/command_line_interface/examples/crossterm.rs:98](bk/crates/cats/command_line_interface/examples/crossterm.rs#L98) - requires an interactive terminal and key input
- [bk/crates/cats/command_line_interface/examples/user_interaction/inquire.rs:49](bk/crates/cats/command_line_interface/examples/user_interaction/inquire.rs#L49) - Requires user input

#### `cryptography`

- [bk/crates/cats/cryptography/examples/secrets/keyring.rs:80](bk/crates/cats/cryptography/examples/secrets/keyring.rs#L80) - needs an OS credential store
- [bk/crates/cats/cryptography/examples/tls/tokio_rustls.rs:64](bk/crates/cats/cryptography/examples/tls/tokio_rustls.rs#L64) - Requires network

#### `data_structures`

- [bk/crates/cats/data_structures/examples/maps/multimap.rs:61](bk/crates/cats/data_structures/examples/maps/multimap.rs#L61) - requires external resources

#### `database`

- [bk/crates/cats/database/examples/amqp/lapin.rs:144](bk/crates/cats/database/examples/amqp/lapin.rs#L144) - needs RabbitMQ
- [bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs:215](bk/crates/cats/database/examples/cassandra/cdrs_tokio.rs#L215) - needs Cassandra at 127.0.0.1:9042
- [bk/crates/cats/database/examples/connection_pool/deadpool2.rs:135](bk/crates/cats/database/examples/connection_pool/deadpool2.rs#L135) - requires a reachable PostgreSQL service and PG__* environment variables
- [bk/crates/cats/database/examples/mssql/tiberius.rs:87](bk/crates/cats/database/examples/mssql/tiberius.rs#L87) - Requires an external Microsoft SQL Server instance
- [bk/crates/cats/database/examples/nosql/mongodb.rs:80](bk/crates/cats/database/examples/nosql/mongodb.rs#L80) - needs MongoDB reachable through MONGO_URI
- [bk/crates/cats/database/examples/nosql/redis.rs:94](bk/crates/cats/database/examples/nosql/redis.rs#L94) - needs a reachable Redis server
- [bk/crates/cats/database/examples/oracle/diesel_oci.rs:108](bk/crates/cats/database/examples/oracle/diesel_oci.rs#L108) - requires external oracle db and oracle client library
- [bk/crates/cats/database/examples/oracle/oracle.rs:59](bk/crates/cats/database/examples/oracle/oracle.rs#L59) - requires Oracle Free, Instant Client, and ORACLE_* settings
- [bk/crates/cats/database/examples/oracle/sibyl.rs:67](bk/crates/cats/database/examples/oracle/sibyl.rs#L67) - requires Oracle Instant Client and a reachable Oracle service
- [bk/crates/cats/database/examples/postgres/aggregate_data.rs:85](bk/crates/cats/database/examples/postgres/aggregate_data.rs#L85) - needs Postgres
- [bk/crates/cats/database/examples/postgres/cornucopia.rs:88](bk/crates/cats/database/examples/postgres/cornucopia.rs#L88) - needs Postgres
- [bk/crates/cats/database/examples/postgres/create_tables.rs:69](bk/crates/cats/database/examples/postgres/create_tables.rs#L69) - needs Postgres
- [bk/crates/cats/database/examples/postgres/insert_query_data.rs:110](bk/crates/cats/database/examples/postgres/insert_query_data.rs#L110) - needs Postgres
- [bk/crates/cats/database/examples/search/elasticsearch.rs:317](bk/crates/cats/database/examples/search/elasticsearch.rs#L317) - Requires a disposable Elasticsearch service configured through ELASTIC_URL
- [bk/crates/cats/database/tests/postgres/main.rs:21](bk/crates/cats/database/tests/postgres/main.rs#L21) - needs Postgres

#### `development_tools`

- [bk/crates/cats/development_tools/examples/crate_registries/crates_io.rs:131](bk/crates/cats/development_tools/examples/crate_registries/crates_io.rs#L131) - requires crates.io API access

#### `development_tools_debugging`

- [bk/crates/cats/development_tools_debugging/examples/log/log_syslog.rs:38](bk/crates/cats/development_tools_debugging/examples/log/log_syslog.rs#L38) - requires a Unix syslog daemon

#### `development_tools_ffi`

- [later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs:188](later/crates/cats/development_tools_ffi/examples/python/use_rust_from_python.rs#L188) - needs maturin, Python, and an active Python virtual environment
- [later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs:51](later/crates/cats/development_tools_ffi/examples/ruby/magnus.rs#L51) - needs Ruby headers and libruby
- [later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs:58](later/crates/cats/development_tools_ffi/examples/ruby/rutie.rs#L58) - needs Ruby

#### `development_tools_testing`

- [bk/crates/cats/development_tools_testing/examples/test_runners/tests1.rs:89](bk/crates/cats/development_tools_testing/examples/test_runners/tests1.rs#L89) - This test takes an hour to run. Only run it manually when needed

#### `email`

- [bk/crates/cats/email/examples/lettre.rs:104](bk/crates/cats/email/examples/lettre.rs#L104) - needs SMTP credentials and a reachable SMTP relay

#### `filesystem`

- [bk/crates/cats/filesystem/examples/open.rs:38](bk/crates/cats/filesystem/examples/open.rs#L38) - Relies on default programs that may not be installed by default.

#### `finance`

- [later/crates/cats/finance/examples/market_data/yahoo_history.rs:96](later/crates/cats/finance/examples/market_data/yahoo_history.rs#L96) - needs network access

#### `game_engines`

- [later/crates/cats/game_engines/examples/game_engines/ggez.rs:138](later/crates/cats/game_engines/examples/game_engines/ggez.rs#L138) - opens a graphics window
- [later/crates/cats/game_engines/examples/game_engines/macroquad.rs:85](later/crates/cats/game_engines/examples/game_engines/macroquad.rs#L85) - opens a graphics window

#### `games`

- [later/crates/cats/games/examples/games.rs:69](later/crates/cats/games/examples/games.rs#L69) - requires interactive example runtime

#### `gui`

- [later/crates/cats/gui/examples/2d_renderers/femtovg.rs:201](later/crates/cats/gui/examples/2d_renderers/femtovg.rs#L201) - requires an interactive desktop display and OpenGL context
- [later/crates/cats/gui/examples/2d_renderers/minifb.rs:79](later/crates/cats/gui/examples/2d_renderers/minifb.rs#L79) - requires interactive example runtime
- [later/crates/cats/gui/examples/2d_renderers/vello.rs:175](later/crates/cats/gui/examples/2d_renderers/vello.rs#L175) - requires a compatible GPU adapter
- [later/crates/cats/gui/examples/2d_renderers/webrender.rs:136](later/crates/cats/gui/examples/2d_renderers/webrender.rs#L136) - requires interactive example runtime
- [later/crates/cats/gui/examples/clipboard/arboard.rs:53](later/crates/cats/gui/examples/clipboard/arboard.rs#L53) - requires a desktop clipboard and overwrites its current text
- [later/crates/cats/gui/examples/clipboard/arboard.rs:72](later/crates/cats/gui/examples/clipboard/arboard.rs#L72) - needs a desktop session with a system clipboard
- [later/crates/cats/gui/examples/file_dialogs/rfd.rs:58](later/crates/cats/gui/examples/file_dialogs/rfd.rs#L58) - opens a native file dialog
- [later/crates/cats/gui/examples/gtk/gtk4.rs:104](later/crates/cats/gui/examples/gtk/gtk4.rs#L104) - requires an interactive GTK session
- [later/crates/cats/gui/examples/gtk/relm4.rs:99](later/crates/cats/gui/examples/gtk/relm4.rs#L99) - requires an interactive GTK session
- [later/crates/cats/gui/examples/immediate_mode_gui/egui.rs:134](later/crates/cats/gui/examples/immediate_mode_gui/egui.rs#L134) - requires an interactive display
- [later/crates/cats/gui/examples/retained_mode_gui/floem.rs:61](later/crates/cats/gui/examples/retained_mode_gui/floem.rs#L61) - opens a window
- [later/crates/cats/gui/examples/retained_mode_gui/iced.rs:74](later/crates/cats/gui/examples/retained_mode_gui/iced.rs#L74) - opens a window
- [later/crates/cats/gui/examples/retained_mode_gui/xilem.rs:60](later/crates/cats/gui/examples/retained_mode_gui/xilem.rs#L60) - requires an interactive example runtime
- [later/crates/cats/gui/examples/web/wry_window.rs:186](later/crates/cats/gui/examples/web/wry_window.rs#L186) - requires a display
- [later/crates/cats/gui/examples/window_creation/baseview.rs:76](later/crates/cats/gui/examples/window_creation/baseview.rs#L76) - requires interactive example runtime
- [later/crates/cats/gui/examples/window_creation/tao.rs:76](later/crates/cats/gui/examples/window_creation/tao.rs#L76) - opens a window
- [later/crates/cats/gui/examples/window_creation/winit.rs:75](later/crates/cats/gui/examples/window_creation/winit.rs#L75) - opens a native window

#### `multimedia`

- [later/crates/cats/multimedia/examples/multimedia/multimedia1.rs:42](later/crates/cats/multimedia/examples/multimedia/multimedia1.rs#L42) - requires audio output device

#### `multimedia_audio`

- [later/crates/cats/multimedia_audio/examples/audio/audio.rs:86](later/crates/cats/multimedia_audio/examples/audio/audio.rs#L86) - requires audio device

#### `network_programming`

- [bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs:109](bk/crates/cats/network_programming/examples/reverse_proxy/pingora.rs#L109) - requires a local upstream on 127.0.0.1:8000 and a proxy listener

#### `os`

- [bk/crates/cats/os/examples/external_commands/continuous.rs:63](bk/crates/cats/os/examples/external_commands/continuous.rs#L63) - requires access to a Linux systemd journal

#### `os_windows_apis`

- [bk/crates/cats/os_windows_apis/examples/windows/winapi.rs:60](bk/crates/cats/os_windows_apis/examples/windows/winapi.rs#L60) - displays a Windows message box
- [bk/crates/cats/os_windows_apis/examples/windows/windows.rs:35](bk/crates/cats/os_windows_apis/examples/windows/windows.rs#L35) - displays a Windows message box

#### `other`

- [later/crates/other/examples/cloud/aws_lambda.rs:88](later/crates/other/examples/cloud/aws_lambda.rs#L88) - requires the AWS Lambda runtime environment (AWS_LAMBDA_RUNTIME_API)
- [later/crates/other/examples/cloud/aws_s3_compatible.rs:86](later/crates/other/examples/cloud/aws_s3_compatible.rs#L86) - requires an S3-compatible server on localhost:9000
- [later/crates/other/examples/cloud/aws_sdk.rs:43](later/crates/other/examples/cloud/aws_sdk.rs#L43) - needs AWS credentials and network access
- [later/crates/other/examples/cloud/kubernetes.rs:234](later/crates/other/examples/cloud/kubernetes.rs#L234) - needs a cluster (kind or k3d)
- [later/crates/other/examples/cloud/shuttle.rs:55](later/crates/other/examples/cloud/shuttle.rs#L55) - requires Shuttle runtime

#### `rendering_graphics_api`

- [later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs:208](later/crates/cats/rendering_graphics_api/examples/gpu_abstraction_layers/wgpu.rs#L208) - requires a compatible GPU adapter
- [later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/wgpu_adapter_info.rs:52](later/crates/cats/rendering_graphics_api/examples/native_graphics_apis/wgpu_adapter_info.rs#L52) - needs a GPU adapter

#### `science_robotics`

- [later/crates/cats/science_robotics/examples/robot_operating_systems/ros2_pubsub.rs:78](later/crates/cats/science_robotics/examples/robot_operating_systems/ros2_pubsub.rs#L78) - needs a ROS 2 environment
- [later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs:171](later/crates/cats/science_robotics/examples/robotics_frameworks/openrr.rs#L171) - requires interactive example runtime

#### `text_editors`

- [bk/crates/cats/text_editors/examples/language_server_protocol/lsp_client.rs:208](bk/crates/cats/text_editors/examples/language_server_protocol/lsp_client.rs#L208) - needs rust-analyzer on the PATH

#### `web_programming`

- [bk/crates/cats/web_programming/examples/mime/request.rs:98](bk/crates/cats/web_programming/examples/mime/request.rs#L98) - needs network
- [bk/crates/cats/web_programming/examples/scraping/broken.rs:172](bk/crates/cats/web_programming/examples/scraping/broken.rs#L172) - needs network
- [bk/crates/cats/web_programming/examples/scraping/extract_links.rs:76](bk/crates/cats/web_programming/examples/scraping/extract_links.rs#L76) - needs network
- [bk/crates/cats/web_programming/examples/scraping/unique.rs:83](bk/crates/cats/web_programming/examples/scraping/unique.rs#L83) - needs network

#### `web_programming_http_client`

- [bk/crates/cats/web_programming_http_client/examples/download/download.rs:74](bk/crates/cats/web_programming_http_client/examples/download/download.rs#L74) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/get.rs:33](bk/crates/cats/web_programming_http_client/examples/requests/get.rs#L33) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/get1.rs:36](bk/crates/cats/web_programming_http_client/examples/requests/get1.rs#L36) - needs network
- [bk/crates/cats/web_programming_http_client/examples/requests/header.rs:87](bk/crates/cats/web_programming_http_client/examples/requests/header.rs#L87) - needs network
