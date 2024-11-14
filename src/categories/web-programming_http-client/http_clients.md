# HTTP clients

{{#include http_clients.incl.md}}{{hi:HTTP client}}

## `reqwest` {#reqwest}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}}
[![reqwest-crates.io][c-reqwest-crates.io-badge]][c-reqwest-crates.io]
[![reqwest-github][c-reqwest-github-badge]][c-reqwest-github]
[![reqwest-lib.rs][c-reqwest-lib.rs-badge]][c-reqwest-lib.rs]

Full-fat HTTP client. Can be used in both synchronous and asynchronous code. Requires tokio runtime.

## `ureq` {#ureq}

[![ureq][c-ureq-badge]][c-ureq]{{hi:ureq}}
[![ureq-crates.io][c-ureq-crates.io-badge]][c-ureq-crates.io]
[![ureq-github][c-ureq-github-badge]][c-ureq-github]
[![ureq-lib.rs][c-ureq-lib.rs-badge]][c-ureq-lib.rs]
[![cat-web-programming::http-client][cat-web-programming::http-client-badge]][cat-web-programming::http-client]{{hi:HTTP client}}

Minimal synchronous HTTP client focussed on simplicity and minimising dependencies.

## `hyper` {#hyper}

[![hyper][c-hyper-badge]][c-hyper]{{hi:hyper}}{{hi:HTTP client}}{{hi:HTTP server}}
[![hyper-crates.io][c-hyper-crates.io-badge]][c-hyper-crates.io]
[![hyper-github][c-hyper-github-badge]][c-hyper-github]
[![hyper-lib.rs][c-hyper-lib.rs-badge]][c-hyper-lib.rs]

A low-level HTTP implementation (both client and server). Implements HTTP/1, and HTTP/2. Works best with the tokio async runtime, but can support other runtimes.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO expand
</div>
