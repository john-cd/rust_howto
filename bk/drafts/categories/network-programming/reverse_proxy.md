# Reverse Proxies, Ingress

{{#include reverse_proxy.incl.md}}

## `rathole` {#rathole}

[![rathole][c~rathole~docs~badge]][c~rathole~docs]{{hi:rathole}}
[![rathole~crates.io][c~rathole~crates.io~badge]][c~rathole~crates.io]
[![rathole~repo][c~rathole~repo~badge]][c~rathole~repo]
[![rathole~lib.rs][c~rathole~lib.rs~badge]][c~rathole~lib.rs]

[Rathole][c~rathole~repo]↗ is a lightweight and high-performance reverse proxy for NAT traversal, written in Rust. [`rathole`][c~rathole~docs]↗{{hi:rathole}} is similar to tools like [`frp`][c~frp~docs]↗{{hi:frp}} and [`ngrok`][c~ngrok~docs]↗{{hi:ngrok}}.

NAT (Network Address Translation) can make it challenging to expose services on devices behind a NAT to the internet. Rathole helps to overcome this by allowing these services to be accessible via a [server][p~server] with a public IP.

Configuration of a service is split into the client side and the [server][p~server] side, and a token is mandatory.

- Server Setup: On a [server][p~server] with a public IP, create a `server.toml` file with the necessary configurations.

```toml
# server.toml
[server]
bind_addr = "0.0.0.0:2333" # `2333` specifies the port that `rathole` listens for clients

[server.services.my_nas_ssh]
token = "use_a_secret_that_only_you_know" # Token that is used to authenticate the client for the service. Change to an arbitrary value.
bind_addr = "0.0.0.0:5202" # `5202` specifies the port that exposes `my_nas_ssh` to the Internet
```

Run:

```sh
./rathole server.toml
```

- Client Setup: On the device behind NAT, create a `client.[toml][p~toml]` file to connect to the server.

```toml
# client.toml
[client]
remote_addr = "myserver.com:2333" # The address of the server. The port must be the same with the port in `server.bind_addr`

[client.services.my_nas_ssh]
token = "use_a_secret_that_only_you_know" # Must be the same with the server to pass the validation
local_addr = "127.0.0.1:22" # The address of the service that needs to be forwarded
```

```sh
./rathole client.toml
```

[`rathole`][c~rathole~docs]↗{{hi:rathole}} can automatically determine to run in the server mode or the client mode, according to the content of the configuration file, if only one of `[server]` and `[client]` block is present.

## `ngrok` {#ngrok}

[![ngrok][c~ngrok~docs~badge]][c~ngrok~docs]{{hi:ngrok}}{{hi:ngrok-rust}}
[![ngrok~crates.io][c~ngrok~crates.io~badge]][c~ngrok~crates.io]
[![ngrok~repo][c~ngrok~repo~badge]][c~ngrok~repo]
[![ngrok~lib.rs][c~ngrok~lib.rs~badge]][c~ngrok~lib.rs]

[ngrok][c~ngrok~website]↗ is a simplified API-first ingress-as-a-service that adds connectivity, security, and observability to your apps. `ngrok-rust` is the Rust-native crate for adding a public internet address with secure ingress traffic directly into your Rust apps.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/ngrok.rs:example}}
```

## `nginx` {#nginx}

[nginx proxy manager][nginx-proxy-manager~website]↗{{hi:nginx proxy manager}}.

## `pingora` {#pingora}

[![pingora][c~pingora~docs~badge]][c~pingora~docs]{{hi:pingora}}
[![pingora~crates.io][c~pingora~crates.io~badge]][c~pingora~crates.io]
[![pingora~repo][c~pingora~repo~badge]][c~pingora~repo]
[![pingora~lib.rs][c~pingora~lib.rs~badge]][c~pingora~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}
[![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

[`pingora`][c~pingora~docs]↗{{hi:pingora}} is a library for building fast, reliable and evolvable network services.

```rust,editable
{{#include ../../../crates/cats/network_programming/examples/reverse_proxy/pingora.rs:example}}
```

### Related Topics {#related-topics}

- [Pingora quick start][c~pingora~quick-start~repo]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[reverse_proxy: write](https://github.com/john-cd/rust_howto/issues/424)
[River][river~repo]↗.
</div>
