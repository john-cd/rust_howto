<!-- TODO Reviewed ??-??-2026. -->

# Rendering Engines

## Windowed Scene

The `engine_scene` example uses three-d to load the bundled OBJ and present
it on an OpenGL desktop:

```console
just cargo test --manifest-path bk/crates/Cargo.toml --locked -p rendering --features engine-scene --example engine_scene
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p rendering --features engine-scene --example engine_scene
```

The asset tests need no display. The application requires an OpenGL-capable
desktop. See `bk/src/categories/rendering_engine/rendering_engines.md`.

## Offscreen Rendering

The `offscreen_render` example renders a triangle with wgpu and reads back
pixels without opening a window:

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p rendering --features wgpu --example offscreen_render
```

Running the example requires a wgpu adapter.
