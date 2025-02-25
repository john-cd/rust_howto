# Multimedia - Encoding

[![cat-multimedia::encoding][cat-multimedia::encoding-badge]][cat-multimedia::encoding]{{hi:Multimedia encoding}}

Encode or decode binary data in multimedia formats.

{{#include encoding.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[multimedia_encoding/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/418)

Codecs: Algorithms for encoding and decoding multimedia data.

## Video Encoding

- [`rav1e`][c-rav1e]⮳{{hi:rav1e}}: AV1 encoder. A good choice for modern video encoding.
- [`x264`][c-x264]⮳{{hi:x264}}: H.264 encoder. A widely used and mature encoder.
- [`x265`][c-x265]⮳{{hi:x265}}: H.265 (HEVC) encoder.
- [`vpx`][c-vpx]⮳{{hi:vpx}}: VP8/VP9 encoder.
- [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}: Bindings to FFmpeg. Provides access to a vast range of encoders, but can be more complex to use.

## Audio Encoding

Many crates exist for specific audio codecs. FFmpeg bindings ([`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}) provide access to many audio encoders.

## Image Encoding

Link to:

- [`image`][c-image]⮳{{hi:image}}: While primarily for image loading and manipulation, [`image`][c-image]⮳{{hi:image}} also supports encoding to some formats (e.g., PNG, JPEG).

## Transcoding (Combining Encoding of Different Media Types)

- [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}: FFmpeg is a powerful tool for transcoding (converting from one format to another).

## Choosing Crates

- AV1 video: [`rav1e`][c-rav1e]⮳{{hi:rav1e}}.
- H.264 video: [`x264`][c-x264]⮳{{hi:x264}}.
- H.265 video: [`x265`][c-x265]⮳{{hi:x265}}.
- VP8/VP9 video: [`vpx`][c-vpx]⮳{{hi:vpx}}.
- Wide range of codecs (video and audio): [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}.
- Image encoding: [`image`][c-image]⮳{{hi:image}} (for supported formats).

For most video encoding tasks, choosing the appropriate codec ([`rav1e`][c-rav1e]⮳{{hi:rav1e}}, [`x264`][c-x264]⮳{{hi:x264}}, [`x265`][c-x265]⮳{{hi:x265}}, [`vpx`][c-vpx]⮳{{hi:vpx}}) is the first step. If you need to work with a wide range of codecs or perform transcoding, [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}} is a powerful but more complex option. For image encoding, the [`image`][c-image]⮳{{hi:image}} crate is usually sufficient. For audio, you'll need to find crates that support the specific audio codecs you need, or use [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}}.

</div>
