# Game Development

[![cat-game-development][cat-game-development-badge]][cat-game-development]{{hi:Game development}}

Crates that focus on some individual part of accelerating the development of games.

{{#include game_development.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[game-development/_index: expand (P2)](https://github.com/john-cd/rust_howto/issues/367)

Game development in Rust is a vibrant area, and the crates you'll need depend on the type of game you're making. Here's a breakdown:

## Engines/Frameworks (High-Level)

`bevy`: A data-driven game engine. Very popular and actively developed. A good choice for 2D and 3D games.
`Amethyst`: Another game engine, focusing on data-oriented design.
`ggez`: A simple 2D game framework. Good for beginners and smaller projects.
[`macroquad`][c-macroquad]⮳{{hi:macroquad}} : Easy to use framework for 2D games and interactive applications.

See also `winit`: A window creation and event handling library. Often used as a foundation for custom engines or when more control is needed.

## Graphics

`wgpu`: A cross-platform, safe, and portable GPU API. Often used with winit or game engines.
`rend3`: A 3D rendering engine built on top of wgpu.
`gfx-hal`: A low-level graphics API abstraction layer.
`image`: For image loading and manipulation.

## Audio

`cpal`: Cross-platform audio I/O.
`sdl2`: Can also be used for audio

## Input

[`winit`][c-winit]⮳{{hi:winit}} : Handles window events, including input.
`sdl2`: Can also be used for input

## Game Logic/State Management

Often handled directly or with ECS libraries. See below.

## Entity Component System (ECS) (For Data-Driven Design)

`bevy_ecs`: Bevy's built-in ECS.
`specs`: A popular and mature ECS library.
`hecs`: Another ECS implementation.

## Physics

`rapier`: A physics engine (2D and 3D).
`nphysics`: Another physics engine.

## Networking

`ggrs`: A P2P networking library for games.
`tokio`: (For asynchronous networking in general)
`mio`: (Lower-level networking)

## UI (User Interface)

`egui`: An immediate mode GUI library.
`iced`: A cross-platform GUI library focused on simplicity and type safety.
`conrod`: An older UI library.

## Math and Linear Algebra

`nalgebra`: A popular linear algebra library.
`glam`: Another linear algebra library, often used in game development.

## Other Useful Crates

`rand`: For random number generation.
`serde`: For serialization/deserialization (saving game state, loading assets, etc.).

## Asset Loading and Management

Engine-specific; but crates like `image` for image loading are often used.

## Serialization/Deserialization (for game state, assets, etc.)

`serde`: Widely used serialization framework.

## Scripting (Optional)

`rhai`: An embeddable scripting language.
`lua`: Lua bindings.

## Choosing the Right Crates

Simple 2D Games: `ggez`, [`macroquad`][c-macroquad]⮳{{hi:macroquad}} , or a combination of [`winit`][c-winit]⮳{{hi:winit}} , `pixels`, and `cpal`.
More Complex 2D/3D Games: `Bevy` or `Amethyst`.
Custom Engines or Low-Level Graphics: [`winit`][c-winit]⮳{{hi:winit}} , `wgpu`, `gfx-hal`.
Data-Driven Design: An ECS library like `bevy_ecs`, `specs`, or `hecs`.
Physics: `rapier` or `nphysics`.

</div>
