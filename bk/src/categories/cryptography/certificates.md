# Create certificates

{{#include certificates.incl.md}}

Cryptographic certificates, also known as digital certificates, are electronic credentials used to establish the identity of entities such as individuals, websites, or organizations in online communications. They are issued by trusted entities called Certificate Authorities (CAs) and contain a public key, the owner's identity, and the CA's digital signature. When a certificate is presented, it allows the recipient to verify the identity of the sender and securely exchange information using the public key. These certificates are fundamental to protocols like SSL/TLS, which ensure secure communication over the internet.

For more formats, see Rust Crypto Formats.

- der{{hi:der}}.
- pem-rfc7468{{hi:pem-rfc7468}}.
- pkcs8{{hi:pkcs8}}.
- x509-cert{{hi:x509-cert}}.

## DER {#der}

[![der-website][c-der-website-badge]][c-der-website] [![der][c-der-badge]][c-der] [![der-crates.io][c-der-crates.io-badge]][c-der-crates.io] [![der-github][c-der-github-badge]][c-der-github] [![der-lib.rs][c-der-lib.rs-badge]][c-der-lib.rs]{{hi:der}}{{hi:Crypto}}{{hi:Asn1}}{{hi:Pkcs}}{{hi:Itu}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

[`der`][c-der]⮳{{hi:der}} is a pure Rust embedded-friendly implementation of the Distinguished [Encoding][p-encoding] Rules (DER) for Abstract Syntax Notation One (ASN.1) as described in ITU X.690 with full support for heapless `no_std` targets. DER (Distinguished Encoding Rules) certificates are a binary format for X.509 digital certificates, commonly used for representing cryptographic keys and identities. X.509 certificates bind a public key to an identity (e.g., a name, an email address, or a domain name). This binding is validated by a trusted third party, known as a Certificate Authority (CA).

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/certs/der.rs:example}}
```

## `pem-rfc7468` {#pem-rfc7468}

[![pem-rfc7468-website][c-pem_rfc7468-website-badge]][c-pem_rfc7468-website] [![pem-rfc7468][c-pem_rfc7468-badge]][c-pem_rfc7468] [![pem-rfc7468-crates.io][c-pem_rfc7468-crates.io-badge]][c-pem_rfc7468-crates.io] [![pem-rfc7468-github][c-pem_rfc7468-github-badge]][c-pem_rfc7468-github] [![pem-rfc7468-lib.rs][c-pem_rfc7468-lib.rs-badge]][c-pem_rfc7468-lib.rs]{{hi:pem-rfc7468}}{{hi:Crypto}}{{hi:Key}}{{hi:Rsa}}{{hi:Pem}}{{hi:Pkcs}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

PEM (Privacy Enhanced Mail) is a text-based format (Base64 encoded DER) commonly used for cryptographic keys, certificates, and other data structures.

[`pem-rfc7468`][c-pem_rfc7468]⮳{{hi:pem-rfc7468}} implements PEM Encoding (RFC 7468) for PKIX, PKCS, and CMS Structures, implementing a strict subset of the original Privacy-Enhanced Mail encoding intended specifically for use with cryptographic keys, certificates, and other messages. It provides a `no_std`-friendly, constant-time implementation suitable for use with cryptographic private keys.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/certs/pem_rfc7468.rs:example}}
```

## `pkcs8` {#pkcs8}

[![pkcs8-website][c-pkcs8-website-badge]][c-pkcs8-website] [![pkcs8][c-pkcs8-badge]][c-pkcs8] [![pkcs8-crates.io][c-pkcs8-crates.io-badge]][c-pkcs8-crates.io] [![pkcs8-github][c-pkcs8-github-badge]][c-pkcs8-github] [![pkcs8-lib.rs][c-pkcs8-lib.rs-badge]][c-pkcs8-lib.rs]{{hi:pkcs8}}{{hi:Crypto}}{{hi:Key}}{{hi:Pkcs}}{{hi:Private}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

PKCS#8 ("Public-Key Cryptography Standards (PKCS) #8": Private-Key Information Syntax Specification - RFC 5208) is a standard syntax for storing private key information, including both private keys and optional attributes, in a secure format. [`pkcs8`][c-pkcs8]⮳{{hi:pkcs8}} is a pure Rust implementation thereof, with additional support for PKCS#8v2 asymmetric key packages (RFC 5958).

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/certs/pkcs8.rs:example}}
```

## `x509-cert` {#x509-cert}

[![x509-cert-website][c-x509_cert-website-badge]][c-x509_cert-website] [![x509-cert][c-x509_cert-badge]][c-x509_cert] [![x509-cert-crates.io][c-x509_cert-crates.io-badge]][c-x509_cert-crates.io] [![x509-cert-github][c-x509_cert-github-badge]][c-x509_cert-github] [![x509-cert-lib.rs][c-x509_cert-lib.rs-badge]][c-x509_cert-lib.rs]{{hi:x509-cert}}{{hi:Crypto}} [![cat-cryptography][cat-cryptography-badge]][cat-cryptography]{{hi:Cryptography}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

An X.509 certificate is a standardized format for public key certificates used in various internet protocols, including SSL/TLS, to secure communications over networks. These certificates contain information about the certificate holder (such as a website or individual), their public key, the issuing Certificate Authority (CA), and a digital [signature][p-signature] from the CA. They help verify the identity of entities and establish encrypted connections, ensuring the confidentiality and integrity of the data being exchanged. [`x509-cert`][c-x509_cert]⮳{{hi:x509-cert}} is a pure Rust implementation of the X.509 Public Key Infrastructure Certificate format as described in RFC 5280.

```rust,editable
{{#include ../../../crates/cats/cryptography/tests/certs/x509_cert.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 write

[[signature | Signature]]
</div>
