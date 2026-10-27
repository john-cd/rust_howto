<!-- TODO Reviewed ??-??-2026 -->

# Multimedia - Encoding

[![cat~multimedia::encoding][cat~multimedia::encoding~badge]][cat~multimedia::encoding]{{hi:Multimedia encoding}}

{{#include encoding.incl.md}}

Encoding converts media samples or frames into a codec's representation. A container such as MP4 or Matroska packages encoded streams with timing and metadata; it does not define the codec itself. Transcoding combines decoding and encoding, while playback also needs a decoder, clock, and output device. Use this index to choose a format library, then follow the media-specific pages for recipes.

| Media or format | Example crates                                                                                                                                   | Scope                                                                                                                                                                                  |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Audio           | [`symphonia`][c~symphonia~docs]↗, [`hound`][c~hound~docs]↗, [`opus`][c~opus~docs]↗                                                               | Symphonia demuxes and decodes audio formats; Hound reads and writes WAV; Opus bindings encode and decode Opus.                                                                         |
| Images          | [`image`][c~image~docs]↗, [`png`][c~png~docs]↗, [`jpeg-decoder`][c~jpeg-decoder~docs]↗, [`zune-jpeg`][c~zune-jpeg~docs]↗, [`webp`][c~webp~docs]↗ | Common image codecs; use the [image page][ex~multimedia_encoding~images] for loading, manipulation, and saving.                                                                        |
| Video           | [`rav1e`][c~rav1e~docs]↗, [`dav1d`][c~dav1d~repo]↗                                                                                               | `rav1e` encodes AV1; `dav1d` provides Rust bindings to the native AV1 decoder library. Broader format support through FFmpeg or GStreamer requires their native libraries and plugins. |
| Containers      | [`mp4`][c~mp4~docs]↗, [`matroska`][c~matroska~docs]↗                                                                                             | Read or write container metadata and streams; container support is separate from codec support.                                                                                        |

## Recipes by Media Type {#recipes-by-media-type}

- [Audio encoding and decoding][ex~multimedia_encoding~audio], including sample formats and WAV files.
- [Image formats][ex~multimedia_encoding~images], including image encoding and decoding.
- [Video encoding and pipelines][ex~multimedia_encoding~video], including AV1 and system multimedia frameworks.
- [Encoding examples][ex~multimedia_encoding~encoding] in this category.

Feature flags often determine which codecs a crate supports. Check the selected format and codec features, licensing requirements, platform support, and native-library requirements before choosing a dependency. For broad format support, FFmpeg and GStreamer are flexible options, but their build and runtime requirements vary by platform.

## Related Topics {#related-topics .skip}

- [[multimedia | Multimedia]].
- [[multimedia_audio | Multimedia: Audio]].
- [[multimedia_images | Multimedia: Images]].
- [[multimedia_video | Multimedia: Video]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
