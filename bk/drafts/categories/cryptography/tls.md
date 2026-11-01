<!-- TODO Reviewed ??-??-2026 -->

# TLS

{{#include tls.incl.md}}

Transport Layer Security (TLS) protects the confidentiality and integrity of network traffic and authenticates the server. It is used by web browsing, [email][p~email], and other client/server protocols. TLS 1.2 and TLS 1.3 are the supported versions in the examples below; Secure Sockets Layer (SSL) is obsolete. Use a maintained TLS library and validate the server name and certificate chain.

## Connect to a Server over TLS {#rustls}

{{#badges rustls cryptography=Cryptography network-programming="Network programming"}}

[`rustls`][c~rustls~docs]↗{{hi:rustls}} is a portable, pure-Rust TLS implementation for clients and servers. The client below loads Mozilla's WebPKI roots, verifies the server certificate and name, and connects to the host in `TLS_SERVER` (port 443 by default). Set `TLS_SERVER` to run a live connection; without it, the example exits without network access. Its test uses a local loopback server and a temporary certificate.

The crate's default provider is `aws-lc-rs`; this example selects `ring` explicitly. Provider choice affects supported platforms and cryptographic capabilities. Keep only the provider features your dependency graph needs, and configure one explicitly when more than one provider is enabled.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/tls/rustls.rs:example}}
```

See also: [Rustls (Wikipedia)][c~rustls~wikipedia]↗.

## Mutual TLS with a Client Certificate {#mtls}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![rustls][c~rustls~docs~badge]][c~rustls~docs]{{hi:rustls}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

Ordinary HTTPS authenticates the server to the client. Mutual TLS (mTLS) also asks the client to present a certificate, which the server validates against a CA it trusts. This authenticates a workload or service; it does not sign a human user in or replace authorization checks. The pattern was inspired by [Rust Cookbook issue #826][rust-cookbook~issue~826]; this recipe and its test are original.

The client trusts the supplied CA certificate and sends a PEM identity bundle containing its certificate chain followed by its private key. PEM is a text encoding of DER data; this example uses a PKCS#8 private key. Keep production private keys in a secret manager or hardware-backed key store, restrict access, rotate them, and never commit or log them. The offline loopback test uses deterministic local fixtures generated in memory from test-only keys.

`tls_certs_only` replaces the client's default roots with the CA supplied here. Rustls still validates the server's certificate chain and the URL hostname; the example does not install an insecure verifier or disable either check. Redirects are disabled so the client identity is not sent to a redirect target. This crate uses reqwest's `rustls-no-provider` feature and installs `ring` only when the application has not selected a Rustls provider. Configure `MTLS_CA_CERT_PEM`, `MTLS_CLIENT_IDENTITY_PEM`, and `MTLS_URL` to make a request with the [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} client.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/tls/mtls.rs:example}}
```

## `native-tls` {#native-tls}

{{#badges native-tls}}

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
