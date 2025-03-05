# Multimedia - Encoding

[![cat-multimedia::encoding][cat-multimedia::encoding-badge]][cat-multimedia::encoding]{{hi:Multimedia encoding}}

{{#include encoding.incl.md}}

Encode or decode binary data in multimedia formats.

## Video Encoding

For most video encoding tasks, choosing the appropriate codec ([`rav1e`][c-rav1e]⮳{{hi:rav1e}}, [`x264`][c-x264]⮳{{hi:x264}}, [`x265`][c-x265]⮳{{hi:x265}}, [`vpx`][c-vpx]⮳{{hi:vpx}}) is the first step. If you need to work with a wide range of codecs or perform transcoding, [`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}} is a powerful but more complex option.

See [[multimedia_video | Multimedia: Video]].

## Audio Encoding

Many crates exist for specific audio codecs: - [`ogg`][c-ogg]⮳{{hi:ogg}}, [`flac`][c-flac]⮳{{hi:flac}}, [`mp3`][c-mp3]⮳{{hi:mp3}}, [`wav`][c-wav]⮳{{hi:wav}}... See [[multimedia_audio | Multimedia: Audio]].

## Image Encoding

While primarily for image loading and manipulation, [`image`][c-image]⮳{{hi:image}} also supports encoding to some formats (e.g., PNG, JPEG).
See [[multimedia_images | Multimedia: Image]].

## Transcoding

[`ffmpeg-next`][c-ffmpeg_next]⮳{{hi:ffmpeg-next}} is a powerful tool for transcoding (converting from one format to another).

## Related Topics

- [[encoding | Encoding]]
- [[multimedia | Multimedia]]
- [[multimedia_audio | Multimedia: Audio]]
- [[multimedia_video | Multimedia: Video]]
- [[multimedia_images | Multimedia: Images]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/418)
- Audio Formats (MP3, WAV, OGG)
- Video Formats (MP4, AVI, MKV)
- Image Formats (JPEG, PNG, GIF)
- Codecs (H.264, VP9, Opus)
</div>
