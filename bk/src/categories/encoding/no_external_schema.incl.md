<!-- TODO Reviewed ??-??-2026. -->

| Format | Crate | Self-describing payload? | `no_std`? | Typical use |
| --- | --- | --- | --- | --- |
| Bincode | [`bincode`][ex~encoding~bincode] | No | Yes, with default features disabled | Compact Rust-to-Rust storage or transport with a shared type definition |
| MessagePack | [`rmp-serde`][ex~encoding~rmp-serde] | Yes, when decoded as generic values | No; it uses `std` | Binary interchange; named maps can preserve struct field names |
| CBOR | [`ciborium`][ex~encoding~ciborium] | Yes | Yes, with default features disabled; uses `alloc` | Standardized interchange, including constrained systems |
| Postcard | [`postcard`][ex~encoding~postcard] | No | Yes | Compact messages for embedded and other constrained systems |

| Recipe | Crates | Categories |
| --- | --- | --- |
| [`bincode`][ex~encoding~bincode] | {{#crate bincode}} | {{#categories encoding=Encoding}} |
| [`ciborium`][ex~encoding~ciborium] | {{#crate ciborium}} | {{#categories encoding=Encoding}} |
| [`postcard`][ex~encoding~postcard] | {{#crate postcard}} | {{#categories encoding=Encoding}} |
| [`rmp-serde`][ex~encoding~rmp-serde] | {{#crate rmp-serde}} | {{#categories encoding=Encoding}} |
| [`rkyv`][ex~encoding~rkyv] | {{#crate rkyv}} | {{#categories encoding=Encoding}} |
