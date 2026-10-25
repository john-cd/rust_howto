<!-- TODO Reviewed ??-??-2026 -->

# Rate Limiting and Retries

{{#include rate_limiting_and_retries.incl.md}}

APIs enforce quotas and fail transiently. A well-behaved client slows itself down before the server has to, and retries only what is safe to retry.

## Limit the Rate of Outgoing Calls {#rate-limit}

[![governor][c~governor~docs~badge]][c~governor~docs]{{hi:governor}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:web-programming::http-client}}

[`governor`][c~governor~docs]↗{{hi:governor}} implements a token bucket (the generic cell rate algorithm). Build a `Quota` such as `Quota::per_second`, create a `RateLimiter::direct`, and `await` `until_ready()` before each call: the future resolves immediately while the burst allowance lasts and then paces the calls. `check()` is the non-blocking variant, which returns an error instead of waiting.

The test uses `FakeRelativeClock`, so it advances time by hand instead of sleeping.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_client/examples/rate_limit/main.rs:example}}
```

Share one limiter between tasks by wrapping it in an `Arc`, and use a keyed limiter (`RateLimiter::keyed`) to limit per user or per host. Check the provider's documented quota and leave some headroom.

## Retry with Exponential Backoff {#retry}

[![backon][c~backon~docs~badge]][c~backon~docs]{{hi:backon}} [![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:web-programming}} [![cat~web-programming::http-client][cat~web-programming::http-client~badge]][cat~web-programming::http-client]{{hi:web-programming::http-client}}

[`backon`][c~backon~docs]↗{{hi:backon}} retries a fallible async function. `ExponentialBuilder` doubles the delay on each attempt up to a maximum, `with_jitter` adds randomness so that many clients do not retry in lockstep, and `with_max_times` bounds the attempts. `notify` is called before each wait, which is a good place to log. `backon` is the maintained choice; the older `backoff` crate is unmaintained (RUSTSEC-2025-0012).

The example treats `429 Too Many Requests` and `5xx` responses as transient and everything else as final. The tests start a local [`mockito`][c~mockito~docs]↗{{hi:mockito}} server that fails twice and then succeeds, and another that always fails to show that the retries stop.

```rust,editable,noplayground
{{#include ../../../crates/cats/web_programming_http_client/examples/retry/main.rs:example}}
```

### Retry Safely

- **Retry only idempotent calls.** `GET`, `PUT`, and `DELETE` can be repeated; a `POST` that creates something may be applied twice unless the API supports an idempotency key.
- **Honour `Retry-After`.** A `429` or `503` response may carry a `Retry-After` header, in seconds or as a date. Prefer it over the computed delay: read the header from the response, and sleep for that long before the next attempt.
- **Combine both tools.** Put the limiter inside the function being retried, so that retries also respect the quota.
- **Cap everything.** Bound the number of attempts and the total time, and surface the last error to the caller.

## Related Topics {#related-topics .skip}

- [[apis | Calling a Web API]].
- [[requests | Making Requests]].
- [[session_login | Session Login]] for limiting login attempts.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[written for issue #1911](https://github.com/john-cd/rust_howto/issues/1911)
</div>
