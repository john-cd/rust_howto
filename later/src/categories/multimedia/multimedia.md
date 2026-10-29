<!-- TODO Reviewed ??-??-2026 -->

# Multimedia Workflows

{{#include multimedia.incl.md}}

This chapter connects media types: play a generated sound, extract audio from a video, and turn a video frame into an image. The [audio](../multimedia_audio/index.md), [image](../multimedia_images/images.md), [encoding](../multimedia_encoding/encoding.md), and [video](../multimedia_video/index.md) chapters own format-specific APIs and processing recipes.

## Play a Generated Sound {#multimedia}

The `rodio` example generates a 440 Hz sine wave and plays it for two seconds through the default output device. No input file or network service is needed. Its test is ignored because opening a real audio device is unsuitable for a headless test runner; errors from device initialization are propagated.

```rust,editable
{{#include ../../../crates/cats/multimedia/examples/multimedia/multimedia1.rs:example}}
```

From `later/crates`, run `cargo run -p multimedia --example multimedia` on a machine with an audio output device.

## Extract Audio from a Video

[FFmpeg][ffmpeg~manual] works at the boundary between video containers and audio/image tools. Install an FFmpeg build with the required decoders, then extract the first audio stream into a WAV file:

```sh
ffmpeg -i input.mp4 -map 0:a:0 -vn audio.wav
```

`-map 0:a:0` selects the first audio stream of the first input; the command fails if it is absent. `-vn` excludes video. WAV output decodes audio to PCM rather than copying the compressed stream. For lossless extraction of an AAC bitstream into an M4A container, use `-c:a copy audio.m4a` instead, provided the selected input codec is compatible with that container.

## Create a Thumbnail from a Video

Select a timestamp and decode one video frame into a PNG image:

```sh
ffmpeg -ss 1 -i input.mp4 -map 0:v:0 -frames:v 1 thumbnail.png
```

Seeking before `-i` allows the demuxer to seek near the requested timestamp; decoder behavior and keyframes affect the result. Choose a timestamp inside the clip and inspect the output. FFmpeg asks before replacing an existing file; use distinct output names when preserving originals.

Once extracted, process the image with the [Images](../multimedia_images/images.md) recipes. For application integration, invoke FFmpeg with separate process arguments and check its exit status before reading the output; do not construct a shell command from untrusted filenames.

## Related Topics {#related-topics .skip}

- [Audio playback and processing](../multimedia_audio/index.md).
- [Media codecs and container metadata](../multimedia_encoding/encoding.md).
- [Image processing](../multimedia_images/images.md).
- [Video processing](../multimedia_video/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
