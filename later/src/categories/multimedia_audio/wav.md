# Write and Read a WAV File in Memory

[hound][c~hound~docs]↗{{hi:hound}} can write a WAV stream to any seekable writer. This example uses an in-memory `Cursor`, finalizes the WAV header, then reads the samples back without an audio device.

```rust,editable
{{#include ../../../crates/cats/multimedia_audio/examples/wav/wav.rs:example}}
```

## Write a Sine Wave to a WAV File {#wav-write-read}

[![hound][c~hound~docs~badge]][c~hound~docs]{{hi:hound}} [![cat~multimedia::audio][cat~multimedia::audio~badge]][cat~multimedia::audio]{{hi:Sine wave}}

A `WavSpec` describes the format of the file: channels, sample rate, bits per sample and whether the samples are integers or floats. The writer streams the samples one at a time, so a rendered composition does not have to fit in memory; wrap a `File` in a `BufWriter` to write to disk. The header holds the length of the data, which is only known at the end: call `finalize`.

```rust,editable
{{#include ../../../crates/cats/multimedia_audio/examples/audio/wav_write_read.rs:example}}
```

Pick the sample format to match what you do with the file. 16-bit integers are the most compatible. 32-bit floats keep the headroom of a processing chain. 24-bit integers are common for recording.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[audio files, playback and MIDI](https://github.com/john-cd/rust_howto/issues/1943)
</div>
