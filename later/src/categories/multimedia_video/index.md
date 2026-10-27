<!-- TODO Reviewed ??-??-2026 -->

# Multimedia - Video

[![cat~multimedia::video][cat~multimedia::video~badge]][cat~multimedia::video]{{hi:Video}}

Record, output, or process video.

For most video-related tasks, FFmpeg ([`ffmpeg-next`][c~ffmpeg-next~docs]↗{{hi:ffmpeg-next}}) is the most comprehensive option, but it can be more complex to work with. If you're focused on specific encoding tasks (e.g., AV1, H.264), the dedicated encoder crates ([`rav1e`][c~rav1e~docs]↗{{hi:rav1e}}, [`x264`][c~x264~docs]↗{{hi:x264}}) might be a better choice. For video streaming, you'll need to combine video processing with [[network-programming | networking]] libraries.

{{#include video.incl.md}}

## Codecs and Formats

Many crates exist for specific codecs:

- [`rav1e`][c~rav1e~docs]↗{{hi:rav1e}}: AV1 encoder. A good choice for modern video encoding.
- [`x264`][c~x264~docs]↗{{hi:x264}}: H.264 encoder. A widely used and mature encoder.
- [`vpx`][c~vpx~docs]↗{{hi:vpx}}: VP8/VP9 encoder.

The runnable example in the table above encodes synthetic frames with [`rav1e`][c~rav1e~docs]↗{{hi:rav1e}}, so it needs no system libraries.

## General Video Decoding, Encoding, and Manipulation

FFmpeg bindings ([`ffmpeg-next`][c~ffmpeg-next~docs]↗{{hi:ffmpeg-next}}) provide access to FFmpeg's formats, codecs, and filters. The [maintainer describes the crate as being in maintenance mode][ffmpeg-next~maintenance]↗: compatibility and bug fixes are priorities, but new FFmpeg APIs may need contributions. Check the binding and native-library versions required by your application.

## Pipelines with GStreamer {#gstreamer}

[`gstreamer`][c~gstreamer~docs]↗{{hi:gstreamer}} binds the GStreamer framework: you connect elements (sources, decoders, filters, encoders, sinks) into a pipeline, then run it. The plugin model makes it a good fit for capture, streaming and playback, and the crate follows the C library's releases (GStreamer must be installed; see [Installing GStreamer][gstreamer~installing~website]↗). Elements for most codecs are provided by plugins, including pure Rust ones from [`gst-plugins-rs`][gst-plugins-rs~repo]↗.

## Drive the FFmpeg CLI {#ffmpeg-sidecar}

[`ffmpeg-sidecar`][c~ffmpeg-sidecar~docs]↗{{hi:ffmpeg-sidecar}} runs the `ffmpeg` executable as a child process and streams raw frames and progress events back to Rust. It avoids linking against FFmpeg's C libraries, which makes builds easy. [Download FFmpeg][ffmpeg~download~website]↗ separately.

## Video Effects and Filters {#effects-and-filters}

Apply effects while the video is decoded or encoded, instead of writing your own pixel loops:

- With FFmpeg, pass a filter graph, such as `scale=640:-1,hue=s=0`, through `ffmpeg-sidecar` or the filter API of [`ffmpeg-next`][c~ffmpeg-next~docs]↗{{hi:ffmpeg-next}}.
- With GStreamer, insert filter elements (`videoscale`, `videobalance`, `gleffects`) into the pipeline.
- For custom per-frame effects, decode frames to RGB buffers and process them with [`image`][c~image~docs]↗{{hi:image}} (see [[multimedia_images | Multimedia: Images]]).

## Video Analysis {#analysis}

To analyze a video, extract frames (with `ffmpeg-sidecar`, `ffmpeg-next` or a GStreamer `appsink`), then process each frame as an image: compute statistics with [`image`][c~image~docs]↗{{hi:image}}, or detect objects and motion with [`opencv`][c~opencv~docs]↗{{hi:opencv}} (see [[computer-vision | Computer Vision]]). [`av-scenechange`][c~av-scenechange~docs]↗{{hi:av-scenechange}} detects scene cuts, which is useful before encoding.

## Video Streaming

For video streaming, you'll often need to combine video encoding/decoding with [[network-programming | networking]] libraries. Crates like [`tokio`][c~tokio~docs]↗{{hi:tokio}} for [[asynchronous | asynchronous]] networking are often used.

## Video Playback

Video playback crates are less common in Rust.

Building a full-fledged video player in Rust is a complex undertaking. You'd likely combine video decoding with [[graphics | graphics]] libraries ([`wgpu`][c~wgpu~docs]↗{{hi:wgpu}}, [`pixels`][c~pixels~docs]↗{{hi:pixels}}, etc.) and [[multimedia_audio | audio]] libraries (e.g. [`cpal`][c~cpal~docs]↗{{hi:cpal}}).

## Related Topics

- [[graphics | Graphics]].
- Image Processing: see [[images | Images]] and [[multimedia_images | Multimedia: Images]].
- [[multimedia | Multimedia]].
- [[multimedia_images | Multimedia Images]].
- [[rendering | Rendering]].
- Transcoding (converting video from one format to another) - see [[multimedia_encoding | Multimedia: Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
