<!-- TODO Reviewed ??-??-2026 -->

# TLS

{{#include tls.incl.md}}

Transport Layer Security (TLS) protects the confidentiality and integrity of network traffic and authenticates the server. It is used by web browsing, [email][p~email], and other client/server protocols. TLS 1.2 and TLS 1.3 are the supported versions in the examples below; Secure Sockets Layer (SSL) is obsolete. Use a maintained TLS library and validate the server name and certificate chain.

## Connect to a Server over TLS {#rustls}

[![rustls][c~rustls~docs~badge]][c~rustls~docs] [![rustls~crates.io][c~rustls~crates.io~badge]][c~rustls~crates.io] [![rustls~repo][c~rustls~repo~badge]][c~rustls~repo] [![rustls~lib.rs][c~rustls~lib.rs~badge]][c~rustls~lib.rs]{{hi:rustls}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

[`rustls`][c~rustls~docs]↗{{hi:rustls}} is a portable, pure-Rust TLS implementation for clients and servers. The client below loads Mozilla's WebPKI roots, verifies the server certificate and name, and connects to the host in `TLS_SERVER` (port 443 by default). Set `TLS_SERVER` to run a live connection; without it, the example exits without network access. Its test uses a local loopback server and a temporary certificate.

The crate's default provider is `aws-lc-rs`; this example selects `ring` explicitly. Provider choice affects supported platforms and cryptographic capabilities. Keep only the provider features your dependency graph needs, and configure one explicitly when more than one provider is enabled.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/tls/rustls.rs:example}}
```

See also: [Rustls (Wikipedia)][c~rustls~wikipedia]↗.

## `native-tls` {#native-tls}

[![native-tls][c~native-tls~docs~badge]][c~native-tls~docs] [![native-tls~crates.io][c~native-tls~crates.io~badge]][c~native-tls~crates.io] [![native-tls~repo][c~native-tls~repo~badge]][c~native-tls~repo] [![native-tls~lib.rs][c~native-tls~lib.rs~badge]][c~native-tls~lib.rs]{{hi:native-tls}}

[`native-tls`][c~native-tls~docs]↗{{hi:native-tls}} provides a cross-platform API over the platform's TLS implementation. It uses SChannel on Windows, Secure Transport on macOS, and OpenSSL on most other platforms.

The example builds a connector without network access by default. Set `NATIVE_TLS_HOST` to a hostname to opt in to a verified TLS handshake on port 443.

Choose [`native-tls`][c~native-tls~docs]↗{{hi:native-tls}} when integration with the system's TLS configuration and certificate store is important. With [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, enable the `native-tls` Cargo feature; its default TLS backend is Rustls. For lower-level Hyper clients, use a connector such as [`hyper-tls`][c~hyper-tls~docs]↗{{hi:hyper-tls}} or [`hyper-rustls`][c~hyper-rustls~docs]↗{{hi:hyper-rustls}}.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/tls/native_tls.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Certificates](certificates.md).
- [Network Programming][p~server].
- [[web-programming_http-client | Web Programming HTTP Client]].
- [[web-programming_http-server | Web Programming HTTP Server]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
