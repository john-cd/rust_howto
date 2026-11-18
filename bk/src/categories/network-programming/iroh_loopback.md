<!-- TODO Reviewed ??-??-2026. -->

# Connect Two Iroh Endpoints on Loopback

Iroh uses a cryptographic endpoint identity as well as a network path. This recipe binds two endpoints to `127.0.0.1`, disables relays and address lookup, and gives the client the server's identity plus loopback socket explicitly. The connection checks both peers' identities before exchanging `ping` and `pong` over a bidirectional QUIC stream.

```rust
{{#include ../../../crates/cats/network_programming/iroh_loopback/src/lib.rs:example}}
```

Run `cargo test -p iroh_loopback` from `bk/crates`. The test first tries to dial the known identity without an address and asserts a bounded failure. It then completes the round trip and closes both endpoints. There is no public relay, hosted account, DNS lookup, or discovery service in this fixture. In a deployed application, Iroh's [address lookup and relay documentation](https://docs.iroh.computer/) explains how peers find each other across networks. Share endpoint addresses only with intended peers, authenticate the identity you expect, and remember that connection metadata such as IP addresses may still be visible to the network.

[IPFS](https://ipfs.tech/) addresses content by its hash; this example addresses a live peer for a small request and response. [Ockam](https://docs.ockam.io/) provides authenticated service channels and routing at a different layer. Choose those tools according to whether the application needs content retrieval, direct peer communication, or service-channel policy.
