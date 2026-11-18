<!-- TODO Reviewed ??-??-2026. -->

# Staged Platform and ML Migration Readiness

Reviewed November 17, 2026, for [#2350](https://github.com/john-cd/rust_howto/issues/2350), [#2351](https://github.com/john-cd/rust_howto/issues/2351), and [#2352](https://github.com/john-cd/rust_howto/issues/2352). This review covers the requested GUI, embedded, robotics, audio, graphics, native-API, Games, Rendering Engines, and ML material. It does not certify every category under `later`.

“Ready” means the named introductory recipe has sufficient prose and executable evidence to propose a focused migration. It still needs the normal `bk` manifest/feature, navigation, reference, and integration checks when moved. A successful mock, compiler fallback, or headless test does not establish real device or visual behavior. No chapters were moved into `bk` by this review.

## Ready for Focused Migration

| Category / scope | Eligible content | Evidence and limits |
| --- | --- | --- |
| [Machine Learning in Practice](src/other/machine-learning/index.md) | Held-out classification, temporal forecasting, and ONNX inference/model replacement. | 9 tests passed with the native ONNX Runtime on Windows; strict Clippy and Rust 1.98.1 compile check passed. Synthetic fixtures demonstrate software behavior, not production accuracy. |
| [Science: Classical ML](src/categories/science/classical_machine_learning.md) | SmartCore classification and Linfa clustering/regression. | 9 tests passed. Keep this as classical ML coverage rather than promoting untested deep-learning/CUDA paths. |
| [Rendering Data Formats](src/categories/rendering_data-formats/index.md) | Inline glTF scene parsing, tested texture layout/conversion, and SVG rasterization recipes. | Included in the successful host sweep; these device-free examples do not claim every listed compressed texture format or external importer is exercised. |
| [Games](src/categories/games/index.md) | The Macroquad marker-collection game from [#2379](https://github.com/john-cd/rust_howto/issues/2379). | Two game-state tests passed. The Windows smoke run presented three frames, scored a marker with scripted movement, and exited. A Linux X11 run exercised the normal keyboard path and checked marker pixels before scoring, after scoring, and after restart. Physical keyboard interaction and human visual review remain unverified. |
| [Linux APIs](src/categories/os_linux-apis/index.md) | Descriptor-based metadata with rustix and statx. | Five Linux tests and a native CLI run passed on the Docker Desktop WSL2 kernel. See the Linux commands and filesystem observations below. |
| [macOS APIs](src/categories/os_macos-apis/macos.md) | The introductory native API example. | The actual macOS API and Objective-C CI test steps passed on macOS 15/ARM64 with Rust 1.100.0. The relevant package sources, workspace manifest, lockfile, and native workflow match the tested revision. |
| [FreeBSD APIs](src/categories/os_freebsd-apis/freebsd.md) | The native sysctl introduction. | The actual native test step passed inside FreeBSD 14.4/x86-64 with Rust 1.100.0; the relevant source and lockfile match the tested revision. |

The native CI evidence is [run 44236893654](https://github.com/john-cd/rust_howto/actions/runs/44236893654), revision `769cf9cbf2cacc22cc2ff4383b6fddf397dbbcc4`. Both job and individual native lint/test step conclusions were inspected; the workflow is advisory, so its overall green status alone would not be enough. [#2214](https://github.com/john-cd/rust_howto/issues/2214) retains the separate CI-ownership decisions.

### ML Group Recheck

The Windows review for [#2374](https://github.com/john-cd/rust_howto/issues/2374)
checked the rendered Practical ML, Classical ML, and Deep Learning chapters
together. The two commands below passed 25 tests across preprocessing, text
features, classical algorithms, Candle CPU training and checkpoint reload, and
the practical classification, forecasting, and ONNX model replacement recipes.
Accelerator backends and catalog entries remain outside this result.

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p science --features candle --example ml --example feature_pipeline --example text_features
just cargo test --manifest-path later/crates/Cargo.toml --locked -p other --features onnx --example machine_learning
```

## Promote Narrow Recipes, Retain the Broader Category

| Category | Tested, promotable subset | Remaining gap |
| --- | --- | --- |
| [GUI](src/categories/gui/index.md) | egui interaction/layout/text logic; Windows Winit creation, minifb pixels, and Softbuffer presentation. | 36 default/minifb/Softbuffer tests passed with 7 interactive tests ignored; Slint added 5 headless tests with 1 interactive test ignored. Native self-closing presentation checks passed. No screenshot, full input/resize matrix, screen reader, or cross-platform toolkit certification: [#2359](https://github.com/john-cd/rust_howto/issues/2359), [#2213](https://github.com/john-cd/rust_howto/issues/2213). |
| [Embedded](src/categories/embedded/index.md) | Explicitly host-side HAL mocks, bounded messages, protocol/storage logic, and the Embassy host-task example. | 18 host tests passed after fixing deadline-task reuse. No board, firmware target, probe, pin map, flash operation, or physical reading was validated: [#2355](https://github.com/john-cd/rust_howto/issues/2355). |
| [Robotics](src/categories/science_robotics/index.md) | PID/control, pathfinding, kinematics, nearest-neighbor, mock-I2C, and physics simulation recipes. | Default host tests passed. ROS 2 feature compilation fails because `ROS_DISTRO` is not set; no physical robot/device or middleware round trip is established: [#2355](https://github.com/john-cd/rust_howto/issues/2355). |
| [Audio](src/categories/multimedia_audio/index.md) | WAV, synthesis, resampling, parsing, and the bundled synthetic HRIR/HRTF convolution recipe. | Host tests and a one-second Windows CPAL output-stream test passed. MIDI device communication and audible quality remain distinct from pure decoding and successful streaming: [#2356](https://github.com/john-cd/rust_howto/issues/2356). |
| [Rendering](src/categories/rendering/index.md) | The distinct fontdue glyph recipe and remaining engine catalogs. Raster/vector, tessellation, SVG, and projection recipes are already in published Graphics; staged duplicates were removed under #2361. | The earlier host tests passed before duplicate retirement. Do not promote catalog descriptions as proof that the corresponding windowed engines were run: [#2357](https://github.com/john-cd/rust_howto/issues/2357). |
| [Graphics APIs](src/categories/rendering_graphics-api/index.md) | Adapter discovery, wgpu compute/readback, shader validation/translation, and the new offscreen triangle render. | 16 tests passed with ignored GPU tests enabled. The triangle's center/background pixels passed on both Vulkan and DirectX 12 using Intel Iris Xe. OpenGL helpers without a current context and raw Vulkan discovery do not prove a complete scene: [#2357](https://github.com/john-cd/rust_howto/issues/2357). |
| [Rendering Engines](src/categories/rendering_engine/index.md) | The expanded engine-responsibility guide and bounded offscreen rendering core. | The new example has GPU pixel checks, strict Clippy, and a Rust 1.98.1 compile check. It uses wgpu directly; a full engine-owned scene/material/window lifecycle remains [#2357](https://github.com/john-cd/rust_howto/issues/2357). |
| [Game Engines](src/categories/game-engines/index.md) | The tested Macroquad and ggez game loops on Windows. | Unit tests passed, and native loops presented three and two frames respectively before exiting. Bevy/Fyrox, input/visual inspection, and other targets were not validated: [#2357](https://github.com/john-cd/rust_howto/issues/2357), [#2359](https://github.com/john-cd/rust_howto/issues/2359). |

### Macroquad Desktop Keyboard Check

At revision `f636015f6`, both locked game tests passed on Debian Trixie with
Rust 1.100.0. With Xvfb and Mesa software rendering, the normal game executable
accepted X11 keyboard events sent by `xdotool`: holding Right collected the
first marker, R restarted the round, and Escape exited with status zero.
The gold marker's horizontal pixel centroid in three X11 window captures
moved from 267 to 399 and returned to 267 on restart. This exercised the
keyboard and rendering paths with the scripted game smoke mode disabled.

### Linux ALSA Output

For [#2356](https://github.com/john-cd/rust_howto/issues/2356), the CPAL
output-stream test passed on Debian Trixie with Rust 1.100.0. In a disposable
container with `pkg-config` and `libasound2-dev`, a private ALSA configuration
routed the default PCM through the file plugin to a null sink. The capture
contained changing samples on both channels (signed 16-bit little-endian at
48 kHz). A null sink can drain faster than real time, so the byte count does
not establish playback duration or audible quality.

With that output configured, run from the repository root:

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p multimedia_audio --example audio -- audio::tests::test_audio_example_entry_point --ignored --nocapture
```

Windows listed Microsoft GS Wavetable Synth as an output and reported no
MIDI input ports. The Linux VM exposed no ALSA sequencer device. The MIDI
loopback example still needs a dedicated port pair for an end-to-end run.

### Linux statx

The native Linux checks for [#2358](https://github.com/john-cd/rust_howto/issues/2358)
used the source and lockfile at revision `a7ffd0590`, Rust 1.100.0, and a Debian
Trixie container on Docker Desktop's Linux 6.18.58.1 WSL2 kernel. Run from the
repository root on Linux with the repository's build paths configured:

```sh
just cargo clippy --manifest-path later/crates/Cargo.toml --locked -p os_linux_apis --examples --tests -- -D warnings
just cargo test --manifest-path later/crates/Cargo.toml --locked -p os_linux_apis --examples -- --nocapture
fixture_dir=$(mktemp -d)
TMPDIR="$fixture_dir" just cargo run --manifest-path later/crates/Cargo.toml --locked -p os_linux_apis --example linux
rmdir "$fixture_dir"
```

Clippy passed and all five example tests passed, including the three native
statx tests. On overlayfs the CLI printed 23 bytes and a birth timestamp,
confirming that the returned mask included both SIZE and BTIME. The suite
checked the SIZE bit against a 13-byte fixture and verified ENOENT for an
absent path. Removing the dedicated temporary directory succeeded without
recursive deletion. The default container seccomp filter was active and
allowed statx. Other kernels, filesystems, or sandbox profiles can omit
fields or reject the call.

## Not Ready or Already Published

- GTK/Relm4: `--features gtk --example gtk` compiles on Windows only because its actual bodies and dependencies are excluded there. This is not GTK validation. The remaining native SDK/toolkit work is [#2359](https://github.com/john-cd/rust_howto/issues/2359).
- [No Allocation](https://john-cd.com/rust_howto/categories/no-std/no_alloc.html): the recipes and package are already published in `bk`. The published example passed its test and `--no-default-features` check. The duplicate future preview was subsequently removed in [#2361](https://github.com/john-cd/rust_howto/issues/2361); links now target the published chapter.
- Physical embedded/robotics, drones/UAVs, MIDI-device I/O, screen-reader behavior, and untested GUI/engine backends remain excluded from broad readiness claims. Their existing or new issues name the required host/device evidence.

## Reproduce the Checks

Run from the repository root with the configured shared package/cache paths. Build output stays in the checkout's external slot. The Windows host used Rust 1.100.0, with Rust 1.98.1 for the two explicit baseline compile checks. Both NVIDIA RTX 3060 Laptop and Intel Iris Xe adapters are installed; the observed GPU render runs selected Intel Iris Xe. No NVIDIA runtime result is claimed.

```sh
just cargo test --manifest-path later/crates/Cargo.toml --locked -p rendering -p science_robotics -p multimedia_audio -p rendering_data_formats -p game_development --examples
just cargo test --manifest-path later/crates/cats/embedded/Cargo.toml --locked --examples
just cargo test --manifest-path later/crates/Cargo.toml --locked -p other --features onnx --example machine_learning
just cargo test --manifest-path later/crates/Cargo.toml --locked -p science --example ml
just cargo test --manifest-path later/crates/Cargo.toml --locked -p rendering_graphics_api --features wgpu --examples -- --include-ignored --nocapture
just cargo test --manifest-path later/crates/Cargo.toml --locked -p gui --features minifb,softbuffer --examples
just cargo test --manifest-path later/crates/Cargo.toml --locked -p gui --features slint --example retained_mode_gui
just cargo test --manifest-path later/crates/Cargo.toml --locked -p multimedia_audio --example audio -- --ignored --nocapture
just cargo +1.98.1 check --manifest-path later/crates/Cargo.toml --locked -p other --features onnx --example machine_learning
just cargo +1.98.1 check --manifest-path later/crates/Cargo.toml --locked -p rendering_graphics_api --features wgpu --example offscreen_render
```

The first command passed 65 tests across 18 targets, with one audio-device test initially ignored; that device test was then run explicitly and passed. The corrected Embassy run passed all 18 tests. The ML forecast's held-out MAE was approximately `0.0075`, compared with `1.05` for repeating the last training week, on the committed synthetic series. No real-data forecasting quality claim follows from that result.

For self-closing Windows presentation checks, set the process environment variable and remove it afterward:

```powershell
$env:RUST_HOWTO_SMOKE_TEST = '1'
just cargo run --manifest-path later/crates/Cargo.toml --locked -p gui --features minifb --example minifb_renderer
just cargo run --manifest-path later/crates/Cargo.toml --locked -p gui --features softbuffer --example window_creation -- software-pixels
just cargo run --manifest-path later/crates/Cargo.toml --locked -p gui --example window_creation -- winit
just cargo run --manifest-path later/crates/Cargo.toml --locked -p game_engines --features ggez --example game_engines -- ggez
just cargo run --manifest-path later/crates/Cargo.toml --locked -p game_engines --example game_engines -- macroquad
just cargo run --manifest-path later/crates/Cargo.toml --locked -p games --example games
Remove-Item Env:RUST_HOWTO_SMOKE_TEST
```

For the DirectX 12 pixel test, set `WGPU_BACKEND=dx12` for the `offscreen_render` example; unset it to use the default adapter/backend selection. Native presentation checks establish successful API calls and bounded shutdown. They are not screenshots or human visual-quality checks. The desktop inspection helper failed to initialize with “failed to write kernel assets”; no workaround using unobserved UI actions was attempted.

## Changes and Evidence

The full future preview renders without missing-include diagnostics. Local destinations, reference labels, and fragments passed checks across the ten reviewed preview pages. Unreviewed future chapters are not claimed link-clean.

- Fixed the stale No Allocation include and clarified fallible versus panicking capacity operations.
- Fixed Embassy host cleanup: the executor now waits for the deadline task to complete or cancel its timer after the consumer signals success. A repeated-run regression check verifies static task slots can be reused.
- Added bounded native smoke modes without changing normal interactive behavior. The ggez smoke countdown safely handles additional callbacks during deferred shutdown.
- Added shader validation, a render/readback pixel contract, and malformed-frame checks; GPU tests remain explicit opt-ins.
- Added held-out classifier evaluation, chronological forecast evaluation, finite interval checks, ONNX candidate probing, rejected-swap preservation, and retained-snapshot tests. Model artifact bytes were not regenerated; the optional Python generator now writes beside its script regardless of working directory.
- Corrected the Science ML dependency-version description and HRTF 0.10 history/frame-size guidance.

Detailed command logs and the No Allocation preview are under the local external build directory `D:/code_build/rust_howto/main/platform-review`. This review does not change CI ownership or close physical/visual validation gaps merely because compilation succeeded.
