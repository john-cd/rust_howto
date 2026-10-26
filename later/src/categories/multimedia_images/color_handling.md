<!-- TODO Reviewed ??-??-2026 -->

# Color Handling

{{#include color_handling.incl.md}}

A color value needs both its component values and a definition of what those values mean. RGB channels alone do not identify the color: the primaries, white point, transfer function, and component range matter. Keep this information with the value as it moves between image decoding, processing, and display.

## Encoded sRGB and Linear Light

Most common image files and displays use sRGB-encoded channel values. Those encoded values are convenient for storage and display, but they are nonlinear with respect to light intensity. Operations such as blending, filtering, and interpolation should generally be performed in a linear-light representation, then converted back to the output encoding.

[`palette`][c~palette~docs]↗{{hi:palette}} makes the distinction explicit in its types. For example, convert an 8-bit sRGB color to linear sRGB for processing, then encode it for display:

```rust
use palette::{LinSrgb, Srgb};

let stored = Srgb::<u8>::new(128, 64, 32);
let linear: LinSrgb<f32> = stored.into_linear();

// Perform color calculations on `linear` values here.

let display: Srgb<u8> = Srgb::from_linear(linear);
```

Do not interpret `Srgb<u8>` components as linear values in the range 0.0 to 1.0 without decoding the transfer function. Likewise, do not apply the RGB transfer function to alpha; alpha describes coverage or opacity, not light intensity.

## Alpha and Compositing

An RGBA buffer may store straight alpha, where RGB is independent of alpha, or premultiplied alpha, where RGB is already multiplied by alpha. Compositing formulas and renderer APIs expect a particular convention. Convert consistently at the boundary; otherwise translucent edges can show dark or bright fringes. See [Pixel Buffers](pixel_buffers.md) for channel layout and storage details.

When blending colors, decode RGB into a linear space first. Keep alpha linear, composite using the renderer's convention, and encode RGB back to the output color space afterward. A blend that directly averages encoded sRGB bytes is not the same as a blend of light intensities.

## Choosing a Color Space

- Use sRGB for common display and web-facing image data when that is the format expected by the file or surface.
- Use a linear RGB space for lighting, compositing, and filtering calculations that model light.
- Use a perceptual space for tasks such as comparing or adjusting perceived lightness, then convert to the target output space.
- Track profiles and gamut when input images or displays use color spaces beyond sRGB. Converting channel types alone does not interpret an embedded ICC profile or guarantee that a color fits the destination gamut.

`palette` provides color types and conversions; it is not a substitute for a complete color-management pipeline when an application must honor embedded profiles and display characteristics.

## `palette` {#palette}

[![palette][c~palette~docs~badge]][c~palette~docs] [![palette~crates.io][c~palette~crates.io~badge]][c~palette~crates.io] [![palette~repo][c~palette~repo~badge]][c~palette~repo] [![palette~lib.rs][c~palette~lib.rs~badge]][c~palette~lib.rs]{{hi:palette}}{{hi:Linear}}{{hi:Pixel}}{{hi:Rgb}}{{hi:Color}}{{hi:Conversion}} [![cat~multimedia::images][cat~multimedia::images~badge]][cat~multimedia::images]{{hi:Images}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Convert and manage colors with a focus on correctness, flexibility and ease of use.

{{#example palette}}

## Related Topics {#related-topics .skip}

- [Images](images.md) covers loading, saving, and image formats.
- [Pixel Buffers](pixel_buffers.md) covers channel layout and alpha storage.
- [Rendering](../rendering/index.md) covers display and compositing APIs.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  write
</div>
