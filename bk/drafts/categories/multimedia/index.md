# Multimedia

[![cat-multimedia][cat-multimedia-badge]][cat-multimedia]{{hi:Multimedia}}

Audio, video, and image processing or rendering engines.

While there aren't dominant, all-encompassing multimedia frameworks in Rust yet, some game engines or graphics libraries provide some multimedia-related functionality.

For most multimedia tasks, you'll need to combine several crates. For example, you might use [`image`][c-image]⮳{{hi:image}} to load images, [`cpal`][c-cpal]⮳{{hi:cpal}} to play audio, and [`wgpu`][c-wgpu]⮳{{hi:wgpu}} to render video. FFmpeg bindings are very powerful, but they can be more complex to work with.

{{#include multimedia.incl.md}}

## Audio Processing

- Audio Decoding and Encoding
- Audio Effects and Filters
- Audio Streaming
- Audio Synthesis
- Audio Analysis

- General audio: [`cpal`][c-cpal]⮳{{hi:cpal}} is a good starting point.
Link

## Image Processing

- Image Decoding and Encoding
- Image Filters and Transformations
- Image Analysis
- Image Compression
- Image Rendering
- Image processing: [`image`][c-image]⮳{{hi:image}}.

## Video Processing

- Video Decoding and Encoding
- Video Effects and Filters
- Video Streaming
- Video Playback
- Video Analysis

- Video encoding: [`rav1e`][c-rav1e]⮳{{hi:rav1e}}, [`x264`][c-x264]⮳{{hi:x264}}, or [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}} (if you need many codecs).
Link

### File Formats and Codecs

- Audio Formats (MP3, WAV, OGG)
- Video Formats (MP4, AVI, MKV)
- Image Formats (JPEG, PNG, GIF)
- Codecs (H.264, VP9, Opus)

## Related Topics

### Low-level graphics

[`wgpu`][c-wgpu]⮳{{hi:wgpu}} (if you need to render video or perform GPU-accelerated processing).

### Networking and Streaming

- Real-time Streaming Protocols (RTSP, WebRTC)
- Media Servers
- Live Streaming
- Peer-to-Peer Streaming
- Network Protocols (HTTP, RTP)

### Cross-Platform Development

- WebAssembly (Wasm)
- Mobile Development (Android, iOS)
- Desktop Applications (Windows, macOS, Linux)
- Embedded Systems

### Performance Optimization

- Memory Management
- Concurrency and Parallelism
- Low-Level System Programming
- Profiling and Benchmarking
- Hardware Acceleration

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/940)
Link to related pages
</div>
