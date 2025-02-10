# TLS

{{#include tls.incl.md}}

## `rustls` {#rustls}

[![rustls][c-rustls-badge]][c-rustls] [![rustls-crates.io][c-rustls-crates.io-badge]][c-rustls-crates.io] [![rustls-github][c-rustls-github-badge]][c-rustls-github] [![rustls-lib.rs][c-rustls-lib.rs-badge]][c-rustls-lib.rs]{{hi:rustls}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

Rustls is a portable pure-rust high-level implementation of TLS. Implements TLS 1.2 and higher.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/tls/rustls.rs:example}}
```

## `native-tls` {#native-tls}

[![native-tls][c-native_tls-badge]][c-native_tls] [![native-tls-crates.io][c-native_tls-crates.io-badge]][c-native_tls-crates.io] [![native-tls-github][c-native_tls-github-badge]][c-native_tls-github] [![native-tls-lib.rs][c-native_tls-lib.rs-badge]][c-native_tls-lib.rs]{{hi:native-tls}}

A wrapper over a platform's native TLS implementation. Delegates to the system TLS implementations on windows and macOS, and uses OpenSSL on Linux.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/tls/native_tls.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 write
</div>
