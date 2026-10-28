<!-- TODO Reviewed ??-??-2026 -->

# Encoding

{{#include encoding.incl.md}}

Multimedia encoding encompasses transforming raw sample or frame streams into compressed codec formats, packaging encoded bitstreams into container structures, and inspecting container metadata.

For higher-level image processing and codec handling, see the [Image Formats page][ex~multimedia_encoding~images]. For video streams and pipeline architectures, see the [Video Encoding page][ex~multimedia_encoding~video].

## Decode Audio Formats to PCM {#decode-audio}

[`symphonia`][c~symphonia~docs]↗{{hi:symphonia}} probes and decodes compressed or uncompressed audio streams into uncompressed linear PCM sample buffers. It handles format detection, container demuxing, and codec decoding across various formats like WAV, MP3, FLAC, and AAC.

```rust,editable
{{#include ../../../crates/cats/multimedia_encoding/examples/encoding/audio_decode.rs:example}}
```

## Encode PCM Audio Samples to WAV {#encode-wav}

[`hound`][c~hound~docs]↗{{hi:hound}} writes raw linear PCM sample buffers into WAV container files. WAV is standard uncompressed audio format widely supported by digital audio workstations and media players.

```rust,editable
{{#include ../../../crates/cats/multimedia_encoding/examples/encoding/wav_encode.rs:example}}
```

## Read MP4 Container Metadata {#mp4-metadata}

[`mp4`][c~mp4~docs]↗{{hi:mp4}} reads ISO base media file format (MP4/fMP4) containers, allowing applications to inspect track metadata, track types, timescales, and clip durations without parsing full elementary bitstreams.

```rust,editable
{{#include ../../../crates/cats/multimedia_encoding/examples/encoding/mp4_metadata.rs:example}}
```

## Encode Video Frames with `rav1e` {#encoding}

[`rav1e`][c~rav1e~docs]↗{{hi:rav1e}} encodes video frames into AV1 packet bitstreams. The example below configures an AV1 encoder context, submits a raw frame, and receives encoded packet payloads.

```rust,editable
{{#include ../../../crates/cats/multimedia_encoding/examples/encoding/encoding.rs:example}}
```

## Related Topics {#related-topics .skip}

- Audio and video codecs.
- Container formats and demuxing.
- File formats and serialization.
- Performance-sensitive multimedia pipelines.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
