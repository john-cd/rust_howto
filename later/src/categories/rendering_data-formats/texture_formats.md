# Texture Formats

[![cat~rendering::data-formats][cat~rendering::data-formats~badge]][cat~rendering::data-formats]{{hi:Texture formats}}{{hi:Rendering data formats}}

Separate image decoding from GPU texture containers. Common raster assets such as PNG, JPEG, GIF, and WebP are covered in [[multimedia_images | Multimedia: Images]]. KTX2 and DDS store texture metadata and payloads in containers that graphics applications can upload or transcode for their target devices.

| Format          | Rust crate                                   | Role                                                                                                           |
| --------------- | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| KTX2            | [`ktx2`][c~ktx2~docs]↗                       | Parses and validates the container header, metadata, and mip-level data.                                       |
| DDS             | [`ddsfile`][c~ddsfile~docs]↗                 | Reads and writes DDS container metadata and exposes the stored texture data. It does not decode pixel formats. |
| Basis Universal | [`basis-universal`][c~basis-universal~docs]↗ | Transcodes universal compressed texture data into GPU-friendly formats.                                        |

Choose a target format supported by the graphics API and device. A container parser provides metadata and bytes; it does not guarantee that the payload can be decoded or uploaded without a compatible transcoder.

## Related Topics {#related-topics .skip}

- [[multimedia_images | Multimedia: Images]].
- [[rendering_graphics-api | Rendering: Graphics API]].
- [[gpu_abstraction_layers | GPU Abstraction Layers]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
