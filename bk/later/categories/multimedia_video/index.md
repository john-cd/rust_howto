# Multimedia - Video

[![cat-multimedia::video][cat-multimedia::video-badge]][cat-multimedia::video]{{hi:Video}}

Record, output, or process video.

For most video-related tasks, FFmpeg ([`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}) is the most comprehensive option, but it can be more complex to work with. If you're focused on specific encoding tasks (e.g., AV1, H.264), the dedicated encoder crates ([`rav1e`][c-rav1e]⮳{{hi:rav1e}}, [`x264`][c-x264]⮳{{hi:x264}}) might be a better choice. For video streaming, you'll need to combine video processing with [[network-programming | networking]] libraries.

{{#include video.incl.md}}

## Codecs and Formats

Many crates exist for specific codecs:

- [`rav1e`][c-rav1e]⮳{{hi:rav1e}}: AV1 encoder. A good choice for modern video encoding.
- [`x264`][c-x264]⮳{{hi:x264}}: H.264 encoder. A widely used and mature encoder.
- [`x265`][c-x265]⮳{{hi:x265}}: H.265 (HEVC) encoder.
- [`vpx`][c-vpx]⮳{{hi:vpx}}: VP8/VP9 encoder.

## General Video Decoding, Encoding, and Manipulation

FFmpeg bindings ([`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}) is a very powerful and comprehensive library for working with video (and audio). It supports a wide range of formats and codecs. FFmpeg is also capable of video manipulation (resizing, cropping, filtering, etc.). However, it can be complex to use due to its extensive API. This crate is currently in maintenance mode.

## Video Streaming

For video streaming, you'll often need to combine video encoding/decoding with [[network-programming | networking]] libraries. Crates like [`tokio`][c-tokio]⮳{{hi:tokio}} for [[asynchronous | asynchronous]] networking are often used.

## Video Playback

Video playback crates are less common in Rust.

Building a full-fledged video player in Rust is a complex undertaking. You'd likely combine video decoding with [[graphics | graphics]]  libraries ([`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`pixels`][c-pixels]⮳{{hi:pixels}}, etc.) and [[multimedia_audio | audio]] libraries (e.g. [`cpal`][c-cpal]⮳{{hi:cpal}}).

## Related Topics

- [[graphics | Graphics]].
- Image Processing: see [[images | Images]] and [[multimedia_images | Multimedia: Images]].
- [[multimedia | Multimedia]].
- [[multimedia_images | Multimedia Images]].
- [[rendering | Rendering]].
- Transcoding (converting video from one format to another) - see [[multimedia_encoding | Multimedia: Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/943)
review `gstreamer`
Cover
- Video Effects and Filters.
- Video Analysis.
</div>
