<!-- TODO Reviewed ??-??-2026 -->

# External FFI Bindings

[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}

This category collects low-level Rust bindings to APIs implemented outside Rust, commonly C libraries and operating-system APIs. The `-sys` suffix is a convention, not a guarantee: for example, [`libc`][c~libc~docs]↗{{hi:libc}} and `windows-sys` expose system APIs without that suffix.

When a maintained safe wrapper exists, start with the wrapper in [[api-bindings | API Bindings]]. Use a low-level binding directly when you need its API or are building a wrapper. In that case, check the crate's safety contract, native-library requirements, and platform support; FFI code must uphold the foreign API's ABI, ownership, lifetime, and thread-safety requirements.

The table is a set of examples, not an exhaustive catalog. The `-sys` crates commonly provide the low-level layer used by higher-level Rust crates.

| Area          | Example bindings                                                                                                                                              | Related chapter                                   |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| Audio         | [`alsa-sys`][c~alsa-sys~docs]↗{{hi:alsa-sys}}: Bindings to the ALSA library. `portaudio-sys`: Bindings to the PortAudio library.                              | [[multimedia_audio | Multimedia: Audio]]          |
| Cryptography  | `openssl-sys`: Bindings to OpenSSL. `libsodium-sys`: Bindings to the libsodium library.                                                                       | [[cryptography | Cryptography]]                   |
| Database      | `libsqlite3-sys`: Bindings to the SQLite library. `pq-sys`{{hi:pq-sys}}: Bindings to the PostgreSQL library.                                                  | [[database | Database]]                           |
| Graphics      | [`glfw-sys`][c~glfw-sys~docs]↗{{hi:glfw-sys}}: Bindings to the GLFW library. [`vulkan-sys`][c~vulkan-sys~docs]↗{{hi:vulkan-sys}}: Bindings to the Vulkan API. | [[graphics | Graphics]]                           |
| Networking    | `libcurl-sys`: Bindings to the libcurl library. `libssh2-sys`: Bindings to the libssh2 library.                                                               | [[network-programming | Network Programming]]     |
| Parsing       | `libxml2-sys`: Bindings to the libxml2 library.                                                                                                               | [[parsing | Parsing]]                             |
| System        | [`libc`][c~libc~docs]↗{{hi:libc}}: Bindings to various C library functions and types. `windows-sys`: Bindings to Windows APIs.                                | [[os | OS]] [[os_windows-apis | OS Windows APIs]] |
| Miscellaneous | `libgit2-sys`: Bindings to the libgit2 library.                                                                                                               |                                                   |

{{#include external_ffi_bindings.incl.md}}

## Related Topics

- [[api-bindings | API Bindings]].
- [[development-tools_ffi | Development Tools: FFI]].
- [[os | OS]].
- [[network-programming | Network Programming]].
- [[cryptography | Cryptography]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
review

```rust,editable
{{#include ../../../crates/cats/external_ffi_bindings/examples/external_ffi_bindings.rs:example}}
```

</div>
