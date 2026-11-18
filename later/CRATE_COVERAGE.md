<!-- TODO Reviewed ??-??-2026. -->

# Crate Coverage Review

## Candidates for New Recipes

The candidates below fill specific gaps in the authored chapters and examples
under both `later/` and `bk/`. The review checked chapter prose, recipe lists,
example source, and direct dependencies. A registry ranking or a transitive
dependency can help find a crate, but a new recipe still needs an uncovered
reader task. Other categories have no candidate in this review.

| Crate | Proposed recipe | Scope |
| --- | --- | --- |
| [`gdbstub`](https://docs.rs/gdbstub/latest/gdbstub/) | Single step a toy emulator in GDB and inspect its simulated registers. | Extend the existing [emulator](src/categories/emulators/emulators.md) with a local debugger session. The existing recipe already explains instruction execution and ROM loading. |
| [`meshopt`](https://docs.rs/meshopt/latest/meshopt/optimize/index.html) | Reorder indices in a small triangle mesh for the vertex cache. | Verify that the triangle multiset stays the same, including winding. The existing [3D model recipe](../bk/src/categories/rendering_data-formats/3d_model_formats.md) covers loading and inspecting a scene. |
| [`turmoil`](https://docs.rs/turmoil/latest/turmoil/) | Partition and repair a link between two simulated hosts and verify that delivery resumes. | Exercise a two-host network lifecycle. The existing [simulation recipes](src/categories/simulation/simulation.md) cover event queues, ODEs, and sampling. |

## Registry Snapshot

Snapshot taken November 18, 2026. Each row shows three recent download
leaders and the all-time leader. The queries used
`/api/v1/crates?category=<slug>&sort=recent-downloads&per_page=5` and
`sort=downloads&per_page=1`. Counts include transitive use and depend on
[category tags](https://crates.io/category_slugs). Editorial topics without a
matching registry category use an adjacent category for context. Aerospace and
Cryptocurrencies remain deferred.

The snapshot is historical registry evidence. Candidate recipes are listed
above.

| Staged topic | Registry category | Recent leaders (downloads) | All-time leader (downloads) |
| --- | --- | --- | --- |
| Accessibility | [accessibility](https://crates.io/categories/accessibility) | [atspi](https://crates.io/crates/atspi) (8,652,697), [atspi-common](https://crates.io/crates/atspi-common) (8,631,189), [atspi-proxies](https://crates.io/crates/atspi-proxies) (8,610,380) | [atspi](https://crates.io/crates/atspi) (20,068,667) |
| API bindings | [api-bindings](https://crates.io/categories/api-bindings) | [socket2](https://crates.io/crates/socket2) (391,040,344), [flate2](https://crates.io/crates/flate2) (239,813,259), [libloading](https://crates.io/crates/libloading) (165,026,726) | [socket2](https://crates.io/crates/socket2) (1,513,309,992) |
| Audio | [multimedia::audio](https://crates.io/categories/multimedia%3A%3Aaudio) | [symphonia-core](https://crates.io/crates/symphonia-core) (11,944,164), [symphonia-metadata](https://crates.io/crates/symphonia-metadata) (11,887,955), [symphonia](https://crates.io/crates/symphonia) (11,660,201) | [alsa](https://crates.io/crates/alsa) (29,895,468) |
| Cloud | [web-programming](https://crates.io/categories/web-programming) | [http](https://crates.io/crates/http) (340,610,945), [http-body](https://crates.io/crates/http-body) (296,342,679), [hyper](https://crates.io/crates/hyper) (293,027,553) | [http](https://crates.io/crates/http) (1,275,998,014) |
| Compilers | [compilers](https://crates.io/categories/compilers) | [codespan-reporting](https://crates.io/crates/codespan-reporting) (40,448,563), [link-cplusplus](https://crates.io/crates/link-cplusplus) (14,126,291), [cxxbridge-flags](https://crates.io/crates/cxxbridge-flags) (13,194,216) | [codespan-reporting](https://crates.io/crates/codespan-reporting) (172,058,362) |
| Computer Vision | [computer-vision](https://crates.io/categories/computer-vision) | [opencv](https://crates.io/crates/opencv) (1,457,579), [re_sdk](https://crates.io/crates/re_sdk) (696,930), [rerun](https://crates.io/crates/rerun) (619,785) | [opencv](https://crates.io/crates/opencv) (5,282,250) |
| Cross-platform Development | [os](https://crates.io/categories/os) | [getrandom](https://crates.io/crates/getrandom) (837,408,391), [windows-sys](https://crates.io/crates/windows-sys) (526,806,632), [libc](https://crates.io/crates/libc) (499,169,836) | [getrandom](https://crates.io/crates/getrandom) (2,657,592,291) |
| Data Processing | [database](https://crates.io/categories/database) | [rusqlite](https://crates.io/crates/rusqlite) (63,915,620), [postgres-types](https://crates.io/crates/postgres-types) (29,781,559), [tokio-postgres](https://crates.io/crates/tokio-postgres) (28,103,012) | [rusqlite](https://crates.io/crates/rusqlite) (158,911,224) |
| Development Tools: FFI | [development-tools::ffi](https://crates.io/categories/development-tools%3A%3Affi) | [bindgen](https://crates.io/crates/bindgen) (107,593,826), [pyo3-build-config](https://crates.io/crates/pyo3-build-config) (77,994,178), [pyo3](https://crates.io/crates/pyo3) (75,960,286) | [bindgen](https://crates.io/crates/bindgen) (471,366,174) |
| Procedural Macro Helpers | [development-tools::procedural-macro-helpers](https://crates.io/categories/development-tools%3A%3Aprocedural-macro-helpers) | [syn](https://crates.io/crates/syn) (918,602,034), [quote](https://crates.io/crates/quote) (500,441,825), [proc-macro2](https://crates.io/crates/proc-macro2) (493,168,046) | [syn](https://crates.io/crates/syn) (3,100,312,688) |
| Embedded | [embedded](https://crates.io/categories/embedded) | [zerocopy](https://crates.io/crates/zerocopy) (342,949,909), [icu_provider](https://crates.io/crates/icu_provider) (281,950,181), [icu_collections](https://crates.io/crates/icu_collections) (280,836,188) | [zerocopy](https://crates.io/crates/zerocopy) (1,167,677,763) |
| Emulators | [emulators](https://crates.io/categories/emulators) | [bitfields](https://crates.io/crates/bitfields) (2,612,904), [bitfields-impl](https://crates.io/crates/bitfields-impl) (2,611,644), [gdbstub](https://crates.io/crates/gdbstub) (1,522,898) | [gdbstub](https://crates.io/crates/gdbstub) (6,546,287) |
| External FFI Bindings | [external-ffi-bindings](https://crates.io/categories/external-ffi-bindings) | [libc](https://crates.io/crates/libc) (499,169,836), [linux-raw-sys](https://crates.io/crates/linux-raw-sys) (355,845,905), [winapi-util](https://crates.io/crates/winapi-util) (134,618,701) | [libc](https://crates.io/crates/libc) (2,008,476,737) |
| Finance | [finance](https://crates.io/categories/finance) | [databento](https://crates.io/crates/databento) (514,288), [quickfix](https://crates.io/crates/quickfix) (236,545), [polymarket_client_sdk_v2](https://crates.io/crates/polymarket_client_sdk_v2) (175,930) | [databento](https://crates.io/crates/databento) (1,258,748) |
| Games | [games](https://crates.io/categories/games) | [presser](https://crates.io/crates/presser) (14,740,050), [tobj](https://crates.io/crates/tobj) (825,492), [map_3d](https://crates.io/crates/map_3d) (280,961) | [presser](https://crates.io/crates/presser) (28,665,059) |
| Game Development | [game-development](https://crates.io/categories/game-development) | [ash](https://crates.io/crates/ash) (20,170,263), [softbuffer](https://crates.io/crates/softbuffer) (14,833,459), [egui](https://crates.io/crates/egui) (10,220,663) | [ash](https://crates.io/crates/ash) (53,249,722) |
| Game Engines | [game-engines](https://crates.io/categories/game-engines) | [glam](https://crates.io/crates/glam) (67,836,995), [include_dir](https://crates.io/crates/include_dir) (22,733,551), [bevy_ecs](https://crates.io/crates/bevy_ecs) (2,793,613) | [glam](https://crates.io/crates/glam) (193,269,091) |
| GPU Programming | [rendering::graphics-api](https://crates.io/categories/rendering%3A%3Agraphics-api) | [ash](https://crates.io/crates/ash) (20,170,263), [gl_generator](https://crates.io/crates/gl_generator) (17,679,623), [khronos_api](https://crates.io/crates/khronos_api) (17,667,195) | [ash](https://crates.io/crates/ash) (53,249,722) |
| GUI | [gui](https://crates.io/categories/gui) | [dpi](https://crates.io/crates/dpi) (34,541,365), [wayland-protocols](https://crates.io/crates/wayland-protocols) (31,909,403), [wayland-client](https://crates.io/crates/wayland-client) (31,322,673) | [wayland-protocols](https://crates.io/crates/wayland-protocols) (97,617,873) |
| Machine Learning in Practice | [science](https://crates.io/categories/science) | [num-traits](https://crates.io/crates/num-traits) (318,930,542), [num-integer](https://crates.io/crates/num-integer) (225,256,268), [num-bigint](https://crates.io/crates/num-bigint) (223,287,064) | [num-traits](https://crates.io/crates/num-traits) (1,257,607,555) |
| Classical Machine Learning | [science](https://crates.io/categories/science) | [num-traits](https://crates.io/crates/num-traits) (318,930,542), [num-integer](https://crates.io/crates/num-integer) (225,256,268), [num-bigint](https://crates.io/crates/num-bigint) (223,287,064) | [num-traits](https://crates.io/crates/num-traits) (1,257,607,555) |
| Multimedia | [multimedia](https://crates.io/categories/multimedia) | [png](https://crates.io/crates/png) (120,408,720), [image](https://crates.io/crates/image) (84,737,765), [zune-core](https://crates.io/crates/zune-core) (72,726,603) | [png](https://crates.io/crates/png) (320,537,168) |
| Multimedia Encoding | [multimedia::encoding](https://crates.io/categories/multimedia%3A%3Aencoding) | [image](https://crates.io/crates/image) (84,737,765), [zune-core](https://crates.io/crates/zune-core) (72,726,603), [tiff](https://crates.io/crates/tiff) (52,732,950) | [image](https://crates.io/crates/image) (256,524,421) |
| Native OS APIs | [os](https://crates.io/categories/os) | [getrandom](https://crates.io/crates/getrandom) (837,408,391), [windows-sys](https://crates.io/crates/windows-sys) (526,806,632), [libc](https://crates.io/crates/libc) (499,169,836) | [getrandom](https://crates.io/crates/getrandom) (2,657,592,291) |
| No std | [no-std](https://crates.io/categories/no-std) | [hashbrown](https://crates.io/crates/hashbrown) (927,892,343), [getrandom](https://crates.io/crates/getrandom) (837,408,391), [rand](https://crates.io/crates/rand) (655,144,650) | [hashbrown](https://crates.io/crates/hashbrown) (3,150,644,258) |
| FreeBSD APIs | [os::freebsd-apis](https://crates.io/categories/os%3A%3Afreebsd-apis) | [evdevil](https://crates.io/crates/evdevil) (3,161), [mdconfig](https://crates.io/crates/mdconfig) (1,457), [rawether](https://crates.io/crates/rawether) (1,444) | [capsicum](https://crates.io/crates/capsicum) (21,858) |
| Linux APIs | [os::linux-apis](https://crates.io/categories/os%3A%3Alinux-apis) | [android-properties](https://crates.io/crates/android-properties) (8,544,908), [ashpd](https://crates.io/crates/ashpd) (8,419,861), [landlock](https://crates.io/crates/landlock) (7,591,016) | [android-properties](https://crates.io/crates/android-properties) (26,702,145) |
| macOS APIs | [os::macos-apis](https://crates.io/categories/os%3A%3Amacos-apis) | [core-foundation](https://crates.io/crates/core-foundation) (161,705,812), [security-framework](https://crates.io/crates/security-framework) (118,840,845), [security-framework-sys](https://crates.io/crates/security-framework-sys) (98,781,262) | [core-foundation](https://crates.io/crates/core-foundation) (566,708,584) |
| Rendering | [rendering](https://crates.io/categories/rendering) | [tiny-skia](https://crates.io/crates/tiny-skia) (29,960,271), [unic-ucd-version](https://crates.io/crates/unic-ucd-version) (20,430,344), [ash](https://crates.io/crates/ash) (20,170,263) | [unic-ucd-version](https://crates.io/crates/unic-ucd-version) (83,941,882) |
| Rendering Data Formats | [rendering::data-formats](https://crates.io/categories/rendering%3A%3Adata-formats) | [hexasphere](https://crates.io/crates/hexasphere) (2,658,765), [svg](https://crates.io/crates/svg) (2,005,850), [geozero](https://crates.io/crates/geozero) (923,783) | [hexasphere](https://crates.io/crates/hexasphere) (11,708,442) |
| Rendering Engines | [rendering::engine](https://crates.io/categories/rendering%3A%3Aengine) | [slint](https://crates.io/crates/slint) (1,127,305), [slint-build](https://crates.io/crates/slint-build) (1,083,894), [meshopt](https://crates.io/crates/meshopt) (418,843) | [slint](https://crates.io/crates/slint) (2,632,992) |
| Rendering Graphics APIs | [rendering::graphics-api](https://crates.io/categories/rendering%3A%3Agraphics-api) | [ash](https://crates.io/crates/ash) (20,170,263), [gl_generator](https://crates.io/crates/gl_generator) (17,679,623), [khronos_api](https://crates.io/crates/khronos_api) (17,667,195) | [ash](https://crates.io/crates/ash) (53,249,722) |
| Robotics | [science::robotics](https://crates.io/categories/science%3A%3Arobotics) | [serial2](https://crates.io/crates/serial2) (17,524,272), [mcap](https://crates.io/crates/mcap) (2,797,743), [levenberg-marquardt](https://crates.io/crates/levenberg-marquardt) (430,449) | [serial2](https://crates.io/crates/serial2) (26,174,150) |
| Science | [science](https://crates.io/categories/science) | [num-traits](https://crates.io/crates/num-traits) (318,930,542), [num-integer](https://crates.io/crates/num-integer) (225,256,268), [num-bigint](https://crates.io/crates/num-bigint) (223,287,064) | [num-traits](https://crates.io/crates/num-traits) (1,257,607,555) |
| Science Geo | [science::geo](https://crates.io/categories/science%3A%3Ageo) | [rstar](https://crates.io/crates/rstar) (17,608,301), [robust](https://crates.io/crates/robust) (7,901,457), [geo](https://crates.io/crates/geo) (5,562,713) | [rstar](https://crates.io/crates/rstar) (57,177,755) |
| Science Neuroscience | [science::neuroscience](https://crates.io/categories/science%3A%3Aneuroscience) | [corpus-ipc](https://crates.io/crates/corpus-ipc) (2,467), [ritk-spatial](https://crates.io/crates/ritk-spatial) (771), [ritk-wgpu-compat](https://crates.io/crates/ritk-wgpu-compat) (670) | [spiking_neural_networks](https://crates.io/crates/spiking_neural_networks) (29,521) |
| Scheduling | [date-and-time](https://crates.io/categories/date-and-time) | [rustix](https://crates.io/crates/rustix) (376,774,084), [time](https://crates.io/crates/time) (253,415,038), [chrono](https://crates.io/crates/chrono) (247,220,846) | [rustix](https://crates.io/crates/rustix) (1,427,776,407) |
| Scripting | [parser-implementations](https://crates.io/categories/parser-implementations) | [syn](https://crates.io/crates/syn) (918,602,034), [serde_json](https://crates.io/crates/serde_json) (444,871,656), [shlex](https://crates.io/crates/shlex) (371,454,568) | [syn](https://crates.io/crates/syn) (3,100,312,688) |
| Simulation | [simulation](https://crates.io/categories/simulation) | [enigo](https://crates.io/crates/enigo) (1,778,241), [madsim-tokio](https://crates.io/crates/madsim-tokio) (1,720,431), [madsim](https://crates.io/crates/madsim) (1,603,687) | [turmoil](https://crates.io/crates/turmoil) (16,611,026) |
| Value Formatting | [value-formatting](https://crates.io/categories/value-formatting) | [itoa](https://crates.io/crates/itoa) (439,248,983), [zmij](https://crates.io/crates/zmij) (366,896,646), [ryu](https://crates.io/crates/ryu) (303,938,988) | [itoa](https://crates.io/crates/itoa) (1,674,099,672) |
| Video | [multimedia::video](https://crates.io/categories/multimedia%3A%3Avideo) | [v_frame](https://crates.io/crates/v_frame) (21,695,423), [lab](https://crates.io/crates/lab) (16,167,495), [symphonia](https://crates.io/crates/symphonia) (11,660,201) | [v_frame](https://crates.io/crates/v_frame) (62,463,461) |
| Virtualization | [virtualization](https://crates.io/categories/virtualization) | [landlock](https://crates.io/crates/landlock) (7,591,016), [wasmi_core](https://crates.io/crates/wasmi_core) (7,415,464), [wasmi](https://crates.io/crates/wasmi) (6,785,500) | [wasmi](https://crates.io/crates/wasmi) (28,391,593) |
| Visualization | [visualization](https://crates.io/categories/visualization) | [criterion-plot](https://crates.io/crates/criterion-plot) (85,252,110), [plotters](https://crates.io/crates/plotters) (69,028,029), [termtree](https://crates.io/crates/termtree) (46,239,431) | [criterion-plot](https://crates.io/crates/criterion-plot) (330,523,504) |
| WebAssembly | [wasm](https://crates.io/categories/wasm) | [reqwest](https://crates.io/crates/reqwest) (272,665,231), [uuid](https://crates.io/crates/uuid) (266,527,102), [wasi](https://crates.io/crates/wasi) (199,671,113) | [uuid](https://crates.io/crates/uuid) (1,007,278,690) |
| WebSockets | [web-programming::websocket](https://crates.io/categories/web-programming%3A%3Awebsocket) | [tungstenite](https://crates.io/crates/tungstenite) (132,582,443), [tokio-tungstenite](https://crates.io/crates/tokio-tungstenite) (119,601,367), [actix-http](https://crates.io/crates/actix-http) (13,720,089) | [tungstenite](https://crates.io/crates/tungstenite) (409,993,484) |
| Written in Rust | [development-tools](https://crates.io/categories/development-tools) | [syn](https://crates.io/crates/syn) (918,602,034), [quote](https://crates.io/crates/quote) (500,441,825), [proc-macro2](https://crates.io/crates/proc-macro2) (493,168,046) | [syn](https://crates.io/crates/syn) (3,100,312,688) |
| Command-line Utilities (published comparison) | [command-line-utilities](https://crates.io/categories/command-line-utilities) | [crossterm](https://crates.io/crates/crossterm) (60,213,403), [is-wsl](https://crates.io/crates/is-wsl) (19,878,180), [is-docker](https://crates.io/crates/is-docker) (19,877,555) | [crossterm](https://crates.io/crates/crossterm) (236,757,069) |

## Examples Already in the Staged Book

These recipes were added or updated during the coverage review.

- [Source diagnostics](src/categories/compilers/source_diagnostics.md) combines
  Logos byte spans with codespan-reporting labels.
- [Terminal tables](../bk/src/categories/visualization/terminal_tables.md) uses tabled
  when readers need exact values and units.
- [Route measurements](src/categories/science/route_measurements.md) uses geo
  to compare a measured route with a simple model.
- [Collect Three Markers](src/categories/games/index.md#collect-markers) uses
  Macroquad to introduce an input, update, and draw loop.
- [Media fixture](../bk/src/categories/multimedia/multimedia.md#media-round-trip)
  connects the existing multimedia crates and format chapters to an executed
  FFmpeg workflow.
