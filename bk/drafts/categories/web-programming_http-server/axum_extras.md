<!-- TODO Reviewed ??-??-2026 -->

# Axum Add-ons

{{#include axum_extras.incl.md}}

Small additions that most real [`axum`][c~axum~docs]↗{{hi:axum}} servers need, beyond routing and extractors. The recipes share one example crate; each is tested without opening a network port, except the TLS test, which binds to a loopback port.

## Extract Typed Headers {#typed-header}

[![axum-extra][c~axum-extra~docs~badge]][c~axum-extra~docs]{{hi:axum-extra}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

[`axum-extra`][c~axum-extra~docs]↗{{hi:axum-extra}} re-exports the [`headers`][c~headers~docs]↗{{hi:headers}} crate's strongly typed headers. `TypedHeader<Authorization<Bearer>>` and `TypedHeader<UserAgent>` parse the request for you; a missing or malformed header is rejected with `400 Bad Request` before your handler runs.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/axum_extras/typed_header.rs:example}}
```

The tests send valid, missing, and malformed headers through the router with `oneshot`. Enable the `typed-header` feature of `axum-extra`.

## Redirect HTTP to HTTPS {#https-redirect}

[![axum-extra][c~axum-extra~docs~badge]][c~axum-extra~docs]{{hi:axum-extra}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

Run a second, tiny server on the plain HTTP port whose only job is to send browsers to the HTTPS site. The `Host` extractor supplies the requested host; the pure function `make_https` swaps the scheme and port, so the rewriting is unit-tested without a server. The response is a permanent redirect (`308`).

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/axum_extras/https_redirect.rs:example}}
```

The example reads the `Host` header with `TypedHeader<headers::Host>`. This is the direct request header; it does not resolve `Forwarded`, `X-Forwarded-Host`, or the request URI authority as the deprecated `axum_extra::extract::Host` extractor did. Behind a trusted proxy, parse forwarding headers only after configuring which proxy headers to trust. Only redirect to hosts you serve: validate the host before building a redirect target.

## Serve over HTTPS with `axum-server` {#axum-server-tls}

[![axum-server][c~axum-server~docs~badge]][c~axum-server~docs]{{hi:axum-server}} [![rcgen][c~rcgen~docs~badge]][c~rcgen~docs]{{hi:rcgen}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

[`axum-server`][c~axum-server~docs]↗{{hi:axum-server}} serves a router with TLS through [`rustls`][c~rustls~docs]↗{{hi:rustls}}. `RustlsConfig::from_pem_file(cert, key)` loads a certificate chain and private key from disk (`from_pem` takes bytes); `bind_rustls` then serves the router. A `Handle` lets you learn the bound address with `listening()` and stop the server with `graceful_shutdown`, which gives in-flight requests a grace period to finish.

The test generates a throwaway self-signed certificate with [`rcgen`][c~rcgen~docs]↗{{hi:rcgen}}, serves on an ephemeral port, trusts that certificate in a [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} client, and checks a request. Use a certificate from a real certificate authority in production, for example one issued automatically by ACME.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs:example}}
```

`rustls` needs a process-wide crypto provider; install one at start-up as the test does. See [[certificates | Certificates]].

## Show One-time Flash Messages {#flash-messages}

[![axum-messages][c~axum-messages~docs~badge]][c~axum-messages~docs]{{hi:axum-messages}} [![tower-sessions][c~tower-sessions~docs~badge]][c~tower-sessions~docs]{{hi:tower-sessions}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

A flash message survives exactly one redirect: a handler records "Settings saved", redirects, and the next page displays and consumes it. [`axum-messages`][c~axum-messages~docs]↗{{hi:axum-messages}} stores the messages in a [`tower-sessions`][c~tower-sessions~docs]↗{{hi:tower-sessions}} session, so the session layer must wrap the `MessagesManagerLayer`. Use a persistent session store in production.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/axum_extras/flash_messages.rs:example}}
```

The test follows the redirect by hand, replaying the session cookie, and checks that the message appears once and is empty on the second read.

## Set and Read Cookies {#cookies}

[![tower-cookies][c~tower-cookies~docs~badge]][c~tower-cookies~docs]{{hi:tower-cookies}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

[`tower-cookies`][c~tower-cookies~docs]↗{{hi:tower-cookies}} is a cookie manager middleware: add `CookieManagerLayer` to the router, then take `Cookies` as an extractor to `get` or `add` cookies. The example remembers a language choice.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/axum_extras/cookies.rs:example}}
```

Set `HttpOnly`, `Secure`, and `SameSite` as appropriate. [`axum-extra`][c~axum-extra~docs]↗{{hi:axum-extra}} offers a similar `CookieJar` extractor, with signed and private (encrypted) variants for tamper-proof or confidential values; never put secrets in a plain cookie.

## Debug Handler Errors with `#[debug_handler]` {#debug-handler}

[![axum-macros][c~axum-macros~docs~badge]][c~axum-macros~docs]{{hi:axum-macros}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

When a function is not a valid handler, the compiler reports a long error about unsatisfied `Handler` trait bounds. [`axum-macros`][c~axum-macros~docs]↗{{hi:axum-macros}}' `#[debug_handler]` attribute (re-exported as `axum::debug_handler` with axum's `macros` feature) checks the signature and points at the actual problem.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/axum_extras/debug_handler.rs:example}}
```

For example, this handler does not compile, because `String` consumes the request body and must therefore be the last argument. With the attribute, the error says so:

```rust,compile_fail,noplayground
# use axum::debug_handler;
# use axum::extract::State;
# #[derive(Clone)]
# struct AppState {
#     greeting: &'static str,
# }
#[debug_handler]
async fn bad(body: String, State(state): State<AppState>) -> String {
    format!("{body}{}", state.greeting)
}
```

Remove the attribute once the handler compiles; it only improves diagnostics. Async handlers that hold a non-`Send` value across an `.await` are another common culprit that it reports clearly.

## Related Topics {#related-topics .skip}

- [[axum | Axum]].
- [[middleware | Middleware]].
- [[session_login | Session Login]].
- [[web-programming_http-server | HTTP Servers]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[written for issue #1939](https://github.com/john-cd/rust_howto/issues/1939)

[written for issue #1910](https://github.com/john-cd/rust_howto/issues/1910)

</div>
