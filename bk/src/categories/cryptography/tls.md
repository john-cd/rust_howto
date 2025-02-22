# TLS

{{#include tls.incl.md}}

Transport Layer Security (TLS) is a cryptographic protocol designed to provide secure communication over a computer network. It is widely used in applications such as web browsing, [email][p-email], and instant messaging to ensure that data transmitted between the client and [server][p-server] remains private and integral. TLS is the successor to the Secure Sockets Layer (SSL) protocol.

## `rustls` {#rustls}

[![rustls][c-rustls-badge]][c-rustls] [![rustls-crates.io][c-rustls-crates.io-badge]][c-rustls-crates.io] [![rustls-github][c-rustls-github-badge]][c-rustls-github] [![rustls-lib.rs][c-rustls-lib.rs-badge]][c-rustls-lib.rs]{{hi:rustls}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

[`rustls`][c-rustls]⮳{{hi:rustls}} is a portable pure-rust high-level implementation of TLS. It implements TLS 1.2 and higher. Being written entirely in Rust, it avoids any dependencies on system-level TLS libraries. It is portable and works consistently across different platforms without needing to manage system-specific TLS libraries. It can be used in web servers, clients, and other network-dependent applications.

Choose `rustls` for portability and consistent behavior across platforms, and when you need fine-grained control over TLS settings, and when performance is critical.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/tls/rustls.rs:example}}
```

## `native-tls` {#native-tls}

[![native-tls][c-native_tls-badge]][c-native_tls] [![native-tls-crates.io][c-native_tls-crates.io-badge]][c-native_tls-crates.io] [![native-tls-github][c-native_tls-github-badge]][c-native_tls-github] [![native-tls-lib.rs][c-native_tls-lib.rs-badge]][c-native_tls-lib.rs]{{hi:native-tls}}

[`native-tls`][c-native_tls]⮳{{hi:native-tls}} is a wrapper over a platform's native TLS implementation and provides a cross-platform API for TLS/SSL communication.It abstracts over platform-specific TLS implementations, using SChannel on Windows, Secure Transport on macOS, and OpenSSL on other platforms

Choose `native-tls` when ease of use and automatic certificate management are preferred and when integration with the system's existing TLS infrastructure is important.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/tls/native_tls.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 write

https://en.wikipedia.org/wiki/Rustls

</div>
