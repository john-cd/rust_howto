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
| 4 | Games, Game Development, Game Engines | The Macroquad game and desktop frame loops have runtime evidence. Parry and glam teach collision queries and transforms. Bevy and Fyrox cover headless logic. | Migrate all three categories together. Retarget GUI and physics links, and preserve the documented limits of the headless engine examples. |
| 5 | Graphics and Rendering | Migrated under [Graphics](../bk/src/categories/graphics/index.md). | Keep GUI renderer examples with their GUI package. |
| 6 | General scientific computing and geospatial processing, excluding ML | Geo has substantial recipes and a feature-gated GEOS path; the Science landing page is currently dominated by ML. | Audit numerical coverage already in the book; add a non-ML scientific workflow, validate geometry/coordinate handling, and reorganize Science navigation before a grouped migration. |
| 7 | GUI, with accessibility and platform support | Windows presentation and headless logic checks passed. Toolkit, input/resize, accessibility, and native Linux gaps remain. | Finish a documented platform/toolkit matrix and real interaction coverage before promoting the whole category. |

Games and graphics share windowing
and rendering infrastructure; prepare them together when that reduces duplicated
work. Keep independent groups moving while engine work continues.

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

## Published Native OS and Multimedia Groups

Native OS APIs and Multimedia have moved to the published book. Their platform
and runtime evidence remains in [Platform readiness](PLATFORM_READINESS.md).

- [Native OS APIs and Process Boundaries](../bk/src/other/native-os/index.md).
- [Multimedia](../bk/src/categories/multimedia/index.md).

## Games and Graphics Groups

- Keep the playable marker game, independent collision and transform recipes, and
  engine comparison together. The marker game includes deterministic state tests and
  a bounded desktop smoke mode.
- Graphics and Rendering are in [the published Graphics section](../bk/src/categories/graphics/index.md). The windowed three-d recipe has prior Linux lifecycle and pixel evidence in #2357. The separate wgpu examples document shader validation, GPU readback, and offscreen rendering.
- Keep additional engine and surface recovery work tied to the application that needs it.

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
