# Multimedia - Encoding

[![cat~multimedia::encoding][cat~multimedia::encoding~badge]][cat~multimedia::encoding]{{hi:Multimedia encoding}}

{{#include encoding.incl.md}}

Encode or decode binary data in multimedia formats.

## Video Encoding

Crates are available for most common codecs: [`rav1e`][c~rav1e~docs]⮳{{hi:rav1e}}, [`x264`][c~x264~docs]⮳{{hi:x264}}, [`x265`][c~x265~docs]⮳{{hi:x265}}, [`vpx`][c~vpx~docs]⮳{{hi:vpx}}.

If you need to work with a wide range of codecs (MP4, AVI, MKV, H.264, VP9, Opus...), [`ffmpeg-next`][c~ffmpeg_next~docs]⮳{{hi:ffmpeg-next}} is a powerful but more complex option.

See [[multimedia_video | Multimedia: Video]].

## Audio Encoding

Many crates exist for specific audio codecs (MP3, WAV, OGG): [`ogg`][c~ogg~docs]⮳{{hi:ogg}}, [`flac`][c~flac~docs]⮳{{hi:flac}}, [`mp3`][c~mp3~docs]⮳{{hi:mp3}}, [`wav`][c~wav~docs]⮳{{hi:wav}}... See [[multimedia_audio | Multimedia: Audio]].

## Image Encoding

While primarily for image loading and manipulation, [`image`][c~image~docs]⮳{{hi:image}} also supports encoding to some formats (e.g., JPEG, PNG, GIF).
See [[multimedia_images | Multimedia: Image]].

## Transcoding

[`ffmpeg-next`][c~ffmpeg_next~docs]⮳{{hi:ffmpeg-next}} is a powerful tool for transcoding (converting from one format to another).

## Related Topics

- [[encoding | Encoding]].
- [[multimedia | Multimedia]].
- [[multimedia_audio | Multimedia: Audio]].
- [[multimedia_video | Multimedia: Video]].
- [[multimedia_images | Multimedia: Images]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/418)
</div>
