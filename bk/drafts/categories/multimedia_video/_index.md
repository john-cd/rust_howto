# Multimedia - Video

[![cat-multimedia::video][cat-multimedia::video-badge]][cat-multimedia::video]{{hi:Video}}

Record, output, or process video.

## Video

{{#include video.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 write](https://github.com/john-cd/rust_howto/issues/943)

- Codecs: Algorithms for encoding and decoding video.
- Formats: How encoded video data is structured (e.g., MP4, MKV).
- Transcoding: Converting video from one format to another.

## Choosing Crates

- General video decoding, encoding, manipulation: [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}} (powerful, but complex).
- AV1 encoding: [`rav1e`][c-rav1e]⮳{{hi:rav1e}}.
- H.264 encoding: [`x264`][c-x264]⮳{{hi:x264}}.
- H.265 encoding: [`x265`][c-x265]⮳{{hi:x265}}.
- VP8/VP9 encoding: [`vpx`][c-vpx]⮳{{hi:vpx}}.

For most video-related tasks, FFmpeg ([`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}) is the most comprehensive option, but it can be more complex to work with. If you're focused on specific encoding tasks (e.g., AV1, H.264), the dedicated encoder crates ([`rav1e`][c-rav1e]⮳{{hi:rav1e}}, [`x264`][c-x264]⮳{{hi:x264}}) might be a better choice. For video streaming, you'll need to combine video processing with networking libraries. Building a video player is a complex project that would require integrating multiple crates.

## Codecs and Formats

Many crates exist for specific codecs (e.g., MP3, AAC, Vorbis). FFmpeg bindings ([`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}) provide access to a very wide range of codecs.

## Video Decoding

- [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}: Bindings to FFmpeg. FFmpeg is a very powerful and comprehensive library for working with video (and audio). It supports a wide range of formats and codecs. However, it can be complex to use due to its extensive API.

## Video Encoding

- [`rav1e`][c-rav1e]⮳{{hi:rav1e}}: AV1 encoder. A good choice for modern video encoding.
- [`x264`][c-x264]⮳{{hi:x264}}: H.264 encoder. A widely used and mature encoder.
- [`x265`][c-x265]⮳{{hi:x265}}: H.265 (HEVC) encoder.
- [`vpx`][c-vpx]⮳{{hi:vpx}}: VP8/VP9 encoder.
- [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}: Also used for encoding.

## Video Manipulation

- [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}: FFmpeg is also capable of video manipulation (resizing, cropping, filtering, etc.).

## Video Streaming

For video streaming, you'll often need to combine video encoding/decoding with networking libraries. There aren't specific "streaming crates," but crates like [`tokio`][c-tokio]⮳{{hi:tokio}} for asynchronous networking are often used.

## Video Players (Less Common in Rust)

Building a full-fledged video player in Rust is a complex undertaking. You'd likely combine video decoding with graphics libraries ([`wgpu`][c-wgpu]⮳{{hi:wgpu}}, [`pixels`][c-pixels]⮳{{hi:pixels}}, etc.) and audio libraries ([`cpal`][c-cpal]⮳{{hi:cpal}}).

## Image Processing

[[images | Images]]
[[multimedia_images | Multimedia Images]]

- [`image`][c-image]⮳{{hi:image}}: A widely used crate for image loading and manipulation.

## Graphics (Related to Multimedia)

- [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: A cross-platform GPU API. Used for rendering video or other graphical elements.
- [`pixels`][c-pixels]⮳{{hi:pixels}}: For working with pixel buffers (often used with images or video frames).

[[graphics | Graphics]]
[[graphics_apis | Graphics APIs]]
[[rendering_graphics-api | Rendering Graphics API]]

</div>
