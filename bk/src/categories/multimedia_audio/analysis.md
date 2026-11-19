<!-- TODO Reviewed ??-??-2026. -->

# Analyze a Window of Audio

PCM samples describe amplitude over time. A frequency spectrum describes a fixed
window of those samples, not the whole stream. Keep channel layout and sample rate
explicit: interleaved stereo must be deinterleaved or deliberately downmixed before
passing one channel to a mono analyzer. The sample rate determines each bin's frequency.

## Plan an FFT and Measure a Tone

[`rustfft`](https://docs.rs/rustfft/6.4.1/rustfft/) provides reusable transform plans.
This example generates a known tone, applies a periodic Hann window, and returns
one-sided linear peak amplitudes. It compensates for window gain and doubles interior
bins, but does not double DC or the even-length Nyquist bin. The tests check frequency,
amplitude, silence, Nyquist scaling, and invalid input.

```rust,ignore,noplayground
{{#include ../../../crates/cats/multimedia_audio/examples/spectrum.rs:example}}
```

```sh
just cargo run --manifest-path bk/crates/Cargo.toml --locked -p multimedia_audio --example spectrum
just cargo test --manifest-path bk/crates/Cargo.toml --locked -p multimedia_audio --example spectrum
```

The 1024-sample frame at 8192 Hz has 8 Hz bin spacing. The generated 1024 Hz tone is
exactly bin-centered, so its peak has a precise reference value. A tone between bins
spreads energy across neighboring bins even after windowing; the largest-bin frequency
is a quantized estimate, not a precision pitch detector. A silent frame has no peak.
Linear peak amplitude is not power spectral density, RMS loudness, or perceived volume.
For dBFS use a documented reference and floor before taking logarithms.

## Keep Analysis Off the Audio Callback

Plan FFTs, allocate buffers, load models, decode files, and perform UI work away from
the realtime callback. This teaching analyzer allocates its working vectors per call;
it is intended for an analysis worker, not a hard realtime callback. A streaming
implementation can preallocate FFT scratch and reuse both input and output buffers.

Use a bounded queue or an appropriate single-producer/single-consumer buffer for audio
frames. Define what happens when it fills: dropping a visualization frame may be valid,
while silently losing a recording frame is not. Avoid locks, filesystem I/O, logging,
and unbounded waits in the callback. Report underruns or dropped analysis frames through
counters read by the UI. Preserve ownership so a device callback cannot use freed state.

## Common Multimedia Crates and Recipe Boundaries

| Task | Common crates | Where to start |
| --- | --- | --- |
| Device capture/output | `cpal` | [Audio](audio.md); requires an actual device/backend. |
| Playback and mixing sources | `rodio` | [Audio](audio.md); output-stream lifetime must cover playback. |
| WAV read/write | `hound` | [WAV](wav.md), generated fixtures without a device. |
| Decode compressed audio | `symphonia` | [Encoding](../multimedia_encoding/encoding.md); codecs and container features must match the input. |
| Resample and analyze | `rubato`, `rustfft`, `realfft` | [Resampling](audio.md#resample) and this FFT recipe. RealFFT is a real-input FFT alternative, not an additional tested adapter here. |
| MIDI and spatial audio | `midir`, `hrtf` | [Audio](audio.md); MIDI-device I/O remains separate from message parsing. |
| Video codecs and containers | `rav1e`, `mp4` | [Encoding](../multimedia_encoding/encoding.md) and [Video](../multimedia_video/video.md); encoding packets alone does not produce every container. |
| Native media pipelines | `gstreamer`, `ffmpeg-next`, `ffmpeg-sidecar` | [Video](../multimedia_video/video.md); native libraries or external executables are prerequisites. |
| Interactive transport | `str0m`, `webrtc` | [Streaming](../multimedia/streaming.md); signaling, codec negotiation and teardown are separate responsibilities. |

These entries establish task coverage, not a claim that all backends or every listed
alternative has been executed. Start with the device-free fixture, then run device and
pipeline checks on each advertised platform.

## Related Topics

- [Audio](audio.md) and [WAV](wav.md).
- [Encoding](../multimedia_encoding/encoding.md).
- [Video](../multimedia_video/video.md).
- [Background UI Work](../gui/background_work.md).
