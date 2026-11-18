<!-- TODO Reviewed ??-??-2026. -->

# Multimedia

[![cat~multimedia][cat~multimedia~badge]][cat~multimedia]{{hi:Multimedia}}

Audio, video, and image processing or rendering engines.

While there aren't dominant, all-encompassing multimedia frameworks in Rust yet, game engines or graphics libraries provide multimedia-related functionality. For most multimedia tasks, you'll need to combine several crates. For example, you might use [`image`][c~image~docs]↗{{hi:image}} to load images, [`cpal`][c~cpal~docs]↗{{hi:cpal}} to play audio, and [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} to render video.

{{#include multimedia.incl.md}}

## Sub-categories

- [Audio](../multimedia_audio/index.md): playback, decoding, resampling, and WAV files.
- [Images](https://github.com/john-cd/rust_howto/blob/ceaf4b050e21e3981d121bb1414fc26b01e16a4b/bk/src/categories/multimedia_images/index.md): loading, transforming, and encoding images.
- [Video](../multimedia_video/index.md): FFmpeg, GStreamer, and frame analysis.
- [Encoding](../multimedia_encoding/index.md): codecs and container formats.

Real-time streaming (RTSP, WebRTC, RTP) is covered in [Streaming Media](streaming.md). For transport protocols and networking, see [[network-programming | Network Programming]].

## Related Topics

- [[graphics | Graphics]], [[rendering | Rendering]], [[rendering_data-formats | Rendering Data Formats]], [Rendering Engines](../rendering_engine/index.md), [[rendering_graphics-api | Rendering: Graphics API]].
- [[wasm | WASM]], [[gui | GUI]] and [[embedded | Embedded Systems]] for other targets.
- [[memory-management | Memory Management]], [[concurrency | Concurrency]], [Development Tools - Profiling](https://github.com/john-cd/rust_howto/blob/86dcf0197f8d6810877a6021ffedcf84aa9230c8/bk/src/categories/development-tools_profiling/index.md), [Benchmarking](https://github.com/john-cd/rust_howto/blob/86dcf0197f8d6810877a6021ffedcf84aa9230c8/bk/src/categories/development-tools_profiling/benchmarking.md), and [[hardware-support | Hardware]] acceleration for performance.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
