# TLS

{{#include tls.incl.md}}

Transport Layer Security (TLS) is a cryptographic protocol designed to provide secure communication over a computer network. It is widely used in applications such as web browsing, [email][p~email], and instant messaging to ensure that data transmitted between the client and [server][p~server] remains private and integral. TLS is the successor to the Secure Sockets Layer (SSL) protocol.

## `rustls` {#rustls}

[![rustls][c~rustls~docs~badge]][c~rustls~docs] [![rustls~crates.io][c~rustls~crates.io~badge]][c~rustls~crates.io] [![rustls~repo][c~rustls~repo~badge]][c~rustls~repo] [![rustls~lib.rs][c~rustls~lib.rs~badge]][c~rustls~lib.rs]{{hi:rustls}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

[`rustls`][c~rustls~docs]↗{{hi:rustls}} is a portable pure-rust high-level implementation of TLS. It implements TLS 1.2 and higher. Being written entirely in Rust, it avoids any dependencies on system-level TLS libraries. It is portable and works consistently across different platforms without needing to manage system-specific TLS libraries. It can be used in web servers, clients, and other network-dependent applications.

Choose [`rustls`][c~rustls~docs]↗{{hi:rustls}} (i) for portability and consistent behavior across platforms; (ii) when needing fine-grained control over TLS settings; and (iii) when performance is critical.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/tls/rustls.rs:example}}
```

See also: [Rustls (Wikipedia)][c~rustls~wikipedia]↗.

## `native-tls` {#native-tls}

[![native-tls][c~native-tls~docs~badge]][c~native-tls~docs] [![native-tls~crates.io][c~native-tls~crates.io~badge]][c~native-tls~crates.io] [![native-tls~repo][c~native-tls~repo~badge]][c~native-tls~repo] [![native-tls~lib.rs][c~native-tls~lib.rs~badge]][c~native-tls~lib.rs]{{hi:native-tls}}

[`native-tls`][c~native-tls~docs]↗{{hi:native-tls}} is a wrapper over a platform's native TLS implementation and provides a cross-platform API for TLS/SSL communication.It abstracts over platform-specific TLS implementations, using SChannel on Windows, Secure Transport on macOS, and OpenSSL on other platforms

Choose [`native-tls`][c~native-tls~docs]↗{{hi:native-tls}} when ease of use and automatic certificate management are preferred and when integration with the system's existing TLS infrastructure is important.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/tls/native_tls.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1182)
</div>
