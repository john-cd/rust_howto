<!-- TODO Reviewed ??-??-2026 -->

# Game Development

[![cat~game-development][cat~game-development~badge]][cat~game-development]{{hi:Game development}}

This category collects building blocks for making games with Rust, from game frameworks and math libraries to graphics, audio, input, physics, and networking. Start with the recipes below, or compare complete engines and frameworks in the [[game-engines | Game Engines]] category.

For a small example, open the [game development recipes][ex~game-development~game-development]. Choose a framework or engine based on your target platforms, renderer, editor and asset workflow, and the amount of control you need. You can also combine focused libraries to build a custom engine.

## Game Development Recipes

The table is maintained in a separate include so the recipe links stay aligned with the examples:

{{#include game_development.incl.md}}

## Building Blocks

- **Game frameworks:** [`ggez`][c~ggez~docs]↗{{hi:ggez}} and [`macroquad`][c~macroquad~docs]↗{{hi:macroquad}} provide higher-level starting points for 2D games. See the [[game-engines | Game Engines]] category for more complete options such as Bevy and Fyrox.
- **Windowing and graphics:** [`winit`][c~winit~docs]↗{{hi:winit}} handles windows and input events; [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} provides a portable graphics API. Browse the [2D rendering][are-we-game-yet?-2d-rendering~website]↗ and [3D rendering][are-we-game-yet?-3d-rendering~website]↗ catalogs for related projects.
- **Math and physics:** [`glam`][c~glam~docs]↗{{hi:glam}} provides game-oriented vector and matrix types; [`rapier`][rapier.rs~website]↗ provides 2D and 3D physics. Browse the [math][are-we-game-yet?-math~website]↗ and [physics][are-we-game-yet?-physics~website]↗ catalogs for more options.
- **Audio and input:** [`cpal`][c~cpal~docs]↗{{hi:cpal}} provides cross-platform audio I/O. [`winit`][c~winit~docs]↗{{hi:winit}} and [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}} can handle input; compare libraries in the [audio][are-we-game-yet?-audio~website]↗ and [input][are-we-game-yet?-input~website]↗ catalogs.
- **Entity component systems:** [`bevy_ecs`][c~bevy_ecs~docs]↗{{hi:bevy_ecs}}, [`hecs`][c~hecs~docs]↗{{hi:hecs}}, and [`specs`][c~specs~docs]↗{{hi:specs}} help organize data-driven game logic. See the [ECS catalog][are-we-game-yet?-ecs~website]↗ for other choices.
- **Other systems:** Find libraries for animation, assets, networking, scripting, UI, and VR in the [Are We Game Yet? catalog][are-we-game-yet?~website]↗.

## Project Status

[`Tetra`][c~tetra~docs]↗{{hi:Tetra}} is passively maintained: its upstream project plans no new features, though it may still release occasional fixes. The maintainers suggest [`macroquad`][c~macroquad~docs]↗{{hi:macroquad}} or [`ggez`][c~ggez~docs]↗{{hi:ggez}} for a more actively developed 2D framework; see the [Tetra status note][tetra~status~repo]↗.

Piston is a modular engine, so activity can differ between its components. Check the [core repository's activity][piston~activity~repo] and the crates you intend to use. If you prefer an integrated engine or framework, compare Bevy, Fyrox, ggez, and Macroquad in the [[game-engines | Game Engines]] category.

## Related Topics

- [[games | Games]] written in Rust.
- [[graphics | Graphics]] and [[rendering | Rendering]].
- [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]].
- [[artificial_intelligence | Artificial Intelligence]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
