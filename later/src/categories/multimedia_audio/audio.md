<!-- TODO Reviewed ??-??-2026 -->

# Audio

{{#include audio.incl.md}}

## Record, Output, or Process Audio {#audio}

```rust,editable
{{#include ../../../crates/cats/multimedia_audio/examples/audio/audio.rs:example}}
```

Also see [Write and read a WAV file in memory](wav.md) for an example that does not require an audio device.

## Play a Custom Sound Source with `rodio` {#rodio-source}

[![rodio][c~rodio~docs~badge]][c~rodio~docs]{{hi:rodio}} [![cat~multimedia::audio][cat~multimedia::audio~badge]][cat~multimedia::audio]{{hi:Audio playback}}{{hi:Synthesis}}

[`rodio`][c~rodio~docs]↗{{hi:rodio}} plays sounds on top of [`cpal`][c~cpal~docs]↗{{hi:cpal}}. A sound is a `rodio::Source`: an `Iterator` of `f32` samples that also reports its channel count, sample rate and duration. The audio thread pulls the samples, so generating a sound is just a matter of implementing the iterator. Adapters such as `mix`, `amplify` and `take_duration` combine sources. `Player` queues them on the mixer of a `DeviceSink`; the sound stops when the sink is dropped.

```rust,no_run,editable
{{#include ../../../crates/cats/multimedia_audio/examples/audio/rodio_source.rs:example}}
```

Opening a device needs hardware, so `play_chord` is not called by the tests. They pull samples from the source directly.

### Real-time constraints {#real-time-constraints}

The code that fills an audio buffer, whether a `cpal` callback or `Iterator::next` of a `rodio` source, runs on a high-priority thread that must finish before the device runs out of samples. Inside it:

- do not allocate, lock a `Mutex`, perform I/O, or log;
- send commands from the rest of the application through a lock-free queue or a ring buffer, or through atomics, and do heavy work (decoding, file access) on another thread;
- prepare buffers in advance.

Choose `cpal` directly when you need control over buffers, formats and latency, or to capture input. Choose `rodio` to play sounds and files with little code. Samples are `f32` in `rodio`, whereas a device may want `i16` or `u16`: `cpal` converts with `FromSample`, as in the first example.

## List MIDI Ports and Decode Messages {#midi-ports}

[![midir][c~midir~docs~badge]][c~midir~docs]{{hi:midir}} [![cat~multimedia::audio][cat~multimedia::audio~badge]][cat~multimedia::audio]{{hi:MIDI}}

[`midir`][c~midir~docs]↗{{hi:midir}} is a cross-platform library for realtime MIDI input and output. `MidiInput::ports` lists the ports, and `connect` registers a callback that receives raw messages. A MIDI message is a few bytes: a status byte, whose high nibble is the message kind and whose low nibble is the channel, followed by data bytes. Decoding is a pure function, which is easy to test without any hardware.

```rust,editable
{{#include ../../../crates/cats/multimedia_audio/examples/audio/midi_ports.rs:example}}
```

The `midir` callback also runs on a thread of the library: apply the real-time rules above, and forward the decoded messages to the application through a channel.

## Binaural Audio with `hrtf` {#hrtf}

[`hrtf`][c~hrtf~docs]↗{{hi:hrtf}} renders a mono source as binaural stereo from a head-related impulse response (HRIR) sphere file, which gives the impression of a position in 3D space over headphones. The crate does not ship HRIR data: download a sphere in the format that `HrirSphere` loads.


## Related Topics {#related-topics .skip}

- [[multimedia | Multimedia]].
- [[multimedia_encoding | Multimedia: Encoding]].
- [[multimedia_video | Multimedia: Video]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[audio: write](https://github.com/john-cd/rust_howto/issues/415)
[audio files, playback and MIDI](https://github.com/john-cd/rust_howto/issues/1943)
[ring-buffer recipe for audio callbacks](https://github.com/john-cd/rust_howto/issues/1908)
</div>
