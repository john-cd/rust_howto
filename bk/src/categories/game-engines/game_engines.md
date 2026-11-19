<!-- TODO Reviewed ??-??-2026. -->

# Game Engines

For a playable Macroquad walkthrough with movement and scoring, see
[Collect Three Markers](../games/index.md#collect-markers). The examples here
compare how frameworks own window creation and the frame loop.

Choose a complete engine when you want an editor, asset workflow, or integrated runtime. A framework gives you a smaller starting point and leaves more decisions to your application. The version column shows the dependencies used by the examples in this book's `game_engines` crate; check each project's current release and platform guides before starting a new project.

## Compare Engines

| Engine | Version in examples | 2D / 3D | Editor | ECS | Web and mobile | Maturity and status |
| --- | --- | --- | --- | --- | --- | --- |
| [Bevy][c~bevy~docs]↗{{hi:Bevy}} | [0.20.1][c~bevy~crates.io] | Yes / yes | No integrated editor; editor prototypes are in progress. | Built in | WebAssembly supported; WebGPU is experimental. Mobile can work but requires platform setup. | Actively developed; [0.20.0 released in October 2026][bevy~release~notes]. |
| [Fyrox][c~fyrox~docs]↗{{hi:Fyrox}} | [1.0.1][c~fyrox~crates.io] | Yes / yes | Built-in scene editor and project manager. | No; scene and plugin model. | [WebAssembly and Android export are documented][fyrox~platforms~book]; iOS is not listed. | Active 1.0 series; see [repository activity][fyrox~activity~repo]. |
| [ggez][c~ggez~docs]↗{{hi:ggez}} | [0.10.0][c~ggez~crates.io] | Yes / basic 3D | No | External library | Windows, Linux, and macOS are officially supported; web and mobile are not. | Established 0.x framework; check [repository activity][ggez~activity~repo]. |
| [Macroquad][c~macroquad~docs]↗{{hi:Macroquad}} | [0.4.16][c~macroquad~crates.io] | Yes / limited 3D | No | External library | Windows, Linux, macOS, HTML5, Android, and iOS. | Active 0.4 series; see [repository activity][macroquad~activity~repo]. |

These options differ in scope and platform support. For individual game-development libraries such as math, audio, graphics, or physics crates, see [Game Development][p~game-development].

{{#include game_engines.incl.md}}

## Develop Games with `bevy` {#bevy}

[![bevy~website][c~bevy~website~badge]][c~bevy~website] {{#badges bevy}} {{hi:bevy}} {{hi:Engine}} {{hi:Game}} {{hi:Gamedev}} {{hi:Graphics}} {{#categories game-engines="Game engines",gui=GUI,rendering=Rendering,graphics=Graphics}}

Bevy is a data-driven engine and app framework with an integrated entity-component-system (ECS). It supports both 2D and 3D games, and its modular plugins let an app choose which engine services to include.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p game_engines --example game_engines`.

```rust,editable,noplayground
{{#include ../../../crates/cats/game_engines/examples/game_engines/bevy.rs:example}}
```

This example installs `MinimalPlugins`, spawns one entity with position
and velocity components at startup, and updates its position in an ECS
system. The test supplies a fixed time step so movement is reproducible.
`App::update` advances one schedule iteration. A complete desktop game
typically uses `DefaultPlugins` with the Cargo features for its window,
input, asset, and rendering systems.

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p game_engines --features bevy --example game_engines -- bevy
```

## Develop Games with `fyrox` {#fyrox}

[![fyrox~website][c~fyrox~website~badge]][c~fyrox~website] {{#badges fyrox}} {{hi:3d}} {{hi:Engine}} {{hi:Game}} {{hi:Gui}} {{hi:Sound}} {{#categories game-engines="Game engines",gui=GUI,rendering=Rendering,wasm=WebAssembly,graphics=Graphics}}

Fyrox is a 2D and 3D engine built around scenes, plugins, and scripts rather than an ECS. Its project manager generates a project structure with an editor and platform-specific executors; see the [Fyrox project workflow][fyrox~project~book]↗.

To start a project, install and run the template generator; it creates the game crate, an editor binary and an executor, plus the glue that registers your plugin:

```bash
cargo install fyrox-template
fyrox-template init --name my_game --style 3d
cd my_game
cargo run --package editor    # open the scene editor
cargo run --package executor  # run the game without the editor
```

The snippet shows the game plugin and a headless harness for its state. The engine calls `update` at a fixed rate with a `PluginContext` that gives access to scenes, the resource manager and the window. Keep game rules in plain methods so they can be unit-tested without a window or a GPU. Fyrox is a heavy dependency, so the book crate builds this example only with the `fyrox` Cargo feature, and on Linux it needs the ALSA, udev and windowing development packages (for example `libasound2-dev libudev-dev libx11-dev libxcursor-dev libxi-dev libxrandr-dev libgl1-mesa-dev libwayland-dev libxkbcommon-dev`).

The book runner calls the same tick method that the plugin callback uses
and prints one update. It exercises the counter and the compiled plugin
interface without starting the editor or executor.

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p game_engines --features fyrox --example game_engines -- fyrox
```

If `sccache` fails to launch `rustc` for Fyrox on Windows, clear the wrappers
in the PowerShell session used to run the example:

```powershell
$env:RUSTC_WRAPPER = ''
$env:CARGO_BUILD_RUSTC_WRAPPER = ''
```

```rust,editable,noplayground
{{#include ../../../crates/cats/game_engines/examples/game_engines/fyrox.rs:example}}
```

## Develop 2D Games with `ggez` {#ggez}

[![ggez~website][c~ggez~website~badge]][c~ggez~website] {{#badges ggez}} {{hi:Game}} {{hi:Graphics}} {{hi:2d}} {{hi:ggez}} {{hi:Engine}} {{#categories game-engines="Game engines"}}

ggez is a lightweight framework for 2D games with an event-handler model, drawing, audio, and resource loading. It also offers basic 3D drawing, but does not prescribe physics or an ECS. Its maintainers list Windows, Linux, and macOS as supported platforms; web and mobile targets are not officially supported ([platform details][ggez~platforms~repo]↗).

The example implements `EventHandler`. In `update`, move the ball using
elapsed seconds and reflect its velocity at a wall. In `draw`, clear the
frame, draw the ball at the current position, and finish the canvas. This
keeps the physics step callable without creating a window.

From a desktop at the repository root:

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p game_engines --features ggez --example game_engines -- ggez
```

```rust,editable,noplayground
{{#include ../../../crates/cats/game_engines/examples/game_engines/ggez.rs:example}}
```

## Develop 2D Games with `macroquad` {#macroquad}

{{#badges macroquad}}

Macroquad is an immediate-mode framework designed to make a simple 2D game loop easy to write. It supports desktop, browser, Android, and iOS builds, and also provides basic 3D drawing ([platform details][macroquad~platforms~repo]↗).

Macroquad gives your application an async frame loop. The example updates
the ball using the current frame duration, draws it, then yields through
`next_frame().await` so Macroquad can present the frame and handle events.
Compare this sequence with ggez's separate callbacks.

```console
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p game_engines --example game_engines -- macroquad
```

For either bouncing-ball example, setting `RUST_HOWTO_SMOKE_TEST=1` in the
launching shell bounds the run to a few presented frames. That mode still
needs a desktop window.

```rust,editable,noplayground
{{#include ../../../crates/cats/game_engines/examples/game_engines/macroquad.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Game development libraries][p~game-development].
- [[rendering | Rendering]] and [physics engines](../simulation/physics_engines.md).
- [[multimedia_audio | Audio]] and [window creation](https://github.com/john-cd/rust_howto/blob/develop/later/src/categories/gui/window_creation.md).
- [[games | Games]] made with Rust.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
