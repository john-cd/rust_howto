# Multimedia

[![cat-multimedia][cat-multimedia-badge]][cat-multimedia]{{hi:Multimedia}}

Audio, video, and image processing or rendering engines.

{{#include multimedia.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 write](https://github.com/john-cd/rust_howto/issues/940)

## Key Concepts

- Codecs: Algorithms for encoding and decoding audio and video.
- Formats: How audio and video data is structured (e.g., MP4, AVI).
- Sampling rate: The number of audio samples taken per second.
- Bitrate: The amount of data used to represent audio or video.
- Frames: Individual images in a video.
- Synchronization: Keeping audio and video in sync.

## Audio

Link

## Video

Link

## Multimedia Frameworks

While there aren't dominant, all-encompassing multimedia frameworks in Rust yet, some game engines or graphics libraries might provide some multimedia-related functionality.

## Choosing Crates

- General audio: `cpal` is a good starting point.
- Video encoding: `rav1e`, `x264`, or `ffmpeg-next` (if you need many codecs).
- Image processing: `image`.
- Low-level graphics: `wgpu` (if you need to render video or perform GPU-accelerated processing).

For most multimedia tasks, you'll need to combine several crates. For example, you might use `image` to load images, `cpal` to play audio, and `wgpu` to render video.  FFmpeg bindings are very powerful, but they can be more complex to work with.

</div>
