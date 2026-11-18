<!-- TODO Reviewed ??-??-2026. -->

# Grouped Category Migration Plan

Prioritize complete, coherent groups over isolated recipes. This plan inventories the
remaining staged content on November 17, 2026; it proposes work rather than certifying
new runtime results. The evidence in [PLATFORM_READINESS.md](PLATFORM_READINESS.md)
applies only to the checks recorded there. A whole-category migration may leave an
explicitly optional backend unverified, but it must not advertise that backend as a
working recipe. Remove unsupported recipe promises or complete them before migration.

## Scope and Order

| Order | Group | Current position | Work before a single grouped migration |
| --- | --- | --- | --- |
| 1 | Machine Learning: practical workflows, classical algorithms, and deep learning | Practical ML and classical ML each have 9 passing tests recorded in the readiness review. Deep-learning training and accelerator paths still need an audit. ML is a high-priority group, separate from general science. | Connect preprocessing, training, held-out evaluation, model persistence and inference; complete a bounded CPU deep-learning workflow; validate the full group before migration. |
| 2 | Native operating-system APIs: Linux, macOS, FreeBSD, with existing Windows coverage | macOS and FreeBSD introductory native checks passed; Linux native execution remains open. Windows is already published. Small, bounded group with broad systems-programming value. | Complete native Linux validation; polish all three staged categories together; align their selection/prerequisite guidance with the published Windows chapter. |
| 3 | Multimedia: overview, audio, encoding, video, streaming; integrate existing Images coverage | Audio has substantial deterministic coverage and a Windows CPAL stream result. Encoding/video have recipes, but end-to-end media round trips and backend prerequisites still need a joint audit. Images is already published. | Close codec/container/FFmpeg and stream lifecycle gaps; make all chapter promises match runnable examples; migrate the remaining multimedia categories together. |
| 4 | Games, Game Development, Game Engines | Macroquad/ggez bounded native runs passed; the surrounding category is partly discovery links and unvalidated Bevy/Fyrox descriptions. | Add a coherent small game and a tested engine workflow; give each listed engine an honest documented disposition; migrate all three categories together. |
| 5 | Rendering, Graphics APIs, Rendering Engines, Rendering Data Formats; align existing Graphics/Images | The core offscreen pipeline and format recipes have good evidence. Several engine/native API sections remain catalogs. CPU graphics recipes are already published. | Consolidate duplicated coverage; complete an engine-owned scene lifecycle and the remaining native API recipes; migrate this group as a coherent extension of published Graphics. |
| 6 | General scientific computing and geospatial processing, excluding ML | Geo has substantial recipes and a feature-gated GEOS path; the Science landing page is currently dominated by ML. | Audit numerical coverage already in the book; add a non-ML scientific workflow, validate geometry/coordinate handling, and reorganize Science navigation before a grouped migration. |
| 7 | GUI, with accessibility and platform support | Windows presentation and headless logic checks passed. Toolkit, input/resize, accessibility, and native Linux gaps remain. | Finish a documented platform/toolkit matrix and real interaction coverage before promoting the whole category. |

Start with ML, then native APIs and multimedia. Games and graphics share windowing
and rendering infrastructure; prepare them together when that reduces duplicated
work. Keep ML and the completed native-API or multimedia groups moving independently
of a larger engine rewrite.

## Machine Learning Group

ML is a first-priority group, not deferred scope. Group Machine Learning in Practice,
classical ML, and deep learning into a coherent learning path; keep their migration
separate from general scientific computing so neither group waits for unrelated work.

- Explain data preparation, train/validation/test separation, leakage prevention,
  preprocessing fitted only on training data, metric selection, and baseline comparison.
  Connect the existing classification and chronological forecasting examples to this path.
- Review classical classification, regression, and clustering examples for reproducible
  seeds, meaningful assertions, and algorithm assumptions. State the limits of synthetic
  fixtures instead of treating test accuracy as real-world model quality.
- Complete one small deterministic CPU training-to-inference deep-learning workflow with
  bounded runtime, a loss/quality check, saved/reloaded model parity, and malformed-input
  handling. Audit the existing chapter before choosing which framework example to polish.
- Explain ONNX tensor shapes, element types, preprocessing contracts, runtime setup,
  model provenance/licensing, and inference failures. Preserve the existing model-swap
  rejection and retained-snapshot tests; verify bundled model artifacts are reproducible.
- Separate the tested CPU baseline from optional CUDA/other accelerators. Exercise any
  advertised accelerator recipe on its actual backend or explicitly document it as
  optional background; do not make every accelerator a prerequisite for the CPU group.
- Consolidate overlapping framework comparisons and link to existing arrays, linear
  algebra, and data-processing chapters. Give practical ML, classical ML, and deep
  learning distinct purposes while providing one navigation path through all three.
- Re-run the practical ONNX and classical suites, strict Clippy, baseline toolchain,
  and the selected CPU deep-learning workflow at the final revision. Prior evidence is
  recorded in [PLATFORM_READINESS.md](PLATFORM_READINESS.md); it does not certify all
  deep-learning or accelerator examples.

## Native API Group

- Run the Linux example on native Linux and test successful metadata queries plus
  missing-path/error behavior. Record the actual target/toolchain and native test step.
- Recheck the macOS and FreeBSD evidence against the final source revision. Test the
  successful query and one ordinary error case on each platform.
- Distinguish implemented examples from orientation-only mentions of io_uring, D-Bus,
  systemd, eBPF, Keychain, AppKit, Capsicum, jails, and kqueue. Add an example only when
  it teaches a distinct necessary operation; shorten unsupported catalogs instead of
  implying full API coverage.
- Use consistent prerequisite, permission, platform-gating, run-command, cleanup, and
  error-handling sections. Connect the existing Windows category to the same overview.
- Tracking: [#2358](https://github.com/john-cd/rust_howto/issues/2358) for Linux;
  [#2214](https://github.com/john-cd/rust_howto/issues/2214) for native CI ownership.

## Multimedia Group

- Use one generated short audio/video fixture across encoding, video, and the multimedia
  overview. Demonstrate generation, encode/decode, audio extraction, and thumbnail
  creation; assert duration/sample count, dimensions, and decoded content with appropriate
  lossy tolerances. Keep generated files in temporary output and clean them up.
- Clarify the boundary between codecs, containers, pixel/sample formats, and playback.
  Remove repeated rav1e/WAV recipes in favor of one canonical implementation per task.
- Document FFmpeg/GStreamer discovery and feature requirements. Exercise both an ordinary
  round trip and a missing-tool/bad-input failure without silently skipping the example.
- Add bounded playback/capture examples with shutdown and device-selection behavior.
  Keep deterministic signal/format checks separate from real device checks. Close MIDI
  loopback coverage or describe MIDI as optional background rather than a tested recipe.
- Check whether the streaming chapter is a runnable local session or only signaling
  fragments. Add a bounded loopback/session teardown example before presenting it as a
  complete workflow; explain any external service or browser requirements.
- Check Windows and Linux media backends; include macOS only where support is claimed.
  Reuse published Images for frame processing and published networking for transport.
- Tracking: [#2356](https://github.com/john-cd/rust_howto/issues/2356) for device/backend gaps.

## Games and Graphics Groups

- Expand the small game into an explained update/render/input loop with deterministic
  state tests, collision/scoring, resize behavior, and bounded smoke mode. Keep the Games
  discovery list brief and separate from the tutorial's tested behavior.
- Add one complete engine scene with camera, geometry/material, asset handling, update,
  and clean shutdown. Validate the native window and an observable rendered result.
  Either test Bevy/Fyrox recipes or retain concise comparisons with explicit prerequisites.
- Keep introductory raster/vector/SVG/projection work in published Graphics. Preserve the
  distinct staged fontdue recipe only if it adds useful teaching beyond the published
  skrifa example; otherwise retire it after comparison.
- Add a small asset-to-frame integration: parse a model, prepare texture data, render it,
  and check output. Explain coordinate systems, color space, row alignment, resource
  ownership, and malformed asset errors where they affect the example.
- Audit OpenGL context ownership, Vulkan prerequisites, shader compilation, and window
  surface resize/recovery. Discovery-only tests must not stand in for rendered output.
- Tracking: [#2357](https://github.com/john-cd/rust_howto/issues/2357) for engines and
  [#2359](https://github.com/john-cd/rust_howto/issues/2359) for GUI/platform interaction.

## Science Without ML

- Reorganize the Science overview around numerical computation, measurement/simulation,
  data formats, and spatial analysis. Keep existing classical/deep-learning chapters and
  Machine Learning in Practice in the separate first-priority ML group.
- Reuse existing math, statistics, arrays, linear algebra, and data-processing coverage.
  Add one small physical/numerical computation with units, input assumptions, tolerance,
  and a known reference result; avoid adding another crate catalog.
- In Geo, test GeoJSON properties/geometry handling, spatial queries, distance units,
  coordinate order, invalid geometries, and empty results. State which coordinates and
  reference systems operations accept. Add a reproducible projection example or narrow
  the current projection promises. Run GEOS only with its actual native dependency.
- Make geocoding optional and isolate service-dependent calls from deterministic tests.
- Leave neuroscience and other highly specialized science chapters staged unless separately
  prioritized. Defer robotics and embedded hardware until their device/ROS gaps have
  evidence; see [#2355](https://github.com/john-cd/rust_howto/issues/2355).

## Exit Criteria for Every Group

1. Every category and chapter has a clear purpose, prerequisites, runnable commands,
   explained examples and output, error/cleanup behavior, and useful related topics.
   Catalog entries do not substitute for the workflows promised by the chapter.
2. Audit every Markdown include and Rust target. Run relevant unit/integration tests,
   formatting, strict Clippy, and the supported baseline compile check. Run native or
   device checks wherever an example claims that behavior; record remaining optional limits.
3. Move the entire accepted category group, source assets, features/dependencies, and
   examples into `bk`; update SUMMARY, shared references, indices, related topics, and
   cross-category links. Avoid duplicated implementations across categories.
4. Render the published book and check affected local links and fragments. Review output
   for include failures and erroneous Playground controls on platform-only examples.
5. Remove migrated `later` chapters/packages and stale lockfile entries, render the
   remaining future book, and check links to canonical published destinations.
6. Commit each focused issue separately and integrate linearly into `develop` as groups
   finish. Record concrete evidence and close gap issues only when their criteria pass.

## Deferred Scope

Aerospace, cryptocurrency/crypto-specific application catalogs, and other highly
specialized domains are outside this plan. ML has its own first-priority group. Robotics,
embedded firmware, and specialized neuroscience are lower priority than the broad
native-API, multimedia, games/graphics, and general-science groups above.
