<!-- TODO Reviewed ??-??-2026. -->

# Rendering

This staged package retains the distinct `fontdue`/`ttf-parser` glyph recipe.
The raster, tessellation, vector, SVG, and camera-projection examples are published
in `bk/crates/cats/graphics`; their duplicate staged sources have been removed.

Run the remaining example from the repository root:

```console
just cargo test --manifest-path later/crates/Cargo.toml --locked -p rendering --example text_rendering
```

The chapter is in `later/src/categories/rendering/text_rendering.md`.

## Windowed Scene

The `engine_scene` example uses three-d to load the bundled OBJ and present
it on an OpenGL desktop:

```console
just cargo test --manifest-path later/crates/Cargo.toml --locked -p rendering --features engine-scene --example engine_scene
just cargo run --manifest-path later/crates/Cargo.toml --locked -p rendering --features engine-scene --example engine_scene
```

The asset tests need no display. The application requires an OpenGL-capable
desktop. See `later/src/categories/rendering_engine/rendering_engines.md`.
