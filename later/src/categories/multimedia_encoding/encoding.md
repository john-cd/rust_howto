<!-- TODO Reviewed ??-??-2026 -->

# Encoding

{{#include encoding.incl.md}}

## Encode Your Media {#encoding}

[`rav1e`][c~rav1e~docs]↗{{hi:rav1e}} encodes frames into AV1 packets. The example below encodes one blank frame and reports the packet payload size. A complete video file also needs a container and muxer; this snippet does not write a playable media file.

```rust,editable
{{#include ../../../crates/cats/multimedia_encoding/examples/encoding/encoding.rs:example}}
```

## Related Topics {#related-topics .skip}

- Audio and video codecs.
- Media processing pipelines.
- File formats and serialization.
- Performance-sensitive multimedia code.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [encoding: write](https://github.com/john-cd/rust_howto/issues/417)
</div>
