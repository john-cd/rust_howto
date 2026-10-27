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

| Recipe                                                       | Crates                                                         | Categories                                                      |
| ------------------------------------------------------------ | -------------------------------------------------------------- | --------------------------------------------------------------- |
| [Create a self-signed certificate][ex~cryptography~rcgen]    | [![rcgen][c~rcgen~docs~badge]][c~rcgen~docs]                   | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [DER][ex~cryptography~der]                                   | [![der][c~der~docs~badge]][c~der~docs]                         | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [Read a PEM certificate file][ex~cryptography~read-pem-file] | [![pem-rfc7468][c~pem-rfc7468~docs~badge]][c~pem-rfc7468~docs] | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [`pem-rfc7468`][ex~cryptography~pem-rfc7468]                 | [![pem-rfc7468][c~pem-rfc7468~docs~badge]][c~pem-rfc7468~docs] | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [Parse an X.509 certificate][ex~cryptography~parse-x509]     | [![x509-cert][c~x509-cert~docs~badge]][c~x509-cert~docs]       | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [`pkcs8`][ex~cryptography~pkcs8]                             | [![pkcs8][c~pkcs8~docs~badge]][c~pkcs8~docs]                   | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
| [`x509-cert`][ex~cryptography~x509-cert]                     | [![x509-cert][c~x509-cert~docs~badge]][c~x509-cert~docs]       | [![cat~cryptography][cat~cryptography~badge]][cat~cryptography] |
