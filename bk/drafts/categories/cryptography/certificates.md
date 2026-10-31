<!-- TODO Reviewed ??-??-2026 -->

# Create Certificates

{{#include certificates.incl.md}}

## Create a self-signed certificate with `rcgen` {#rcgen}

[![rcgen][c~rcgen~docs~badge]][c~rcgen~docs]{{hi:rcgen}}{{hi:Certificate}}{{hi:X.509}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

Use [`rcgen`][c~rcgen~docs]↗{{hi:rcgen}} to create certificates for local development and tests. The example creates a self-signed `localhost` certificate; clients will reject it unless they are deliberately configured to trust it.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/certs/rcgen.rs:example}}
```

## DER {#der}

[![der~website][c~der~website~badge]][c~der~website] {{#badges der}} {{hi:Crypto}} {{hi:Asn1}} {{hi:Pkcs}} {{hi:Itu}} {{#categories cryptography=Cryptography,data-structures="Data structures",encoding=Encoding,parser-implementations="Parser implementations",no-std="No standard library"}}

[`der`][c~der~docs]↗{{hi:der}} is a pure Rust embedded-friendly implementation of the Distinguished [Encoding][p~encoding] Rules (DER) for Abstract Syntax Notation One (ASN.1) as described in ITU X.690 with full support for heapless [`#![no_std]`][book~rust-reference~no_std] targets. DER (Distinguished Encoding Rules) certificates are a binary format for X.509 digital certificates, commonly used for representing cryptographic keys and identities. X.509 certificates bind a public key to an identity (e.g., a name, an email address, or a domain name). This binding is validated by a trusted third party, known as a Certificate Authority (CA).

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/certs/der.rs:example}}
```

## `pem-rfc7468` {#pem-rfc7468}

[![pem-rfc7468~website][c~pem-rfc7468~website~badge]][c~pem-rfc7468~website] {{#badges pem-rfc7468}} {{hi:Crypto}} {{hi:Key}} {{hi:Rsa}} {{hi:Pem}} {{hi:Pkcs}} {{#categories cryptography=Cryptography,data-structures="Data structures",encoding=Encoding,parser-implementations="Parser implementations",no-std="No standard library"}}

PEM (Privacy Enhanced Mail) is a text-based format (Base64 encoded DER) commonly used for cryptographic keys, certificates, and other data structures.

[`pem-rfc7468`][c~pem-rfc7468~docs]↗{{hi:pem-rfc7468}} implements PEM Encoding (RFC 7468) for PKIX, PKCS, and CMS Structures, implementing a strict subset of the original Privacy-Enhanced Mail encoding intended specifically for use with cryptographic keys, certificates, and other messages. It provides a [`#![no_std]`][book~rust-reference~no_std]-friendly, constant-time implementation suitable for use with cryptographic private keys.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/certs/pem_rfc7468.rs:example}}
```

## Read a PEM certificate file {#read-pem-file}

[![pem-rfc7468][c~pem-rfc7468~docs~badge]][c~pem-rfc7468~docs]{{hi:pem-rfc7468}}{{hi:PEM}}{{hi:File}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

Read a file as bytes, decode its RFC 7468 block, and check the label before passing the DER bytes to an X.509 parser. Decoding PEM does not verify a certificate or make it trusted.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/certs/read_pem.rs:example}}
```

## `pkcs8` {#pkcs8}

[![pkcs8~website][c~pkcs8~website~badge]][c~pkcs8~website] {{#badges pkcs8}} {{hi:Crypto}} {{hi:Key}} {{hi:Pkcs}} {{hi:Private}} {{#categories cryptography=Cryptography,data-structures="Data structures",encoding=Encoding,parser-implementations="Parser implementations",no-std="No standard library"}}

PKCS#8 ("Public-Key Cryptography Standards (PKCS) #8": Private-Key Information Syntax Specification - RFC 5208) is a standard syntax for storing private key information, including both private keys and optional attributes, in a secure format. [`pkcs8`][c~pkcs8~docs]↗{{hi:pkcs8}} is a pure Rust implementation thereof, with additional support for PKCS#8v2 asymmetric key packages (RFC 5958).

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/certs/pkcs8.rs:example}}
```

## Parse an X.509 certificate {#parse-x509}

[![x509-cert][c~x509-cert~docs~badge]][c~x509-cert~docs]{{hi:x509-cert}}{{hi:X.509}}{{hi:Parser}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:Cryptography}}

[`x509-cert`][c~x509-cert~docs]↗{{hi:x509-cert}} decodes DER certificate structures. Parsing checks the encoding, not the certificate signature, trust chain, dates, hostname, or revocation policy.

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/certs/parse_x509.rs:example}}
```

## Build X.509 certificate fields with `x509-cert` {#x509-cert}

[![x509-cert~website][c~x509-cert~website~badge]][c~x509-cert~website] {{#badges x509-cert}} {{hi:Crypto}} {{#categories cryptography=Cryptography,data-structures="Data structures",encoding=Encoding,no-std="No standard library"}}

The existing example builds and encodes selected certificate fields; it does not create or sign a complete certificate. [`x509-cert`][c~x509-cert~docs]↗{{hi:x509-cert}} implements the X.509 Public Key Infrastructure Certificate format described in [RFC 5280][rfc~5280].

```rust,editable
{{#include ../../../crates/cats/cryptography/examples/certs/x509_cert.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[signature | Signature]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
