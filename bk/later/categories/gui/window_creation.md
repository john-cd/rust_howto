# Window Creation

{{#include window_creation.incl.md}}

## `winit` {#winit}

[![winit][c~winit~docs~badge]][c~winit~docs] [![winit~crates.io][c~winit~crates.io~badge]][c~winit~crates.io] [![winit~github][c~winit~github~badge]][c~winit~github] [![winit~lib.rs][c~winit~lib.rs~badge]][c~winit~lib.rs]{{hi:winit}}{{hi:Windowing}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`winit`][c~winit~docs]⮳{{hi:winit}} is a cross-platform window creation and event handling library. "The defacto standard option. Uses an event loop based architecture. Widely used and should probably be the default choice." ([blessed.rs](https://blessed.rs/crates)).

```rust,editable
{{#include ../../../crates/cats/gui/examples/window_creation/winit.rs:example}}
```

## `tao` {#tao}

[![tao][c~tao~docs~badge]][c~tao~docs] [![tao~crates.io][c~tao~crates.io~badge]][c~tao~crates.io] [![tao~github][c~tao~github~badge]][c~tao~github] [![tao~lib.rs][c~tao~lib.rs~badge]][c~tao~lib.rs]{{hi:tao}}{{hi:Windowing}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}}

[`tao`][c~tao~docs]⮳{{hi:tao}} is a cross-platform window manager library. The TAO of cross-platform windowing. A library in Rust built for [`tauri`][c~tauri~docs]⮳{{hi:tauri}}.

"A fork of [`winit`][c~winit~docs]⮳{{hi:winit}} by the Tauri project, which adds support for things like system menus that desktop apps need." ([blessed.rs](https://blessed.rs/crates)).

```rust,editable
{{#include ../../../crates/cats/gui/examples/window_creation/tao.rs:example}}
```

## `baseview` {#baseview}

[![baseview~github][baseview~github~badge]][baseview~github]

`baseview` is a low-level window system interface for [audio][p~audio] plugin UIs.

```rust
{{#include ../../../crates/cats/gui/examples/window_creation/baseview.rs:example}}
```

## See Also

[`sdl2`][c~sdl2~docs]⮳{{hi:sdl2}} can also be used for windowing and input, but [`winit`][c~winit~docs]⮳{{hi:winit}} is often preferred in the Rust ecosystem.

[`glfw`][c~glfw~docs]⮳{{hi:glfw}} provides bindings to the GLFW library.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[window_creation: write](https://github.com/john-cd/rust_howto/issues/396)
</div>
