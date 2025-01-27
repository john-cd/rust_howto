// ANCHOR: example

// Specialized window creation library targetting windows
// to be embedded in other applications (e.g. DAW plugins)

// A low-level windowing system geared towards making audio plugin UIs.

// `baseview` abstracts the platform-specific windowing APIs (winapi, cocoa,
// xcb) into a platform-independent API, but otherwise gets out of your way so
// you can write plugin UIs.

// Requirements:
// sudo apt-get install libx11-dev libxcb1-dev libx11-xcb-dev libgl1-mesa-dev

pub fn main() -> anyhow::Result<()> {
    Ok(())
}
// ANCHOR_END: example

// [ P2 write; add to markdown; https://github.com/RustAudio/baseview](https://github.com/john-cd/rust_howto/issues/1056)
