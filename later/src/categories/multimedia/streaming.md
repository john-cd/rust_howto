<!-- TODO Reviewed ??-??-2026 -->

# Streaming Media

Real-time media combines a media format and codec with protocols for delivery and session control. The main protocols solve different parts of that problem:

| Protocol     | Role                                                                                                                                                                                                                                    |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RTSP         | Controls a streaming session, such as setting up, starting, pausing, or stopping delivery. The media itself is carried by a separate transport, often RTP.                                                                              |
| RTP and RTCP | RTP carries real-time audio or video packets with sequence numbers and timestamps; RTCP reports on delivery and participants. RTP does not guarantee delivery or quality of service.                                                    |
| WebRTC       | Provides peer connections for interactive, low-latency communication in browsers and native applications. It uses RTP for media, while the application supplies signaling to exchange session descriptions and connectivity candidates. |

RTSP 2.0 is specified separately from the widely deployed RTSP 1.0, and the versions are not wire-compatible beyond version negotiation. Check which version a server supports before choosing a client. See the [RTSP 2.0 specification][rfc~7826] and the [RTP specification][rfc~3550].

## Rust Building Blocks

| Use case                                | Starting point                                                                                                                                                                                                                                                                                                              |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Connect WebRTC peers                    | [`webrtc`][c~webrtc~docs] offers an async, runtime-agnostic API. [`str0m`][c~str0m~docs] is a Sans-I/O WebRTC implementation when the application needs to drive I/O and event handling itself. Both require an application signaling service; connectivity across restrictive networks may also need STUN or TURN servers. |
| Read an RTSP stream                     | [`retina`][c~retina~docs] is an asynchronous RTSP client. Use it to connect to an RTSP server and receive the stream; decode and display or record the media with the appropriate codec and pipeline tools.                                                                                                                 |
| Build an ingest or transcoding pipeline | The [GStreamer Rust bindings][gstreamer~rust~docs] connect sources, demuxers, decoders, encoders, and sinks as a pipeline. They require the native GStreamer libraries and the plugins for the formats in use; see [Multimedia - Video](../multimedia_video/video.md) for installation guidance and examples.               |

Prefer a WebRTC or media pipeline library over implementing RTP packetization directly. A custom RTP endpoint also needs to handle codec payload formats, timing, loss, and RTCP feedback. For one-to-many live delivery, plan for a media server or forwarding service so each publisher does not have to upload a separate copy to every viewer.

## Negotiate a WebRTC Session with `str0m` {#str0m}

[`str0m`][c~str0m~docs]↗{{hi:str0m}} does no network I/O of its own, so the offer and answer can be exchanged in a single process. This example negotiates a send-only audio stream between two peers; a real application would carry the SDP over its signaling service and then move UDP packets between the peers.

```rust,editable
{{#include ../../../crates/cats/multimedia/examples/streaming/main.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
