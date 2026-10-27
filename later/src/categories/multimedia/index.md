<!-- TODO Reviewed ??-??-2026 -->

# Multimedia

[![cat~multimedia][cat~multimedia~badge]][cat~multimedia]{{hi:Multimedia}}

Audio, video, and image processing or rendering engines.

While there aren't dominant, all-encompassing multimedia frameworks in Rust yet, game engines or graphics libraries provide multimedia-related functionality. For most multimedia tasks, you'll need to combine several crates. For example, you might use [`image`][c~image~docs]↗{{hi:image}} to load images, [`cpal`][c~cpal~docs]↗{{hi:cpal}} to play audio, and [`wgpu`][c~wgpu~docs]↗{{hi:wgpu}} to render video.

{{#include multimedia.incl.md}}

## Sub-categories

- [[multimedia_audio | Audio]]: playback, decoding, resampling, and WAV files.
- [[multimedia_images | Images]]: loading, transforming, and encoding images.
- [[multimedia_video | Video]]: FFmpeg, GStreamer, and frame analysis.
- [[multimedia_encoding | Encoding]]: codecs and container formats.

Real-time streaming (RTSP, WebRTC, RTP) is covered in [Streaming Media](streaming.md). For transport protocols and networking, see [[network-programming | Network Programming]].

## Related Topics

- [[graphics | Graphics]], [[rendering | Rendering]], [[rendering_data-formats | Rendering Data Formats]], [[rendering_engines | Rendering Engines]], [[rendering_graphics-api | Rendering: Graphics API]].
- [[wasm | WASM]], [[gui | GUI]] and [[embedded | Embedded Systems]] for other targets.
- [[memory-management | Memory Management]], [[concurrency | Concurrency]], [[development-tools_profiling | Profiling]], [[benchmarking | Benchmarking]] and [[hardware-support | Hardware]] acceleration for performance.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
