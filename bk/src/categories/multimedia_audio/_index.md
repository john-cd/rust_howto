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

- [`cpal`][c-cpal]⮳{{hi:cpal}}: Cross-platform audio input and output. A good general-purpose audio crate.
- [`sdl2`][c-sdl2]⮳{{hi:sdl2}}: Can also be used for audio.
- [`miniaudio`][c-miniaudio]⮳{{hi:miniaudio}}: A minimal audio library.

Audio Input/Output:

- [`cpal`][c-cpal]⮳{{hi:cpal}}: Cross-platform audio input and output. A good general-purpose audio crate for most needs.
- [`miniaudio`][c-miniaudio]⮳{{hi:miniaudio}}: A minimal audio library focused on simplicity.

Audio Processing:

- `rubberband`: Time-stretching and pitch-shifting library.
- `rust-dsp`: Digital signal processing (DSP) library.

Audio Formats and Codecs:

- [`ogg`][c-ogg]⮳{{hi:ogg}}: For Ogg Vorbis files.
- [`flac`][c-flac]⮳{{hi:flac}}: For FLAC files.
- `mp3`: For MP3 files (often requires external dependencies).
- [`wav`][c-wav]⮳{{hi:wav}}: For WAV files.

Synthesis:

- [`synth-rs`][c-synth]⮳{{hi:synth-rs}}: A crate for audio synthesis.

Game Audio:

- [`sdl2`][c-sdl2]⮳{{hi:sdl2}}: (Can also be used for audio in games).

Other Audio Libraries:

- [`symphonia`][c-symphonia]⮳{{hi:symphonia}}: A comprehensive audio decoding library.
- [`iced_audio`][c-iced_audio]⮳{{hi:iced_audio}}: Audio playback for the [`iced`][c-iced]⮳{{hi:iced}} GUI framework.

## Choosing Crates

- General audio I/O: [`cpal`][c-cpal]⮳{{hi:cpal}} is usually the best choice.
- Minimal audio: [`miniaudio`][c-miniaudio]⮳{{hi:miniaudio}} might be suitable.
- Audio processing: `rubberband`, `rust-dsp`.
- Specific formats: Use the crates for the formats you need (e.g., [`ogg`][c-ogg]⮳{{hi:ogg}}, [`flac`][c-flac]⮳{{hi:flac}}, [`wav`][c-wav]⮳{{hi:wav}}).

For most common audio tasks, [`cpal`][c-cpal]⮳{{hi:cpal}} will be sufficient. If you need to work with specific audio formats, use the corresponding crates. For more advanced audio processing, explore crates like `rubberband` or `rust-dsp`. For game development, [`sdl2`][c-sdl2]⮳{{hi:sdl2}} or game-specific audio libraries might be relevant.

</div>
