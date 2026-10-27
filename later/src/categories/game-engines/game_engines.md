<!-- TODO Reviewed ??-??-2026 -->

# Game Engines

Choose a complete engine when you want an editor, asset workflow, or integrated runtime. A framework gives you a smaller starting point and leaves more decisions to your application. The version column shows the dependencies used by the examples in this book's `game_engines` crate; check each project's current release and platform guides before starting a new project.

## Compare Engines

| Engine                                         | Version in examples             | 2D / 3D          | Editor                                                   | ECS                         | Web and mobile                                                                              | Maturity and status                                                         |
| ---------------------------------------------- | ------------------------------- | ---------------- | -------------------------------------------------------- | --------------------------- | ------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| [Bevy][c~bevy~docs]↗{{hi:Bevy}}                | [0.20.0][c~bevy~crates.io]      | Yes / yes        | No integrated editor; editor prototypes are in progress. | Built in                    | WebAssembly supported; WebGPU is experimental. Mobile can work but requires platform setup. | Actively developed; [0.20.0 released in October 2026][bevy~release~notes].  |
| [Fyrox][c~fyrox~docs]↗{{hi:Fyrox}}             | [1.0.1][c~fyrox~crates.io]      | Yes / yes        | Built-in scene editor and project manager.               | No; scene and plugin model. | [WebAssembly and Android export are documented][fyrox~platforms~book]; iOS is not listed.   | Active 1.0 series; see [repository activity][fyrox~activity~repo].          |
| [ggez][c~ggez~docs]↗{{hi:ggez}}                | [0.10.0][c~ggez~crates.io]      | Yes / basic 3D   | No                                                       | External library            | Windows, Linux, and macOS are officially supported; web and mobile are not.                 | Established 0.x framework; check [repository activity][ggez~activity~repo]. |
| [Macroquad][c~macroquad~docs]↗{{hi:Macroquad}} | [0.4.16][c~macroquad~crates.io] | Yes / limited 3D | No                                                       | External library            | Windows, Linux, macOS, HTML5, Android, and iOS.                                             | Active 0.4 series; see [repository activity][macroquad~activity~repo].      |

These options differ in scope and platform support. For individual game-development libraries such as math, audio, graphics, or physics crates, see [Game Development][p~game-development].

{{#include game_engines.incl.md}}

## Develop Games with `bevy` {#bevy}

[![bevy~website][c~bevy~website~badge]][c~bevy~website] [![bevy][c~bevy~docs~badge]][c~bevy~docs] [![bevy~crates.io][c~bevy~crates.io~badge]][c~bevy~crates.io] [![bevy~repo][c~bevy~repo~badge]][c~bevy~repo] [![bevy~lib.rs][c~bevy~lib.rs~badge]][c~bevy~lib.rs]{{hi:bevy}}{{hi:bevy}}{{hi:Engine}}{{hi:Game}}{{hi:Gamedev}}{{hi:Graphics}} [![cat~game-engines][cat~game-engines~badge]][cat~game-engines]{{hi:Game engines}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}} [![cat~rendering][cat~rendering~badge]][cat~rendering]{{hi:Rendering}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

Bevy is a data-driven engine and app framework with an integrated entity-component-system (ECS). It supports both 2D and 3D games, and its modular plugins let an app choose which engine services to include.

```rust,editable
{{#include ../../../crates/cats/game_engines/examples/game_engines/bevy.rs:example}}
```

This headless example uses MinimalPlugins and no renderer. A full game typically uses DefaultPlugins, which adds the windowing, input, asset, and rendering plugins enabled by its Cargo features.

## Develop Games with `fyrox` {#fyrox}

[![fyrox~website][c~fyrox~website~badge]][c~fyrox~website] [![fyrox][c~fyrox~docs~badge]][c~fyrox~docs] [![fyrox~crates.io][c~fyrox~crates.io~badge]][c~fyrox~crates.io] [![fyrox~repo][c~fyrox~repo~badge]][c~fyrox~repo] [![fyrox~lib.rs][c~fyrox~lib.rs~badge]][c~fyrox~lib.rs]{{hi:fyrox}}{{hi:3d}}{{hi:Engine}}{{hi:Game}}{{hi:Gui}}{{hi:Sound}} [![cat~game-engines][cat~game-engines~badge]][cat~game-engines]{{hi:Game engines}} [![cat~gui][cat~gui~badge]][cat~gui]{{hi:GUI}} [![cat~rendering][cat~rendering~badge]][cat~rendering]{{hi:Rendering}} [![cat~wasm][cat~wasm~badge]][cat~wasm]{{hi:WebAssembly}} [![cat~graphics][cat~graphics~badge]][cat~graphics]{{hi:Graphics}}

Fyrox is a 2D and 3D engine built around scenes, plugins, and scripts rather than an ECS. Its project manager generates a project structure with an editor and platform-specific executors; see the [Fyrox project workflow][fyrox~project~book]↗.

```rust,editable
{{#include ../../../crates/cats/game_engines/examples/game_engines/fyrox.rs:example}}
```

## Develop 2D Games with `ggez` {#ggez}

[![ggez~website][c~ggez~website~badge]][c~ggez~website] [![ggez][c~ggez~docs~badge]][c~ggez~docs] [![ggez~crates.io][c~ggez~crates.io~badge]][c~ggez~crates.io] [![ggez~repo][c~ggez~repo~badge]][c~ggez~repo] [![ggez~lib.rs][c~ggez~lib.rs~badge]][c~ggez~lib.rs]{{hi:ggez}}{{hi:Game}}{{hi:Graphics}}{{hi:2d}}{{hi:ggez}}{{hi:Engine}} [![cat~game-engines][cat~game-engines~badge]][cat~game-engines]{{hi:Game engines}}

ggez is a lightweight framework for 2D games with an event-handler model, drawing, audio, and resource loading. It also offers basic 3D drawing, but does not prescribe physics or an ECS. Its maintainers list Windows, Linux, and macOS as supported platforms; web and mobile targets are not officially supported ([platform details][ggez~platforms~repo]↗).

Run the example on Windows or Linux with:

```console
cargo run --locked -p game_engines --example game_engines --features ggez -- ggez
```

```rust,editable
{{#include ../../../crates/cats/game_engines/examples/game_engines/ggez.rs:example}}
```

## Develop 2D Games with `macroquad` {#macroquad}

[![macroquad][c~macroquad~docs~badge]][c~macroquad~docs] [![macroquad~crates.io][c~macroquad~crates.io~badge]][c~macroquad~crates.io] [![macroquad~repo][c~macroquad~repo~badge]][c~macroquad~repo] [![macroquad~lib.rs][c~macroquad~lib.rs~badge]][c~macroquad~lib.rs]{{hi:macroquad}}

Macroquad is an immediate-mode framework designed to make a simple 2D game loop easy to write. It supports desktop, browser, Android, and iOS builds, and also provides basic 3D drawing ([platform details][macroquad~platforms~repo]↗).

```rust,editable
{{#include ../../../crates/cats/game_engines/examples/game_engines/macroquad.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Game development libraries][p~game-development].
- [[rendering | Rendering]] and [physics engines](../simulation/physics_engines.md).
- [[multimedia_audio | Audio]] and [window creation](../gui/window_creation.md).
- [[games | Games]] made with Rust.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
