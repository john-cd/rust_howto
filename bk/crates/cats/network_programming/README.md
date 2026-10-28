<!-- TODO Reviewed ??-??-2026 -->

# Network Programming

Network programming examples covering Axum servers, ngrok tunneling, and load balancing with Pingora. It demonstrates integration with key ecosystem crates including `anyhow`, `axum`, `clap`, `ngrok`, `tokio`, `url`.

## Examples Covered

- **Reverse Proxy** (`examples/reverse_proxy/`): Demonstrates concepts like `ngrok`, `pingora`.
- **Server** (`examples/server/`): Demonstrates concepts like `glommio`, `listen_unused`.

## Usage

To check or run examples in this crate:

```console
cargo check --package network_programming --examples
cargo test --package network_programming
```

Code examples correspond to chapters in `bk/src/categories/network_programming` or related book sections.
