# 2D Renderers

{{#include 2d_renderers.incl.md}}

## `femtovg` {#femtovg}

[![femtovg][c-femtovg-badge]][c-femtovg] [![femtovg-crates.io][c-femtovg-crates.io-badge]][c-femtovg-crates.io] [![femtovg-github][c-femtovg-github-badge]][c-femtovg-github] [![femtovg-lib.rs][c-femtovg-lib.rs-badge]][c-femtovg-lib.rs]{{hi:femtovg}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

Antialiased 2D vector drawing library. "OpenGL based. Offers a simple API. Probably the easiest to get started with."

```rust,editable
{{#include ../../../crates/ex/cats/gui/tests/femtovg.rs:example}}
```

## `skia-safe` {#skia-safe}

[![skia-safe][c-skia_safe-badge]][c-skia_safe] [![skia-safe-crates.io][c-skia_safe-crates.io-badge]][c-skia_safe-crates.io] [![skia-safe-github][c-skia_safe-github-badge]][c-skia_safe-github] [![skia-safe-lib.rs][c-skia_safe-lib.rs-badge]][c-skia_safe-lib.rs]{{hi:skia-safe}}{{hi:Opengl}}{{hi:Pdf}}{{hi:Rust-bindings}}{{hi:Skia}}{{hi:Vulkan}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}} [![cat-multimedia::images][cat-multimedia::images-badge]][cat-multimedia::images]{{hi:Images}} [![cat-rendering::graphics-api][cat-rendering::graphics-api-badge]][cat-rendering::graphics-api]{{hi:Graphics APIs}} [![cat-visualization][cat-visualization-badge]][cat-visualization]{{hi:Visualization}}

"Bindings to the Skia C++ library. The most complete option with excellent performance. However, it can be difficult to get it to compile."

```rust,editable
{{#include ../../../crates/ex/cats/gui/tests/skia_safe.rs:example}}
```

## `vello` {#vello}

[![vello][c-vello-badge]][c-vello] [![vello-crates.io][c-vello-crates.io-badge]][c-vello-crates.io] [![vello-github][c-vello-github-badge]][c-vello-github] [![vello-lib.rs][c-vello-lib.rs-badge]][c-vello-lib.rs]{{hi:vello}}{{hi:2d}}{{hi:Vector-graphics}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}} [![cat-rendering][cat-rendering-badge]][cat-rendering]{{hi:Rendering}}

A GPU compute-centric 2D renderer. "WGPU-based and uses cutting edge techniques to render vector paths using the GPU. Still somewhat immature and hasn't yet put out a stable release."

```rust,editable
{{#include ../../../crates/ex/cats/gui/tests/vello.rs:example}}
```

## `vger` {#vger}

[![vger][c-vger-badge]][c-vger] [![vger-crates.io][c-vger-crates.io-badge]][c-vger-crates.io] [![vger-github][c-vger-github-badge]][c-vger-github] [![vger-lib.rs][c-vger-lib.rs-badge]][c-vger-lib.rs]{{hi:vger}}{{hi:Canvas}}{{hi:Drawing}}{{hi:Gpu}}{{hi:Graphics}}{{hi:Vector}} [![cat-graphics][cat-graphics-badge]][cat-graphics]{{hi:Graphics}}

2D GPU renderer for dynamic UIs.

"A simpler WGPU-based option which is less innovative but currently more stable than vello."

```rust,editable
{{#include ../../../crates/ex/cats/gui/tests/vger.rs:example}}
```

## `webrender` {#webrender}

[![webrender][c-webrender-badge]][c-webrender] [![webrender-crates.io][c-webrender-crates.io-badge]][c-webrender-crates.io] [![webrender-github][c-webrender-github-badge]][c-webrender-github] [![webrender-lib.rs][c-webrender-lib.rs-badge]][c-webrender-lib.rs]{{hi:webrender}}

A GPU accelerated 2D renderer for web content. "OpenGL-based. Mature with production usage in Firefox but documentation and OSS maintenance are lacking."

```rust,editable
{{#include ../../../crates/ex/cats/gui/tests/webrender.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[2d_renderers: write; titles (P2)](https://github.com/john-cd/rust_howto/issues/377)

</div>
