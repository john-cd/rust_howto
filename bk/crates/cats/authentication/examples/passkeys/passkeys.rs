// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A passkey (WebAuthn) relying party built on `webauthn-rs`.
//!
//! The server never verifies signatures itself: `webauthn-rs` checks the
//! challenge, the origin, the relying party (RP) ID hash, the user-presence
//! flag, the signature and the signature counter. This module adds what the
//! library leaves to the application: one-shot, expiring challenge state and
//! credential storage tied to an account.
//!
//! A software authenticator plays the browser and the device, so the whole
//! flow runs offline. A real deployment replaces it with
//! `navigator.credentials.create()` and `navigator.credentials.get()` in the
//! browser, which must run on HTTPS (or `localhost`).

use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use webauthn_authenticator_rs::WebauthnAuthenticator;
use webauthn_authenticator_rs::softpasskey::SoftPasskey;
use webauthn_rs::Webauthn;
use webauthn_rs::WebauthnBuilder;
use webauthn_rs::prelude::CreationChallengeResponse;
use webauthn_rs::prelude::Passkey;
use webauthn_rs::prelude::PasskeyAuthentication;
use webauthn_rs::prelude::PasskeyRegistration;
use webauthn_rs::prelude::PublicKeyCredential;
use webauthn_rs::prelude::RegisterPublicKeyCredential;
use webauthn_rs::prelude::RequestChallengeResponse;
use webauthn_rs::prelude::Url;
use webauthn_rs::prelude::Uuid;

/// How long a challenge stays valid after it is issued.
const CHALLENGE_LIFETIME: Duration = Duration::from_mins(2);

/// An account: a stable, random user handle plus its registered passkeys.
///
/// The user handle (`id`) is what authenticators store. It must not reveal
/// the user name or e-mail address.
struct Account {
    /// The user handle: random and never reused.
    id: Uuid,
    /// The credentials this account can sign in with.
    passkeys: Vec<Passkey>,
}

/// A challenge in flight, usable once before `expires`.
struct Pending<T> {
    /// The account the ceremony belongs to.
    user: String,
    /// The library state that binds the challenge to this request.
    state: T,
    /// When the challenge stops being valid.
    expires: Instant,
}

/// The relying party: the `Webauthn` verifier plus in-memory storage.
///
/// Production code keeps `accounts` in a database and the pending state in a
/// server-side session or cache (serializing `PasskeyRegistration` needs the
/// `danger-allow-state-serialisation` feature, because the state contains
/// the secret challenge and must never reach the client).
struct RelyingParty {
    /// The library verifier, fixed to one RP ID and one origin.
    webauthn: Webauthn,
    /// Registered accounts by user name.
    accounts: HashMap<String, Account>,
    /// Registration ceremonies in flight, by ticket.
    registrations: HashMap<Uuid, Pending<(Uuid, PasskeyRegistration)>>,
    /// Authentication ceremonies in flight, by ticket.
    authentications: HashMap<Uuid, Pending<PasskeyAuthentication>>,
}

impl RelyingParty {
    /// Creates a relying party for `rp_id`, which must be the origin's
    /// effective domain (or a registrable suffix of it).
    fn new(rp_id: &str, origin: &Url) -> Result<Self> {
        let webauthn = WebauthnBuilder::new(rp_id, origin)?
            .rp_name("Rust How-to")
            .build()?;
        Ok(Self {
            webauthn,
            accounts: HashMap::new(),
            registrations: HashMap::new(),
            authentications: HashMap::new(),
        })
    }

    /// Starts registering a passkey for `user`; returns the ticket that the
    /// browser sends back and the options for `navigator.credentials.create`.
    fn start_registration(
        &mut self,
        user: &str,
        now: Instant,
    ) -> Result<(Uuid, CreationChallengeResponse)> {
        let account = self.accounts.get(user);
        let user_id = account.map_or_else(Uuid::new_v4, |a| a.id);
        // Excluding existing credentials stops the same device registering
        // twice.
        let existing: Option<Vec<_>> = account
            .map(|a| a.passkeys.iter().map(|p| p.cred_id().clone()).collect());
        let (options, state) = self
            .webauthn
            .start_passkey_registration(user_id, user, user, existing)?;
        let ticket = Uuid::new_v4();
        self.registrations.insert(
            ticket,
            Pending {
                user: user.to_owned(),
                state: (user_id, state),
                expires: now + CHALLENGE_LIFETIME,
            },
        );
        Ok((ticket, options))
    }

    /// Verifies the browser's answer and stores the new passkey.
    fn finish_registration(
        &mut self,
        ticket: Uuid,
        credential: &RegisterPublicKeyCredential,
        now: Instant,
    ) -> Result<()> {
        // `remove` makes the challenge single-use, even when verification
        // fails.
        let pending = self
            .registrations
            .remove(&ticket)
            .ok_or_else(|| anyhow!("unknown or already used challenge"))?;
        if now > pending.expires {
            bail!("the challenge has expired");
        }
        let (id, state) = pending.state;
        let passkey = self
            .webauthn
            .finish_passkey_registration(credential, &state)?;
        self.accounts
            .entry(pending.user)
            .or_insert_with(|| Account {
                id,
                passkeys: Vec::new(),
            })
            .passkeys
            .push(passkey);
        Ok(())
    }

    /// Starts signing `user` in; returns a ticket and the options for
    /// `navigator.credentials.get`.
    fn start_authentication(
        &mut self,
        user: &str,
        now: Instant,
    ) -> Result<(Uuid, RequestChallengeResponse)> {
        let account = self
            .accounts
            .get(user)
            .ok_or_else(|| anyhow!("unknown user"))?;
        let (options, state) = self
            .webauthn
            .start_passkey_authentication(&account.passkeys)?;
        let ticket = Uuid::new_v4();
        self.authentications.insert(
            ticket,
            Pending {
                user: user.to_owned(),
                state,
                expires: now + CHALLENGE_LIFETIME,
            },
        );
        Ok((ticket, options))
    }

    /// Verifies the assertion and returns the signed-in account's user name.
    fn finish_authentication(
        &mut self,
        ticket: Uuid,
        credential: &PublicKeyCredential,
        now: Instant,
    ) -> Result<String> {
        let pending = self
            .authentications
            .remove(&ticket)
            .ok_or_else(|| anyhow!("unknown or already used challenge"))?;
        if now > pending.expires {
            bail!("the challenge has expired");
        }
        let result = self
            .webauthn
            .finish_passkey_authentication(credential, &pending.state)?;
        // Persist the new signature counter: it detects cloned authenticators.
        if let Some(account) = self.accounts.get_mut(&pending.user) {
            for passkey in &mut account.passkeys {
                passkey.update_credential(&result);
            }
        }
        Ok(pending.user)
    }
}

/// Registers a passkey and signs in with it, using a software authenticator.
fn main() -> Result<()> {
    let origin = Url::parse("https://example.com")?;
    let mut rp = RelyingParty::new("example.com", &origin)?;
    let mut device = WebauthnAuthenticator::new(SoftPasskey::new(true));
    let now = Instant::now();

    let (ticket, options) = rp.start_registration("alice", now)?;
    let credential = device
        .do_registration(origin.clone(), options)
        .map_err(|e| anyhow!("{e:?}"))?;
    rp.finish_registration(ticket, &credential, now)?;

    let (ticket, options) = rp.start_authentication("alice", now)?;
    let assertion = device
        .do_authentication(origin, options)
        .map_err(|e| anyhow!("{e:?}"))?;
    let user = rp.finish_authentication(ticket, &assertion, now)?;
    println!("signed in as {user}");
    assert_eq!(user, "alice");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<()> {
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A relying party, a software device and the origin they share.
    struct Setup {
        rp: RelyingParty,
        device: WebauthnAuthenticator<SoftPasskey>,
        origin: Url,
        now: Instant,
    }

    /// Builds the fixtures without registering anything.
    fn setup() -> Setup {
        let origin = Url::parse("https://example.com").expect("valid URL");
        Setup {
            rp: RelyingParty::new("example.com", &origin).expect("valid RP"),
            device: WebauthnAuthenticator::new(SoftPasskey::new(true)),
            origin,
            now: Instant::now(),
        }
    }

    /// Registers `alice` on the device.
    fn register(s: &mut Setup) -> Result<()> {
        let (ticket, options) = s.rp.start_registration("alice", s.now)?;
        let credential = s
            .device
            .do_registration(s.origin.clone(), options)
            .map_err(|e| anyhow!("{e:?}"))?;
        s.rp.finish_registration(ticket, &credential, s.now)
    }

    /// Asks the device to answer a fresh sign-in challenge for `alice`.
    fn assertion(
        s: &mut Setup,
        origin: &Url,
    ) -> Result<(Uuid, PublicKeyCredential)> {
        let (ticket, options) = s.rp.start_authentication("alice", s.now)?;
        let assertion = s
            .device
            .do_authentication(origin.clone(), options)
            .map_err(|e| anyhow!("{e:?}"))?;
        Ok((ticket, assertion))
    }

    /// Registration followed by sign-in succeeds.
    #[test]
    fn registers_and_signs_in() -> Result<()> {
        let mut s = setup();
        register(&mut s)?;
        let origin = s.origin.clone();
        let (ticket, assertion) = assertion(&mut s, &origin)?;
        assert_eq!(
            s.rp.finish_authentication(ticket, &assertion, s.now)?,
            "alice"
        );
        Ok(())
    }

    /// A challenge answered once cannot be answered again.
    #[test]
    fn replayed_assertion_is_rejected() -> Result<()> {
        let mut s = setup();
        register(&mut s)?;
        let origin = s.origin.clone();
        let (ticket, assertion) = assertion(&mut s, &origin)?;
        s.rp.finish_authentication(ticket, &assertion, s.now)?;
        assert!(
            s.rp.finish_authentication(ticket, &assertion, s.now)
                .is_err()
        );
        Ok(())
    }

    /// An assertion for one challenge does not satisfy another.
    #[test]
    fn assertion_for_another_challenge_is_rejected() -> Result<()> {
        let mut s = setup();
        register(&mut s)?;
        let origin = s.origin.clone();
        let (_first, assertion) = assertion(&mut s, &origin)?;
        let (second, _) = s.rp.start_authentication("alice", s.now)?;
        assert!(
            s.rp.finish_authentication(second, &assertion, s.now)
                .is_err()
        );
        Ok(())
    }

    /// Challenges expire.
    #[test]
    fn expired_challenges_are_rejected() -> Result<()> {
        let mut s = setup();
        let (ticket, options) = s.rp.start_registration("alice", s.now)?;
        let credential = s
            .device
            .do_registration(s.origin.clone(), options)
            .map_err(|e| anyhow!("{e:?}"))?;
        let late = s.now + CHALLENGE_LIFETIME + Duration::from_secs(1);
        assert!(s.rp.finish_registration(ticket, &credential, late).is_err());

        register(&mut s)?;
        let origin = s.origin.clone();
        let (ticket, assertion) = assertion(&mut s, &origin)?;
        assert!(
            s.rp.finish_authentication(ticket, &assertion, late)
                .is_err()
        );
        Ok(())
    }

    /// A page from an unrelated site cannot even ask the authenticator.
    #[test]
    fn authenticator_refuses_an_unrelated_origin() -> Result<()> {
        let mut s = setup();
        let evil = Url::parse("https://evil.example")?;
        let (_ticket, options) = s.rp.start_registration("alice", s.now)?;
        assert!(s.device.do_registration(evil, options).is_err());
        Ok(())
    }

    /// A subdomain is allowed by the RP ID, but the server expects exactly
    /// the configured origin and rejects ceremonies that ran elsewhere.
    #[test]
    fn server_rejects_a_different_origin() -> Result<()> {
        let mut s = setup();
        let other = Url::parse("https://login.example.com")?;
        let (ticket, options) = s.rp.start_registration("alice", s.now)?;
        let credential = s
            .device
            .do_registration(other.clone(), options)
            .map_err(|e| anyhow!("{e:?}"))?;
        assert!(
            s.rp.finish_registration(ticket, &credential, s.now)
                .is_err()
        );

        register(&mut s)?;
        let (ticket, assertion) = assertion(&mut s, &other)?;
        assert!(
            s.rp.finish_authentication(ticket, &assertion, s.now)
                .is_err()
        );
        Ok(())
    }

    /// An RP ID that is not a suffix of the origin's domain is refused up
    /// front.
    #[test]
    fn rp_id_must_match_the_origin() {
        let origin = Url::parse("https://example.com").expect("valid URL");
        assert!(RelyingParty::new("other.example", &origin).is_err());
    }

    /// Only registered users can start signing in.
    #[test]
    fn unknown_user_cannot_sign_in() {
        let mut s = setup();
        assert!(s.rp.start_authentication("mallory", s.now).is_err());
    }

    /// The main function runs.
    #[test]
    fn example_runs() -> Result<()> {
        main()
    }
}
