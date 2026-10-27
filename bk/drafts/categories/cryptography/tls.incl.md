<!-- TODO Reviewed ??-??-2026 -->

| Recipe                                     | Crates                                                      | Categories                                                      |
| ------------------------------------------ | ----------------------------------------------------------- | --------------------------------------------------------------- |
| [`mTLS`][ex~cryptography~mtls]             | [![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]          | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [`rustls`][ex~cryptography~rustls]         | [![rustls][c~rustls~docs~badge]][c~rustls~docs]             | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [`native-tls`][ex~cryptography~native-tls] | [![native-tls][c~native-tls~docs~badge]][c~native-tls~docs] | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |

Use a maintained TLS implementation instead of assembling cryptographic primitives yourself. This chapter uses `rustls` as the portable Rust default and `native-tls` when an application should use the operating system's TLS stack. Rustls 0.23 offers the `aws-lc-rs` and `ring` crypto providers; select the provider through Cargo features and configure it explicitly when needed. Higher-level clients choose their TLS backend through Cargo features: `reqwest` 0.13 defaults to Rustls and can enable `native-tls`, while applications using Hyper directly add a TLS connector such as `hyper-rustls` or `hyper-tls`.
