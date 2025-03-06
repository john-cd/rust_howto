# Vector-Based 2D Rendering

{{#include 2d_vector_graphics.incl.md}}

[`lyon`][c-lyon]⮳{{hi:lyon}} generates vector paths and shapes. Often used with a rendering library. For basic geometry, you might just use structs and functions within your own code. [`lyon`][c-lyon]⮳{{hi:lyon}} can also generate various types of curves (Bézier curves, etc.).

It's useful for creating the geometry for vector graphics, but you'd typically use a rendering library like [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`raqote`][c-raqote]⮳{{hi:raqote}}, or [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} to actually draw them.

Also consider `femtovg`.

## `lyon` {#lyon}

[`lyon`][c-lyon]⮳{{hi:lyon}} is a library for generating vector paths and shapes. It's useful for creating the geometry for vector [graphics][p-graphics], but you'd typically use a [rendering][p-rendering] library like [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`raqote`][c-raqote]⮳{{hi:raqote}}, or [`tiny-skia`][c-tiny_skia]⮳{{hi:tiny-skia}} to actually draw them. [`lyon`][c-lyon]⮳{{hi:lyon}} handles the math and path calculations.

## Related Topics

## See also

[2D Rendering](https://arewegameyet.rs/ecosystem/2drendering/): Sprites, vectors, splines, hex grids and more

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
</div>
