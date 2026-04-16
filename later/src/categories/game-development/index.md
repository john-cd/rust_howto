# Game Development

[![cat~game-development][cat~game-development~badge]][cat~game-development]{{hi:Game development}}

This section focuses on individual parts of game development. For high-level game engines or frameworks, review the [[game-engines | Game Engines]] category.

Game development in Rust is a vibrant area, and the crates you'll need depend on the type of game you're making:

| Game Type | Rust crate(s) |
|---|---|
| Simple 2D Games | [`ggez`][c~ggez~docs]↗{{hi:ggez}}, [`macroquad`][c~macroquad~docs]↗{{hi:macroquad}}, or a combination of [`winit`][c~winit~docs]↗{{hi:winit}} , [`pixels`][c~pixels~docs]↗{{hi:pixels}}, and [`cpal`][c~cpal~docs]↗{{hi:cpal}} |
| More Complex 2D/3D Games | [`Bevy`][c~bevy~docs]↗{{hi:Bevy}} or [`Amethyst`][c~amethyst~docs]↗{{hi:Amethyst}} |
| Custom Engines or Low-Level Graphics | [`winit`][c~winit~docs]↗{{hi:winit}} , [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}} |
| Data-Driven Design | ECS libraries like [`bevy_ecs`][c~bevy_ecs~docs]↗{{hi:bevy_ecs}}, [`specs`][c~specs~docs]↗{{hi:specs}}, or [`hecs`][c~hecs~docs]↗{{hi:hecs}} |

## Code Examples

{{#include game_development.incl.md}}

## References

Are we Game yet?

- [AI][are-we-game-yet?-ai~website]↗: AI libraries, steering, behaviour trees, planners, etc.
- [Animation][are-we-game-yet?-animation~website]↗: rigging, tweening, anything related to animation.
- [Audio][are-we-game-yet?-audio~website]↗: Wrappers for FMOD, OpenAL, MIDI and similar, and higher level APIs.
- [ECS][are-we-game-yet?-ecs~website]↗: Entity Component System implementations.
- [Input][are-we-game-yet?-input~website]↗: Libraries to handle controllers, gamepads, keyboards, etc.
- [Math][are-we-game-yet?-math~website]↗: Linear algebra libraries, quaternions, color conversion and more.
- [Mesh Tools][are-we-game-yet?-mesh~website]↗: Tools for optimising and processing meshes.
- [Networking][are-we-game-yet?-networking~website]↗: Multiplayer, Protocols, and more.
- [Tools][are-we-game-yet?-tools~website]↗: Tools & other game-dev related libraries.
- [UI][are-we-game-yet?-ui~website]↗: Immediate mode UI libraries and more.
- [VR][are-we-game-yet?-vr~website]↗: VR engines and libraries.
- [Windowing][are-we-game-yet?-windowing~website]↗ Windowing and Context Creation crates.

## Related Topics

| Topic | Description | Relevant Rust crate(s) |
|---|---|---|
| [[graphics | Graphics]] | | [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}: A cross-platform, safe, and portable GPU API. Often used with [`winit`][c~winit~docs]↗{{hi:winit}} or game engines. [`rend3`][c~rend3~docs]↗{{hi:rend3}}: A 3D rendering engine built on top of wgpu. [`gfx-hal`][c~gfx-hal~docs]↗{{hi:gfx-hal}}: A low-level graphics API abstraction layer. [`image`][c~image~docs]↗{{hi:image}} for image loading and manipulation. |
| [[multimedia_audio | Audio]] | | [`cpal`][c~cpal~docs]↗{{hi:cpal}}: Cross-platform audio I/O. [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}}: Can also be used for audio |
| Inputs | See also [[gui | GUI]]. | [`winit`][c~winit~docs]↗{{hi:winit}} : Handles window events, including input. [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}}: Can also be used for input. |
| Game Logic / State Management | Often handled directly or with ECS libraries. See below. | |
| Entity Component System (ECS) | For Data-Driven Design. | [`bevy_ecs`][c~bevy_ecs~docs]↗{{hi:bevy_ecs}} is Bevy's built-in ECS. [`specs`][c~specs~docs]↗{{hi:specs}} is a popular and mature ECS library. [`hecs`][c~hecs~docs]↗{{hi:hecs}} is another ECS implementation. |
| Physics Engine | See [Physics][are-we-game-yet?-physics~website]↗ for 2D and 3D physics engines, collision detection libraries. | [`rapier`][rapier.rs~website]↗: A physics engine (2D and 3D). `nphysics2d`{{hi:nphysics2d}}: Another physics engine. |
| Networking | | [`ggrs`][c~ggrs~docs]↗{{hi:ggrs}}: A P2P networking library for games. [`tokio`][c~tokio~docs]↗{{hi:tokio}}: (For asynchronous networking in general). [`mio`][c~mio~docs]↗{{hi:mio}}: Lower-level networking. |
| UI (User Interface) | See [[gui | GUI]]. | [`egui`][c~egui~docs]↗{{hi:egui}}: An immediate mode GUI library. [`iced`][c~iced~docs]↗{{hi:iced}}: A cross-platform GUI library focused on simplicity and type safety. [`conrod`][c~conrod~docs]↗{{hi:conrod}}: An older UI library. |
| [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]] | | [`nalgebra`][c~nalgebra~docs]↗{{hi:nalgebra}}: A popular linear algebra library. [`glam`][c~glam~docs]↗{{hi:glam}}: Another linear algebra library, often used in game development. |
| Asset Loading and Management | See [3D Format Loaders][are-we-game-yet?~3dformatloaders~website]↗ for FBX, OBJ and more. Crates like [`image`][c~image~docs]↗{{hi:image}} are often used for image loading. |  |
| Serialization/Deserialization | For game state, assets, etc. | [`serde`][c~serde~docs]↗{{hi:serde}}: Widely used serialization framework (saving game state, loading assets, etc.). |
| Scripting | See [Scripting Languages][are-we-game-yet?~scripting-languages]↗ for scripting languages embeddable in a Rust game. | [`rhai`][c~rhai~docs]↗{{hi:rhai}}: An embeddable scripting language. [`lua`][c~lua~docs]↗{{hi:lua}}: Lua bindings. |
| Other Useful Crates | | [`rand`][c~rand~docs]↗{{hi:rand}}: For random number generation. |

See also:

- [[artificial_intelligence | Artificial Intelligence]] for Robotics.
- [[classical_machine_learning | Classical Machine Learning]].
- [[deep_learning | Deep Learning]].
- [[games | Games]].
- [[game_engines | Game Engines]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand](https://github.com/john-cd/rust_howto/issues/367)
cover [`tetra`][c~tetra~docs]↗{{hi:tetra}}, [`piston`][c~piston~docs]↗{{hi:piston}}.
</div>
