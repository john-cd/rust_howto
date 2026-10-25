<!-- TODO Reviewed ??-??-2026 -->

# Cross-origin Resource Sharing

{{#include cors.incl.md}}

## Summary {#summary}

Cross-Origin Resource Sharing (CORS) is a browser protocol that lets a server opt in to sharing a response with scripts from other origins. An origin is the combination of a scheme, host, and port. A browser sends the requesting page's origin in the `Origin` header and checks the server's CORS response headers before exposing the response to JavaScript. See the [Fetch Standard's CORS protocol][fetch~cors].

CORS does not authenticate callers or prevent non-browser clients from sending requests. Keep normal authentication and authorization on the server, and use appropriate cross-site request forgery (CSRF) protections for cookie-based sessions.

## When to Configure CORS {#when-to-configure-cors}

Configure CORS on an HTTP API when browser code hosted at a different origin must read its responses. Same-origin browser requests do not need CORS, and command-line tools, native applications, and server-to-server clients do not enforce the browser's CORS checks.

Allow only the origins, methods, and request headers that the browser client needs. A wildcard origin is suitable for public, credential-free resources. If browser requests include cookies or other credentials, allow an explicit origin and enable credentials; the wildcard origin cannot be used for credentialed requests. Never reflect an arbitrary `Origin` value as allowed. If the response's CORS headers vary by origin, account for that in caches with `Vary: Origin`.

## Browser Preflight {#browser-preflight}

Some cross-origin requests can be sent directly. When a request uses a method or headers outside the browser's CORS-safelisted set, the browser first sends an `OPTIONS` preflight with `Origin`, `Access-Control-Request-Method`, and, when needed, `Access-Control-Request-Headers`. The server's response must allow the origin, method, and requested headers before the browser sends the actual request. For response headers that browser code must read but that are not safelisted, configure `Access-Control-Expose-Headers`.

## Implement CORS {#cors}

[![tower][c~tower~docs~badge]][c~tower~docs] [![tower~crates.io][c~tower~crates.io~badge]][c~tower~crates.io] [![tower~repo][c~tower~repo~badge]][c~tower~repo] [![tower~lib.rs][c~tower~lib.rs~badge]][c~tower~lib.rs]{{hi:tower}}{{hi:Async}}{{hi:Futures}}{{hi:Io}}{{hi:Non-blocking}}{{hi:Service}} [![tower-http][c~tower-http~docs~badge]][c~tower-http~docs] [![tower-http~crates.io][c~tower-http~crates.io~badge]][c~tower-http~crates.io] [![tower-http~repo][c~tower-http~repo~badge]][c~tower-http~repo] [![tower-http~lib.rs][c~tower-http~lib.rs~badge]][c~tower-http~lib.rs]{{hi:tower-http}}{{hi:Async}}{{hi:Futures}}{{hi:Http}}{{hi:Io}}{{hi:Service}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_server/examples/middleware/cors.rs:example}}
```

This example allows `GET` and `POST` from any origin and returns `Access-Control-Allow-Origin: *`. It is intentionally permissive for a local demonstration and does not enable credentials. In an application, use `CorsLayer::allow_origin` with an explicit allowlist, add only the needed methods and headers, and call `allow_credentials(true)` only when the browser client needs credentialed requests and the allowed origins are explicit. The [`tower-http` CORS module documentation][c~tower-http~cors~docs]↗ describes the layer options.

## See Also {#see-also .skip}

- [CORS (Mozilla)][mozilla~cors]↗.
- See [[middleware | Middleware]] for a running Axum server that combines CORS with tracing, compression, and a request timeout.

## Related Topics {#related-topics .skip}

- [[middleware | Middleware]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
