<!-- TODO Reviewed ??-??-2026 -->

# Reverse Proxies and Ingress

{{#include reverse_proxy.incl.md}}

## `rathole` {#rathole}

{{#badges rathole}}

[`rathole`][c~rathole~repo]↗{{hi:rathole}} is a Rust TCP/UDP tunnel for exposing a service behind NAT through a server with a public IP. It forwards connections at the transport layer; it does not inspect HTTP paths or rewrite HTTP headers. Use it to connect a private service to a public relay, then put an HTTP reverse proxy in front when you need host or path routing.

The server and client use matching service names and tokens. The token is stored literally in each TOML configuration file; Rathole does not expand shell variables inside TOML strings. The default TCP transport does not encrypt traffic. This example enables Rathole's Noise transport so the client authenticates the server and the tunnel traffic is encrypted. Generate a unique key pair and service token, keep the private key and token in files readable only by the service account, and do not commit real credentials.

Generate the Noise key pair on a trusted machine with this command. Keep the private key on the server and copy its public key to the client configuration:

```bash
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/rathole_keygen.sh}}
```

On the public server, set the listener address, Noise private key, service token, and relay port in `server.toml`:

```toml
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/rathole_server.toml}}
```

The sample relay service binds to loopback so it can sit behind a local HTTP proxy. Change its `bind_addr` to `0.0.0.0:5202` only when clients should connect to that forwarded port directly, and restrict access with the server firewall.

Run the server with the explicit server mode:

```bash
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/rathole_run_server.sh}}
```

On the device behind NAT, configure the server address, its public Noise key, the matching service token, and the local service address in `client.toml`:

```toml
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/rathole_client.toml}}
```

Run the client on the private device:

```bash
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/rathole_run_client.sh}}
```

With the sample addresses, traffic accepted by the server on `127.0.0.1:5202` reaches the private service at `127.0.0.1:3000`. Rathole can also infer client or server mode when a configuration contains only one mode block; the explicit flags make the role clear in service scripts.

## `ngrok` {#ngrok}

[![ngrok][c~ngrok~docs~badge]][c~ngrok~docs]{{hi:ngrok}}{{hi:ngrok-rust}}
[![ngrok~crates.io][c~ngrok~crates.io~badge]][c~ngrok~crates.io]
[![ngrok~repo][c~ngrok~repo~badge]][c~ngrok~repo]
[![ngrok~lib.rs][c~ngrok~lib.rs~badge]][c~ngrok~lib.rs]

Use [ngrok][c~ngrok~website]↗ when a managed ingress service is appropriate. Its Rust crate can expose a local HTTP application through an ngrok endpoint; the included example reads the authentication token from `NGROK_AUTHTOKEN`.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/ngrok.rs:example}}
```

## `nginx` {#nginx}

For a conventional HTTP reverse proxy, [`nginx`][nginx~website]↗{{hi:nginx}} and other standalone web servers route public requests to local upstream services. Use this when you need a configured edge proxy without writing a Rust service.

## `pingora` {#pingora}

{{#badges pingora asynchronous=Asynchronous network-programming="Network programming"}}

[`Pingora`][c~pingora~docs]↗{{hi:pingora}} is a Rust framework for programmable HTTP/1 and HTTP/2 proxies, including gRPC and WebSocket traffic. It fits custom proxy logic, load balancing, and observability needs; it is a service framework rather than a ready-made reverse-proxy binary. Linux is its primary supported environment, Unix-like platforms receive best-effort support, and Windows support is preliminary.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/pingora.rs:example}}
```

### Related Topics {#related-topics .skip}

- [Pingora quick start][c~pingora~quick-start~repo]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
