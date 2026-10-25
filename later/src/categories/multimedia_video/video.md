<!-- TODO Reviewed ??-??-2026 -->

# Video

{{#include video.incl.md}}

Video applications commonly combine a decoder, frame processing, an encoder, and a container muxer. A codec describes how frames are compressed; a container such as Matroska or MP4 packages video, audio, and timing metadata. Pick libraries based on the formats you need and whether you can install native multimedia libraries on the target system.

## Encode AV1 Frames with rav1e {#rav1e}

[![rav1e][c~rav1e~docs~badge]][c~rav1e~docs] [![rav1e~repo][c~rav1e~repo~badge]][c~rav1e~repo]{{hi:rav1e}}

[`rav1e`][c~rav1e~docs]↗{{hi:rav1e}} is a pure-Rust AV1 encoder. The example fills a synthetic YUV frame, pads its planes, sends it to an encoder context, flushes, and drains encoded packets. It runs offline and demonstrates the encoder API without needing a video file or demuxer.

This code produces AV1 packet bytes. A player or video file also needs a container and timing information; use a muxer or a higher-level multimedia pipeline for those tasks.

```rust,editable
{{#include ../../../crates/cats/multimedia_video/examples/video/video.rs:example}}
```

## Decode, Transcode, and Filter

For broad format support, the [FFmpeg bindings][c~ffmpeg-next~docs]↗{{hi:ffmpeg-next}} expose decoding, encoding, rescaling, and filters through the FFmpeg libraries installed on the system. Building the native bindings requires a compatible FFmpeg development installation. The [GStreamer][c~gstreamer~docs]↗{{hi:GStreamer}} Rust bindings provide a plugin-based pipeline model for capture, conversion, encoding, and playback; available formats depend on installed plugins.

A transcoding pipeline demuxes the input container, decodes frames, applies any transforms, encodes output streams, then muxes them into a destination container. Re-encoding can reduce quality and use substantial CPU time. If only the container needs to change and the codecs are compatible, stream-copying avoids decoding and re-encoding.

## Playback and Streaming

A video player also needs audio and video synchronization, a clock, buffering, seeking, and a display surface. Applications often combine a decoder with a GUI or graphics framework and an audio output library. For network delivery, add a streaming protocol and account for latency, congestion, and media timing; an encoder alone does not provide streaming.

## Video Effects and Analysis

Filters can crop, scale, color-convert, deinterlace, or composite frames. FFmpeg and GStreamer provide filter and pipeline options; application-specific image operations can also run on decoded frames.

[`av-scenechange`][c~av-scenechange~docs]↗{{hi:av-scenechange}} analyzes decoded video and reports candidate scene-change points useful for encoder keyframe placement. Scene-change detection is one form of analysis; object recognition, motion estimation, and human-oriented scene labels require different models and goals. Extract frames before using image-processing tools, and preserve frame timestamps when results must align with the source.

## Related Topics {#related-topics .skip}

- [[multimedia | Multimedia]].
- [[multimedia_encoding | Multimedia: Encoding]].
- [[multimedia_audio | Multimedia: Audio]].
- [[graphics | Graphics]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
