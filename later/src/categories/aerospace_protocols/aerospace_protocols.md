<!-- TODO Reviewed ??-??-2026. -->

# Aerospace Protocols

{{#include aerospace_protocols.incl.md}}

## Decode MAVLink Heartbeats and Detect Gaps {#aerospace_protocols}

The [mavlink][c~mavlink~docs] crate encodes and decodes typed messages from a selected dialect. Its common dialect includes heartbeats, which identify the sender and report its state. This recipe serializes two synthetic MAVLink 2 heartbeats, decodes their headers, and counts how many sequence numbers are missing.

```console
just cargo run --manifest-path later/crates/Cargo.toml --locked -p aerospace_protocols --example aerospace_protocols
```

The output ends with `Missing frames: 2`. The sequence advances from 254 to 1, so entries 255 and 0 are missing. The counter works on one ordered, unfiltered channel without duplicate frames. Filtering a capture down to heartbeats or combining redundant links can create apparent gaps. The [MAVLink packet-loss guide][mavlink~packet-loss] describes the assumptions.

```rust,editable
{{#include ../../../crates/cats/aerospace_protocols/examples/aerospace_protocols/aero_protocols.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].
- [[embedded | Embedded Systems]].
- [[space_protocols | Space Protocols]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
