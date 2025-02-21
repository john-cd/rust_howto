# Multimedia: Audio

[![cat-multimedia::audio][cat-multimedia::audio-badge]][cat-multimedia::audio]{{hi:Audio}}

Record, output, or process audio.

{{#include audio.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 fix](https://github.com/john-cd/rust_howto/issues/941)

## Key Concepts

- Codecs: Algorithms for encoding and decoding audio.
- Sampling rate: The number of audio samples taken per second.
- Bit depth: The number of bits used to represent each sample.
- Channels: The number of audio channels (e.g., mono, stereo).
- DSP: Digital signal processing techniques.

## Audio

- `cpal`: Cross-platform audio input and output. A good general-purpose audio crate.
- `sdl2`: Can also be used for audio.
- `miniaudio`: A minimal audio library.

Audio Input/Output:

- `cpal`: Cross-platform audio input and output. A good general-purpose audio crate for most needs.
- `miniaudio`: A minimal audio library focused on simplicity.

Audio Processing:

- `rubberband`: Time-stretching and pitch-shifting library.
- `rust-dsp`: Digital signal processing (DSP) library.

Audio Formats and Codecs:

- `ogg`: For Ogg Vorbis files.
- `flac`: For FLAC files.
- `mp3`: For MP3 files (often requires external dependencies).
- `wav`: For WAV files.

Synthesis:

- `synth-rs`: A crate for audio synthesis.

Game Audio:

- `sdl2`: (Can also be used for audio in games).

Other Audio Libraries:

- `symphonia`: A comprehensive audio decoding library.
- `iced_audio`: Audio playback for the `iced` GUI framework.

## Choosing Crates

- General audio I/O: `cpal` is usually the best choice.
- Minimal audio: `miniaudio` might be suitable.
- Audio processing: `rubberband`, `rust-dsp`.
- Specific formats: Use the crates for the formats you need (e.g., `ogg`, `flac`, `wav`).

For most common audio tasks, `cpal` will be sufficient. If you need to work with specific audio formats, use the corresponding crates. For more advanced audio processing, explore crates like `rubberband` or `rust-dsp`. For game development, `sdl2` or game-specific audio libraries might be relevant.

</div>
