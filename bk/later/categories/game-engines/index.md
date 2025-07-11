# Game Engines

[![cat~game-engines][cat~game-engines~badge]][cat~game-engines]

Crates that try to provide a "one-stop-shop" for all of your game development needs.

| |
|---|
| [`Bevy`][c~bevy~docs]⮳{{hi:Bevy}}: A data-driven game engine built on the Entity Component System (ECS) architecture. Includes a rendering system. Very popular, actively developed, and a great choice for both 2D and 3D games. Known for its ease of use and rapid iteration. |
| [`Amethyst`][c~amethyst~docs]⮳{{hi:Amethyst}}: Another data-oriented game engine, also using an ECS architecture and with rendering capabilities. Focuses on data-driven design and modularity. |
| [`ggez`][c~ggez~docs]⮳{{hi:ggez}}: A simple and easy-to-use 2D game framework. Good for beginners, learning, and smaller projects. Less complex than Bevy or Amethyst. |
| [`macroquad`][c~macroquad~docs]⮳{{hi:macroquad}}: A user-friendly framework for 2D games and interactive applications. It focuses on ease of use and quick prototyping. |
| [`Fyrox`][c~fyrox~docs]⮳{{hi:Fyrox}}: A 3D game engine with a visual editor. Good for those who prefer a more visual approach to game development. |
| [`Godot-Rust`][c~godot_rust~docs]⮳{{hi:Godot-Rust}}: Bindings that allow you to use Rust as a scripting language within the Godot Engine. This lets you leverage Godot's features and editor while writing game logic in Rust. |
| [`Tetra`][c~tetra~docs]⮳{{hi:Tetra}}: A simple 2D game framework inspired by Love2D. |

It's worth noting that "game engine" is a broad term. Some of these are full-fledged engines with built-in rendering, physics, and other features (like [`Bevy`][c~bevy~docs]⮳{{hi:Bevy}}, [`Amethyst`][c~amethyst~docs]⮳{{hi:Amethyst}}, and [`Fyrox`][c~fyrox~docs]⮳{{hi:Fyrox}}). Others are more focused frameworks that provide the building blocks for creating games (like [`ggez`][c~ggez~docs]⮳{{hi:ggez}}, [`macroquad`][c~macroquad~docs]⮳{{hi:macroquad}}, and [`Tetra`][c~tetra~docs]⮳{{hi:Tetra}}). [`Godot-Rust`][c~godot_rust~docs]⮳{{hi:Godot-Rust}} is a special case, as it integrates Rust into an existing engine.

The best choice depends on your project's needs and your preferred development style. For beginners, [`ggez`][c~ggez~docs]⮳{{hi:ggez}}, `miniquad`, or [`macroquad`][c~macroquad~docs]⮳{{hi:macroquad}} might be good starting points. For more complex projects, [`bevy`][c~bevy~docs]⮳{{hi:bevy}} is a very strong contender. If you want a visual editor, [`Fyrox`][c~fyrox~docs]⮳{{hi:Fyrox}} is a good option. And if you like `Godot`, [`Godot-Rust`][c~godot_rust~docs]⮳{{hi:Godot-Rust}} lets you use Rust with it.

Code examples are found below:

{{#include game_engines.incl.md}}

## Related Topics

- [[games | Games]].
- [[game-development | Game Development]].
- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[window_creation | Window Creation]]. [`winit`][c~winit~docs]⮳{{hi:winit}}: A window creation and event handling library. Often used as a foundation for custom engines or when more control is needed.

## References

- [Are we Game yet? - Engines][are-we-game-yet?~engines~website].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand](https://github.com/john-cd/rust_howto/issues/370)
Needs full review

- [pyxel: A retro game engine for Python](https://github.com/kitao/pyxel)

</div>
