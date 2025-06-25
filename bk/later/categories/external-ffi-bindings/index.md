# External FFI Bindings

[![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}}

Direct Rust FFI bindings to libraries written in other languages; often denoted by a `-sys` suffix. Safe and idiomatic wrappers are in the [[api-bindings | API Bindings]] category.

| Topic | Example(s) | Related Chapter(s) |
| --- | --- | --- |
| Audio | `alsa-sys`: Bindings to the ALSA library. `portaudio-sys`: Bindings to the PortAudio library. | [[multimedia_audio | Multimedia: Audio]] |
| Cryptography | `openssl-sys`: Bindings to OpenSSL. `libsodium-sys`: Bindings to the libsodium library. | [[cryptography | Cryptography]] |
| Database | `libsqlite3-sys`: Bindings to the SQLite library. `pq-sys`: Bindings to the PostgreSQL library. | [[database | Database]] |
| Graphics | `glfw-sys`: Bindings to the GLFW library. `vulkan-sys`: Bindings to the Vulkan API. | [[graphics | Graphics]] |
| Networking | `libcurl-sys`: Bindings to the libcurl library. `libssh2-sys`: Bindings to the libssh2 library. | [[network-programming | Network Programming]] |
| Parsing | `libxml2-sys`: Bindings to the libxml2 library. | [[parsing | Parsing]] |
| System | [`libc`][c-libc]⮳{{hi:libc}}: Bindings to various C library functions and types. `windows-sys`: Bindings to Windows APIs. | [[os | OS]] [[os_windows-apis | OS Windows APIs]] |
| Miscellaneous | `libgit2-sys`: Bindings to the libgit2 library. |  |

{{#include external_ffi_bindings.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/356)

```rust,editable
{{#include ../../../crates/cats/external_ffi_bindings/examples/api_bindings.rs:example}}
```

</div>
