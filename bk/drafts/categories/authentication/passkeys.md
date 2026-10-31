<!-- TODO Reviewed ??-??-2026 -->

# Passkeys (WebAuthn)

{{#include passkeys.incl.md}}

A passkey is a public-key credential that lives on the user's device or in a credential manager. The site stores only the public key; signing in means proving possession of the private key by signing a fresh challenge. There is no shared secret to phish or leak. WebAuthn is the W3C browser API behind passkeys. This recipe is the server side (the _relying party_), using [`webauthn-rs`][c~webauthn-rs~docs]↗ for all protocol verification.

## Register and sign in with passkeys {#passkeys}

{{#badges webauthn-rs}} {{hi:WebAuthn}} {{hi:passkeys}} {{hi:FIDO2}} [![webauthn-authenticator-rs][c~webauthn-authenticator-rs~docs~badge]][c~webauthn-authenticator-rs~docs]{{hi:webauthn-authenticator-rs}} [![cat~authentication][cat~authentication~badge]][cat~authentication]{{hi:Authentication}}

The vocabulary is small:

- The **relying party ID (RP ID)** is a domain, such as `example.com`. A credential is bound to it for life: the authenticator refuses to use it for another domain, which is why passkeys resist phishing. The RP ID must equal the page's host or be a registrable suffix of it.
- The **origin** (`https://example.com`) is what the browser puts in the signed client data. The server compares it with the one it expects, so a ceremony that ran on `https://login.example.com` is rejected when only `https://example.com` is configured.
- **Registration** creates a credential: the server sends a random challenge and options, the browser calls `navigator.credentials.create()`, and the server verifies the response and stores the public key. **Authentication** does the same with `navigator.credentials.get()`.
- The **challenge** must be unpredictable, tied to one request and used once. `webauthn-rs` returns it inside a state value (`PasskeyRegistration`, `PasskeyAuthentication`) that stays on the server; only the options for the browser are sent out.

```rust,editable,noplayground
{{#include ../../../crates/cats/authentication/examples/passkeys/passkeys.rs:example}}
```

The example keeps three things that the library leaves to you:

- **Single-use, expiring challenges.** State is removed from the map when a ceremony finishes, even when verification fails, so a replayed response finds nothing, and every challenge carries a deadline.
- **Credential persistence and account association.** Each account has a random user handle and a list of passkeys. Existing credentials are passed as an exclusion list so a device is not registered twice, and after sign-in `update_credential` stores the new signature counter.
- **Fixed relying party.** `WebauthnBuilder` rejects an RP ID that is not a suffix of the origin's domain.

The tests drive both sides with a software authenticator and check each rejection: a replayed assertion, an assertion that answers another challenge, an expired challenge, a page on an unrelated origin (the authenticator itself refuses), a page on another origin that the authenticator accepts but the server rejects, an RP ID that does not match the origin, and an unknown user.

Notes for real deployments:

- Browsers expose WebAuthn only on HTTPS, with `http://localhost` as the development exception. Use the real domain as RP ID in production and keep it stable: changing it orphans every credential.
- Store accounts and passkeys in a database. Keep the pending state in a server-side session or cache keyed by an unguessable ticket; serializing it needs the `danger-allow-state-serialisation` feature and must never reach the client.
- Use `Passkey` for passwordless sign-in. Use the library's other flows (security-key or attested registration, resident-key support) only when your policy requires them.
- The software authenticator is a test double. Verify the real browser and device path manually (a platform authenticator, a roaming security key and a synced passkey) before launch; that check is not part of the automated tests.
- Offer a recovery path (a second passkey, or recovery codes) so losing a device does not lock users out.

## Related Topics {#related-topics .skip}

- [[authentication | Authentication]] for choosing between sessions, tokens and federated sign-in.
- [[session_login | Session Login]] for keeping the user signed in after the passkey ceremony.
- [[oidc_authorization_code | OpenID Connect Authorization Code with PKCE]] for delegating sign-in to a provider.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
