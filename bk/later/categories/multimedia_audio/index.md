# Multimedia: Audio

[![cat~multimedia::audio][cat~multimedia::audio~badge]][cat~multimedia::audio]{{hi:Audio}}

Record, output, or process audio.

For most common audio tasks, [`cpal`][c~cpal~docs]↗{{hi:cpal}} will be sufficient. If you need to work with specific audio formats, use the corresponding crates (e.g., [`ogg`][c~ogg~docs]↗{{hi:ogg}}, [`flac`][c~flac~docs]↗{{hi:flac}}, [`wav`][c~wav~docs]↗{{hi:wav}}). For game development, [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}} or game-specific audio libraries might be relevant.

## General Audio

- [`cpal`][c~cpal~docs]↗{{hi:cpal}}: Cross-platform audio input and output. A good general-purpose audio crate.
- [`miniaudio`][c~miniaudio~docs]↗{{hi:miniaudio}}: A minimal audio library.
- [`sdl2`][c~sdl2~docs]↗{{hi:sdl2}}: Can also be used for audio.

See also:

- [`symphonia`][c~symphonia~docs]↗{{hi:symphonia}}: A comprehensive audio decoding library.
- [`iced_audio`][c~iced_audio~docs]↗{{hi:iced_audio}}: Audio playback for the [`iced`][c~iced~docs]↗{{hi:iced}} GUI framework.

## Specific Audio Codecs

Use:

- [`ogg`][c~ogg~docs]↗{{hi:ogg}} for Ogg Vorbis files.
- [`flac`][c~flac~docs]↗{{hi:flac}} for FLAC files.
- [`mp3`][c~mp3~docs]↗{{hi:mp3}} for MP3 files (often requires external dependencies).
- [`wav`][c~wav~docs]↗{{hi:wav}} for WAV files.

## Audio Effects, Filters, and Compression

FIXME

## Audio Streaming

FIXME

## Audio Synthesis

Use [`synth-rs`][c~synth~docs]↗{{hi:synth-rs}} for audio synthesis.

## Audio Analysis

## Code Examples

{{#include audio.incl.md}}

## Related Topics

- [[multimedia | Multimedia]].
- [[multimedia_encoding | Multimedia: Encoding]].
- [[multimedia_video | Multimedia: Video]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fix](https://github.com/john-cd/rust_howto/issues/941)
review in depth
synth is old
</div>
