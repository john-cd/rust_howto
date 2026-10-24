<!-- TODO Reviewed ??-??-2026 -->

# Multimedia: Audio

Audio tasks include device input and output, decoding and encoding, synthesis, and digital signal processing. Select a device API, format library, or DSP toolkit according to the part of the pipeline you need.

| Task | Crates | Notes |
|---|---|---|
| Device input and output | [`cpal`][c~cpal~docs]↗{{hi:cpal}}, [`rodio`][c~rodio~docs]↗{{hi:rodio}} | `cpal` exposes audio devices and streams; `rodio` provides higher-level playback. |
| Decode media | [`symphonia`][c~symphonia~docs]↗{{hi:symphonia}} | Decode common compressed and uncompressed formats. |
| Read and write WAV | [`hound`][c~hound~docs]↗{{hi:hound}} | Read and write WAV files; the example creates and reads a WAV in memory. |
| Encode Vorbis and Opus | [`vorbis_rs`][c~vorbis_rs~docs]↗{{hi:vorbis_rs}}, [`opus`][c~opus~docs]↗{{hi:opus}} | Use an encoder library appropriate to the target codec. |
| Synthesis and DSP | [`fundsp`][c~fundsp~docs]↗{{hi:fundsp}}, [`dasp`][c~dasp~docs]↗{{hi:dasp}}, [`rubato`][c~rubato~docs]↗{{hi:rubato}} | Synthesize, process, or resample sample streams. |
| Audio plugins | [`nih-plug`][c~nih-plug~repo]↗{{hi:nih-plug}} | Framework for building audio plugins. |

## Examples

{{#include audio.incl.md}}

## Related Topics

- [[multimedia | Multimedia]].
- [[multimedia_encoding | Multimedia: Encoding]].
- [[multimedia_video | Multimedia: Video]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: category index content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1903)
</div>
