<!-- TODO Reviewed ??-??-2026 -->

# Multimedia: Audio

[![cat~multimedia::audio][cat~multimedia::audio~badge]][cat~multimedia::audio]{{hi:Audio}}

Audio tasks include device input and output, decoding and encoding, synthesis, streaming, and digital signal processing. Select a device API, format library, or DSP toolkit according to the part of the pipeline you need. For most common device input and output tasks, [`cpal`][c~cpal~docs]↗{{hi:cpal}} is sufficient.

| Task                         | Crates                                                                                                              | Notes                                                                                 |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Device input and output      | [`cpal`][c~cpal~docs]↗{{hi:cpal}}, [`rodio`][c~rodio~docs]↗{{hi:rodio}}                                             | `cpal` exposes audio devices and streams; `rodio` provides higher-level playback.     |
| MIDI input and output        | [`midir`][c~midir~docs]↗{{hi:midir}}                                                                                  | Connect to MIDI ports and receive or send raw messages in real time.                  |
| Decode media                 | [`symphonia`][c~symphonia~docs]↗{{hi:symphonia}}                                                                    | Decode common compressed and uncompressed formats.                                    |
| Read and write audio formats | [`hound`][c~hound~docs]↗{{hi:hound}}, [`ogg`][c~ogg~docs]↗{{hi:ogg}}, [`flac`][c~flac~docs]↗{{hi:flac}}             | `hound` handles WAV; use Ogg or FLAC crates when working directly with those formats. |
| Encode Vorbis and Opus       | [`vorbis_rs`][c~vorbis_rs~docs]↗{{hi:vorbis_rs}}, [`opus`][c~opus~docs]↗{{hi:opus}}                                 | Use an encoder library appropriate to the target codec.                               |
| Binaural rendering           | [`hrtf`][c~hrtf~docs]↗{{hi:hrtf}}                                                                                      | Spatialize a mono source with head-related impulse responses.                         |
| Synthesis and DSP            | [`fundsp`][c~fundsp~docs]↗{{hi:fundsp}}, [`dasp`][c~dasp~docs]↗{{hi:dasp}}, [`rubato`][c~rubato~docs]↗{{hi:rubato}} | Synthesize, process, or resample sample streams.                                      |
| Audio plugins                | [`nih-plug`][c~nih-plug~repo]↗{{hi:nih-plug}}                                                                       | Framework for building audio plugins.                                                 |

## Audio Streaming

For low-latency streaming, use a device API such as `cpal` to receive or send sample buffers. A playback library such as `rodio` can handle common output tasks; async runtimes can coordinate surrounding network or application work, but the audio callback should remain bounded and avoid blocking.

## Audio Analysis

Analysis commonly builds on decoding and digital signal processing. [`symphonia`][c~symphonia~docs]↗{{hi:symphonia}} can decode input, while `fundsp`, `dasp`, and `rubato` provide DSP and resampling building blocks.

<!-- `synth-rs` was omitted in the prior rewrite because its reference was flagged as old; check its current maintenance before recommending it. -->

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
