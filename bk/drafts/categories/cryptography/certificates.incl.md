<!-- TODO Reviewed ??-??-2026 -->

An [X.509 certificate][rfc~5280] binds a subject's public key to identity and
usage claims. It contains an issuer, a validity interval, a signature, and
extensions such as subject alternative names, key usage, and basic constraints.

A certificate is not trusted just because it parses or arrives from a peer. A
verifier checks a certification path to a configured trust anchor, certificate
signatures and dates, permitted uses, and the requested service identity (for
TLS, usually a DNS name in the subject alternative name). Applications may also
need explicit revocation and policy checks. Trust anchors are a local policy
choice: [`webpki-roots`][c~webpki-roots~docs] bundles Mozilla's root set into
the program, so root updates require an application update;
[`rustls-native-certs`][c~rustls-native-certs~docs] loads roots from the
platform store. The Rustls project currently recommends
[`rustls-platform-verifier`][c~rustls-platform-verifier~docs] for many deployed
applications.

DER is the binary encoding used for certificates. PEM wraps DER bytes in
Base64 text and labeled boundaries as specified by [RFC 7468][rfc~7468]; PEM
is neither encryption nor a trust decision. Keep the private key separate
from the certificate, protect it, and use self-signed certificates only for
local development or tests unless you have deliberately configured a private
trust anchor.

| Recipe                                                       | Crates                 | Categories                                |
| ------------------------------------------------------------ | ---------------------- | ----------------------------------------- |
| [Create a self-signed certificate][ex~cryptography~rcgen]    | {{#crate rcgen}}       | {{#categories cryptography=Cryptography}} |
| [DER][ex~cryptography~der]                                   | {{#crate der}}         | {{#categories cryptography=Cryptography}} |
| [Read a PEM certificate file][ex~cryptography~read-pem-file] | {{#crate pem-rfc7468}} | {{#categories cryptography=Cryptography}} |
| [`pem-rfc7468`][ex~cryptography~pem-rfc7468]                 | {{#crate pem-rfc7468}} | {{#categories cryptography=Cryptography}} |
| [Parse an X.509 certificate][ex~cryptography~parse-x509]     | {{#crate x509-cert}}   | {{#categories cryptography=Cryptography}} |
| [`pkcs8`][ex~cryptography~pkcs8]                             | {{#crate pkcs8}}       | {{#categories cryptography=Cryptography}} |
| [`x509-cert`][ex~cryptography~x509-cert]                     | {{#crate x509-cert}}   | {{#categories cryptography=Cryptography}} |
