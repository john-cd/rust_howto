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
