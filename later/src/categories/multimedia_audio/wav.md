# Write and Read a WAV File in Memory

[hound][c~hound~docs]↗{{hi:hound}} can write a WAV stream to any seekable writer. This example uses an in-memory `Cursor`, finalizes the WAV header, then reads the samples back without an audio device.

```rust,editable
{{#include ../../../crates/cats/multimedia_audio/examples/wav/wav.rs:example}}
```
