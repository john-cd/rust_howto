## Read and write integers in little-endian byte order

[![byteorder-badge]][byteorder] [![cat-encoding-badge]][cat-encoding]

`byteorder` can reverse the significant bytes of structured data.  This may
be necessary when receiving information over the network, such that bytes
received are from another system.

```rust,editable
{#include ../../../deps/examples/endian-byte.rs}
```
