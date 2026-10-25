<!-- TODO Reviewed ??-??-2026 -->

# Game Engines

[![cat~game-engines][cat~game-engines~badge]][cat~game-engines]

This category covers full game engines, game frameworks, and bindings that connect Rust to another engine. Their scope and supported platforms differ, so compare the features and development model you need rather than treating every entry as interchangeable.

| Project                                             | Kind and starting point                                                                                                     |
| --------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| [`Bevy`][c~bevy~docs]↗{{hi:Bevy}}                   | ECS-based game engine with a renderer; supports building 2D and 3D games.                                                   |
| [`Fyrox`][c~fyrox~docs]↗{{hi:Fyrox}}                | 3D game engine with an editor.                                                                                              |
| [`ggez`][c~ggez~docs]↗{{hi:ggez}}                   | 2D game framework.                                                                                                          |
| [`macroquad`][c~macroquad~docs]↗{{hi:macroquad}}    | Lightweight 2D game framework.                                                                                              |
| [`miniquad`][c~miniquad~docs]↗{{hi:miniquad}}       | Cross-platform rendering library; a lower-level building block used by game frameworks.                                     |
| [`godot-rust`][c~godot-rust~docs]↗{{hi:godot-rust}} | Rust bindings for extending the Godot engine with Rust code; Godot remains the engine and editor.                           |
| [`Tetra`][c~tetra~docs]↗{{hi:Tetra}}                | 2D game framework. Its [upstream status note][tetra~status~repo]↗ says it has been passively maintained since January 2022. |

Before choosing, check the project's current release activity, platform support, renderer, editor and asset workflows, and deployment requirements. The [Are We Game Yet? engine catalog][are-we-game-yet?~engines~website]↗ tracks a broader set of engines and frameworks.

Code examples are found below:

{{#include game_engines.incl.md}}

## Related Topics

- [[games | Games]].
- [[game-development | Game Development]].
- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[window_creation | Window Creation]]. [`winit`][c~winit~docs]↗{{hi:winit}}: A window creation and event handling library. Often used as a foundation for custom engines or when more control is needed.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand](https://github.com/john-cd/rust_howto/issues/370)
[pyxel][pyxel~repo]↗: a retro game engine for Python (not Rust; listed for comparison).
</div>
