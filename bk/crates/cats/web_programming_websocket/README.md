<!-- TODO Reviewed ??-??-2026. -->

# WebSockets

Run the WebSocket echo examples from `bk/crates`. All three use a local loopback connection.

```console
cargo run --locked -p web_programming_websocket --example websocket
cargo run --locked -p web_programming_websocket --example websocket_blocking
cargo run --locked -p web_programming_websocket --example websocket_async_adapter
cargo test --locked -p web_programming_websocket --examples
```
