# Game Development

[![cat-game-development][cat-game-development-badge]][cat-game-development]{{hi:Game development}}

This section focuses on individual parts of game development. For high-level game engines or frameworks, review the [[game-engines | Game Engines]] category.

Game development in Rust is a vibrant area, and the crates you'll need depend on the type of game you're making:

| Game Type | Rust crate(s) |
|---|---|
| Simple 2D Games | [`ggez`][c-ggez]⮳{{hi:ggez}}, [`macroquad`][c-macroquad]⮳{{hi:macroquad}}, or a combination of [`winit`][c-winit]⮳{{hi:winit}} , [`pixels`][c-pixels]⮳{{hi:pixels}}, and [`cpal`][c-cpal]⮳{{hi:cpal}} |
| More Complex 2D/3D Games | [`Bevy`][c-bevy]⮳{{hi:Bevy}} or [`Amethyst`][c-amethyst]⮳{{hi:Amethyst}} |
| Custom Engines or Low-Level Graphics | [`winit`][c-winit]⮳{{hi:winit}} , [`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}} |
| Data-Driven Design | ECS libraries like [`bevy_ecs`][c-bevy_ecs]⮳{{hi:bevy_ecs}}, [`specs`][c-specs]⮳{{hi:specs}}, or [`hecs`][c-hecs]⮳{{hi:hecs}} |

## Code Examples

{{#include game_development.incl.md}}

## Related Topics

| Topic | Description | Relevant Rust crate(s) |
|---|---|---|
| [[graphics | Graphics]] | | [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: A cross-platform, safe, and portable GPU API. Often used with [`winit`][c-winit]⮳{{hi:winit}} or game engines. [`rend3`][c-rend3]⮳{{hi:rend3}}: A 3D rendering engine built on top of wgpu. [`gfx-hal`][c-gfx_hal]⮳{{hi:gfx-hal}}: A low-level graphics API abstraction layer. [`image`][c-image]⮳{{hi:image}} for image loading and manipulation. |
| [[multimedia_audio | Audio]] | | [`cpal`][c-cpal]⮳{{hi:cpal}}: Cross-platform audio I/O. [`sdl2`][c-sdl2]⮳{{hi:sdl2}}: Can also be used for audio |
| Inputs | See also [[gui | GUI]]. | [`winit`][c-winit]⮳{{hi:winit}} : Handles window events, including input. [`sdl2`][c-sdl2]⮳{{hi:sdl2}}: Can also be used for input. |
| Game Logic / State Management | Often handled directly or with ECS libraries. See below. | |
| Entity Component System (ECS) | For Data-Driven Design. | [`bevy_ecs`][c-bevy_ecs]⮳{{hi:bevy_ecs}} is Bevy's built-in ECS. [`specs`][c-specs]⮳{{hi:specs}} is a popular and mature ECS library. [`hecs`][c-hecs]⮳{{hi:hecs}} is another ECS implementation. |
| Physics Engine | See [Physics](https://arewegameyet.rs/ecosystem/physics) for 2D and 3D physics engines, collision detection libraries. | [`rapier`](https://rapier.rs): A physics engine (2D and 3D). [`nphysics`][c-nphysics]⮳{{hi:nphysics}}: Another physics engine. |
| Networking | | [`ggrs`][c-ggrs]⮳{{hi:ggrs}}: A P2P networking library for games. [`tokio`][c-tokio]⮳{{hi:tokio}}: (For asynchronous networking in general). [`mio`][c-mio]⮳{{hi:mio}}: Lower-level networking. |
| UI (User Interface) | See [[gui | GUI]]. | [`egui`][c-egui]⮳{{hi:egui}}: An immediate mode GUI library. [`iced`][c-iced]⮳{{hi:iced}}: A cross-platform GUI library focused on simplicity and type safety. [`conrod`][c-conrod]⮳{{hi:conrod}}: An older UI library. |
| [[mathematics | Mathematics]] and [[linear_algebra | Linear Algebra]] | | [`nalgebra`][c-nalgebra]⮳{{hi:nalgebra}}: A popular linear algebra library. [`glam`][c-glam]⮳{{hi:glam}}: Another linear algebra library, often used in game development. |
| Asset Loading and Management | See [3D Format Loaders](https://arewegameyet.rs/ecosystem/3dformatloaders) for FBX, OBJ and more. Crates like [`image`][c-image]⮳{{hi:image}} are often used for image loading. | |
| Serialization/Deserialization | For game state, assets, etc. | [`serde`][c-serde]⮳{{hi:serde}}: Widely used serialization framework (saving game state, loading assets, etc.). |
| Scripting | See [Scripting Languages](https://arewegameyet.rs/ecosystem/scripting) for scripting languages embeddable in a Rust game. | [`rhai`][c-rhai]⮳{{hi:rhai}}: An embeddable scripting language. `lua`: Lua bindings. |
| Other Useful Crates | | [`rand`][c-rand]⮳{{hi:rand}}: For random number generation. |

See also:

- [[artificial_intelligence | Artificial Intelligence]] for Robotics.
- [[classical_machine_learning | Classical Machine Learning]].
- [[deep_learning | Deep Learning]].
- [[games | Games]].
- [[game_engines | Game Engines]].

## References

Are we Game yet?

- [AI](https://arewegameyet.rs/ecosystem/ai/): AI libraries, steering, behaviour trees, planners, etc.
- [Animation](https://arewegameyet.rs/ecosystem/animation/): rigging, tweening, anything related to animation.
- [Audio](https://arewegameyet.rs/ecosystem/audio/): Wrappers for FMOD, OpenAL, MIDI and similar, and higher level APIs.
- [ECS](https://arewegameyet.rs/ecosystem/ecs/): Entity Component System implementations.
- [Input](https://arewegameyet.rs/ecosystem/input/): Libraries to handle controllers, gamepads, keyboards, etc.
- [Math](https://arewegameyet.rs/ecosystem/math/): Linear algebra libraries, quaternions, color conversion and more.
- [Mesh Tools](https://arewegameyet.rs/ecosystem/mesh/): Tools for optimising and processing meshes.
- [Networking](https://arewegameyet.rs/ecosystem/networking/): Multiplayer, Protocols, and more.
- [Tools](https://arewegameyet.rs/ecosystem/tools/): Tools & other game-dev related libraries.
- [UI](https://arewegameyet.rs/ecosystem/ui/): Immediate mode UI libraries and more.
- [VR](https://arewegameyet.rs/ecosystem/vr/): VR engines and libraries.
- [Windowing](https://arewegameyet.rs/ecosystem/windowing/) Windowing and Context Creation crates.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[expand](https://github.com/john-cd/rust_howto/issues/367)
cover [`tetra`][c-tetra]⮳{{hi:tetra}}, [`piston`][c-piston]⮳{{hi:piston}}.
</div>
