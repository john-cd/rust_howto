<!-- TODO Reviewed ??-??-2026 -->

# Aerospace Protocols

{{#include aerospace_protocols.incl.md}}

## Aerospace Protocols {#aerospace_protocols}

The example serializes a MAVLink 2 heartbeat and decodes it from memory. To exchange frames with a separate MAVLink endpoint, applications can configure a connection such as `udpout:127.0.0.1:14550` using `mavlink::connect`; a receiver must be available at that address.

```rust,editable
{{#include ../../../crates/cats/aerospace_protocols/examples/aerospace_protocols/aero_protocols.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[aerospace | Aerospace]].
- [[embedded | Embedded Systems]].
- [[space_protocols | Space Protocols]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
