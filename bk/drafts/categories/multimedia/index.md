# Multimedia

[![cat-multimedia][cat-multimedia-badge]][cat-multimedia]{{hi:Multimedia}}

Audio, video, and image processing or rendering engines.

While there aren't dominant, all-encompassing multimedia frameworks in Rust yet, game engines or graphics libraries provide multimedia-related functionality. For most multimedia tasks, you'll need to combine several crates. For example, you might use [`image`][c-image]⮳{{hi:image}} to load images, [`cpal`][c-cpal]⮳{{hi:cpal}} to play audio, and [`wgpu`][c-wgpu]⮳{{hi:wgpu}} to render video.

{{#include multimedia.incl.md}}

## Audio Processing

See [[multimedia_audio | Multimedia: Audio]].

## Image Processing

See [[multimedia_images | Multimedia: Images]].

## Video Processing

See [[multimedia_video | Multimedia: Video]].

## Encoding

See [[multimedia_encoding | Multimedia: Encoding]].

## Related Topics

### Graphics and Rendering

- [[graphics | Graphics]].
- [[rendering | Rendering]].
- [[rendering_data-formats | Rendering Data Formats]].
- [[rendering_engines | Rendering Engines]].
- [[rendering_graphics-api | Rendering: Graphics API]].

### Networking and Streaming

- Real-time Streaming Protocols (RTSP, WebRTC).
- Media Servers.
- Live Streaming.
- Peer-to-Peer Streaming.
- Network Protocols (HTTP, RTP).

See [[network-programming | Network Programming]].

### Cross-Platform Development

- WebAssembly - see [[wasm | WASM]].
- Mobile Development (Android, iOS).
- Desktop Applications (Windows, macOS, Linux) - see [[gui | GUI]].
- [[embedded | Embedded Systems]].

### Performance Optimization

- [[memory-management | Memory Management]].
- [[concurrency | Concurrency]] and Parallelism.
- Low-Level System Programming.
- [[development-tools_profiling | Profiling]] and [[benchmarking | Benchmarking]].
- [[hardware-support | Hardware]] Acceleration.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/940)
cover areas without links above
</div>
