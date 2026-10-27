<!-- TODO Reviewed ??-??-2026 -->

# Session Login

{{#include session_login.incl.md}}

Cookie-based sessions remain the simplest way to authenticate people who use a web application in a browser. The server verifies a password once, stores the user ID in a server-side session, and sends the browser an opaque session cookie.

## Hash and Verify Passwords {#hash-passwords}

[![argon2][c~argon2~docs~badge]][c~argon2~docs]{{hi:argon2}} [![password-auth][c~password-auth~docs~badge]][c~password-auth~docs]{{hi:password-auth}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:authentication}} [![cat~cryptography][cat~cryptography~badge]][cat~cryptography]{{hi:cryptography}}

Never store passwords, only salted hashes made with a slow, memory-hard function. [`argon2`][c~argon2~docs]↗{{hi:argon2}} implements Argon2id, the algorithm the OWASP password storage guidance recommends first. `hash` generates a random salt and returns a self-describing PHC string (algorithm, parameters, salt, and hash) that can be stored in a single database column; `verify` parses that string and compares in constant time.

[`password-auth`][c~password-auth~docs]↗{{hi:password-auth}} is the one-line alternative: it wraps Argon2 with safe defaults, and `is_hash_obsolete` tells you when a stored hash should be re-hashed with newer parameters. See [[password_hashing | Password Hashing]] for the other algorithms.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/session_login/hashing.rs:example}}
```

## Log In Users with `axum-login` {#session-login}

[![axum-login][c~axum-login~docs~badge]][c~axum-login~docs]{{hi:axum-login}} [![tower-sessions][c~tower-sessions~docs~badge]][c~tower-sessions~docs]{{hi:tower-sessions}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:authentication}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

[`axum-login`][c~axum-login~docs]↗{{hi:axum-login}} adds user authentication and authorization to an [`axum`][c~axum~docs]↗{{hi:axum}} router on top of [`tower-sessions`][c~tower-sessions~docs]↗{{hi:tower-sessions}}. You provide three things:

- a `User` type implementing `AuthUser`, whose `session_auth_hash` ties a session to the current password hash, so changing a password invalidates existing sessions;
- a backend implementing `AuthnBackend`, which turns the submitted credentials into a user (`authenticate`) and reloads the user from the session (`get_user`);
- optionally `AuthzBackend`, which returns a user's permissions, used by `permission_required!`.

The `login_required!` layer redirects anonymous visitors to the login URL, and `permission_required!` rejects authenticated users who lack a permission. The example uses an in-memory user store and `MemoryStore` sessions; replace both with a database in a real application.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/session_login/axum_login.rs:example}}
```

The tests drive the router with `tower::ServiceExt::oneshot`, so no network port is needed. They check that an anonymous request is redirected to the login page, that a wrong password or an unknown user returns `401 Unauthorized`, that a correct password sets a session cookie that unlocks the protected route, and that the permission-guarded route is refused to a user without the permission.

### Harden the Session

- **Cookie flags.** Serve the application over HTTPS and call `SessionManagerLayer::with_secure(true)`. Keep the cookie `HttpOnly` (the default), so that scripts cannot read it. `tower-sessions` defaults to `SameSite=Strict`; relax it with `with_same_site(SameSite::Lax)` only if users must stay logged in when they arrive from links on other sites. The example sets `Secure` to `false` only so that it can run over plain HTTP in tests; see [[axum_extras | Axum Add-ons]] for an HTTP to HTTPS redirect and for TLS with `axum-server`; serving the login page only over HTTPS is what makes the `Secure` flag useful.
- **Session fixation.** `AuthSession::login` issues a new session ID after authenticating, so an ID planted before the login is useless. Do not bypass it by writing the user ID into the session yourself.
- **Timing leaks.** The example verifies a dummy hash when the username is unknown, so that the response time does not reveal which accounts exist. Return the same error for an unknown user and a wrong password.
- **CSRF.** `SameSite` cookies blunt most cross-site request forgery, but state-changing forms should also carry a CSRF token or be checked with the `Origin` header. See the next section.
- **Brute force.** Limit login attempts per account and per address, for example with a token bucket ([[rate_limiting_and_retries | Rate Limiting and Retries]] shows one for outgoing calls).

For single sign-on with an external identity provider, see OAuth 2.0 in the [[authentication | Authentication]] overview.

## Protect Forms with a CSRF Token {#csrf}

[![tower-sessions][c~tower-sessions~docs~badge]][c~tower-sessions~docs]{{hi:tower-sessions}} [![axum][c~axum~docs~badge]][c~axum~docs]{{hi:axum}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:authentication}} [![cat~web-programming::http-server][cat~web-programming::http-server~badge]][cat~web-programming::http-server]{{hi:web-programming::http-server}}

A cross-site request forgery makes a logged-in user's browser send a state-changing request, such as "change my email", that the user never intended. The browser attaches the session cookie by itself, so the cookie alone proves nothing. The synchronizer-token pattern fixes this: the server stores a random token in the session, embeds it as a hidden field in each form, and rejects any post whose field does not match. An attacker's page cannot read the token, so it cannot forge the field.

The example generates 32 random bytes with [`getrandom`][c~getrandom~docs]↗{{hi:getrandom}}, keeps the hex-encoded token in the [`tower-sessions`][c~tower-sessions~docs]↗{{hi:tower-sessions}} `Session`, renders the form, and compares the submitted token in constant time. The tests check that a valid token is accepted, and that a forged token or one from another session gets `403 Forbidden`.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/session_login/csrf.rs:example}}
```

- Check the token on every `POST`, `PUT`, `PATCH`, and `DELETE`, never on `GET`, and never put it in a URL.
- For JSON APIs called by scripts, send the token in a custom header, or require a header that browsers will not send cross-site without a CORS preflight; see [[cors | Cross-origin Resource Sharing]].
- Rotate the token when the user logs in, so it is tied to the authenticated session.

## Related Topics {#related-topics .skip}

- [[authentication | Authentication]].
- [[axum_extras | Axum Add-ons]] for flash messages, cookies, and TLS.
- [[password_hashing | Password Hashing]].
- [[web-programming_http-server | HTTP Servers]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [written for issue #1910](https://github.com/john-cd/rust_howto/issues/1910)
</div>
