<!-- TODO Reviewed ??-??-2026 -->

# Language Server Protocol

{{#include language_server_protocol.incl.md}}

Editors, code-review bots and IDE-like features in custom applications talk to language servers over the Language Server Protocol.

## Talk to a Language Server (`lsp-server`, `lsp-types`) {#lsp}

[![lsp-server][c~lsp-server~docs~badge]][c~lsp-server~docs] [![lsp-server~crates.io][c~lsp-server~crates.io~badge]][c~lsp-server~crates.io] [![lsp-types][c~lsp-types~docs~badge]][c~lsp-types~docs] [![lsp-types~crates.io][c~lsp-types~crates.io~badge]][c~lsp-types~crates.io]{{hi:lsp-server}}{{hi:lsp-types}} [![cat~text-editors][cat~text-editors~badge]][cat~text-editors]{{hi:Language Server Protocol}}

The Language Server Protocol (LSP) is JSON-RPC with `Content-Length` framing, used between editors and language servers such as `rust-analyzer`. [`lsp-types`][c~lsp-types~docs]↗{{hi:lsp-types}} defines the message types; [`lsp-server`][c~lsp-server~docs]↗{{hi:lsp-server}} provides the transport (`Connection::stdio()` for a real process, `Connection::memory()` for tests) and the `initialize` / `shutdown` handshakes. The same crates serve both ends: write a server, or drive an existing one from a custom tool. Responses are matched to requests by `RequestId`. Async alternatives include `tower-lsp` and `async-lsp`.

```rust,editable
{{#include ../../../crates/cats/text_editors/examples/language_server_protocol/lsp_server_minimal.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[ides | Editors and IDEs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
