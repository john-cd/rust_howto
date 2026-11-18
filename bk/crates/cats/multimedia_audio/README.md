<!-- TODO Reviewed ??-??-2026. -->

# Multimedia Audio

This crate contains code examples for the **Multimedia Audio** category on crates.io. It demonstrates integration with key ecosystem crates including `cpal`, `hound`, `hrtf`, `midir`, `rodio`, `rubato`.

## Examples Covered

- **Audio** (`examples/audio/`): Demonstrates concepts like `audio`, `hrtf`, `midi_ports`, `resampling`, `rodio_file`, `rodio_source`, `tone`, `wav_write_read`.
- **Wav** (`examples/wav/`): Demonstrates concepts like `wav`.

## Usage

To check or run examples in this crate:

```console
cargo check --package multimedia_audio --examples
cargo test --package multimedia_audio
```

Code examples correspond to chapters in `bk/src/categories/multimedia_audio` or related book sections.

## Offline Spectrum Analysis

The device-free `spectrum` example analyzes a generated mono tone with RustFFT:

```sh
just cargo test --manifest-path bk/crates/Cargo.toml --locked -p multimedia_audio --example spectrum
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p multimedia_audio --example spectrum
```

See `bk/src/categories/multimedia_audio/analysis.md` for window/scale conventions
and the distinction between an analysis worker and a realtime audio callback.

## MIDI Loopback

List the native MIDI ports from the repository root:

```sh
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p multimedia_audio --example midi_loopback -- --list
```

Use the two exact endpoint names from a dedicated loopback route to run the
round trip. See the MIDI section in
`bk/src/categories/multimedia_audio/audio.md` for expected bytes and prerequisites.
