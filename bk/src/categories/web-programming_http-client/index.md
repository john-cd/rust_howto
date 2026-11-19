<!-- TODO Reviewed ??-??-2026. -->

# HTTP Clients

[![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]

Choose a client based on the API and runtime your application needs. Use
[`ureq`][c~ureq~docs]↗ for synchronous requests, [`reqwest`][c~reqwest~docs]↗
for a higher-level client with blocking and asynchronous APIs, or
[`hyper`][c~hyper~docs]↗ when you need control over connections and HTTP
messages. The examples use local fixtures where possible; network demos that
need a public service are opt-in tests.

The request recipes cover headers and query parameters. Start with [a local HTTP service](offline_http_client.md) to exercise status handling and JSON decoding without a remote dependency. The API recipes add
JSON, pagination, and status handling. The reliability recipes show when to
limit requests, retry with backoff, and honor `Retry-After`. Use the file
transfer recipes for streams and byte ranges.

## HTTP Client Libraries

{{#include http_clients.incl.md}}

## Make HTTP Requests

{{#include requests.incl.md}}

## Call an API

{{#include apis.incl.md}}

## Rate Limit and Retry

{{#include rate_limiting_and_retries.incl.md}}

## Download and Upload

{{#include download.incl.md}}

## Related Topics

- [Asynchronous Programming](../asynchronous/index.md) for async runtimes and I/O.
- [HTTP Servers](../web-programming_http-server/json_patch.md) for server-side request handling.
- [WebSockets](../web-programming_websocket/index.md) for long-lived, bidirectional connections.
- [JSON and Serde](../encoding/serde.md) for request and response bodies.
- [URL parsing](../web-programming/url.md) for composing and validating URLs.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
