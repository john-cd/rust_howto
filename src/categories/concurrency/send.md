# Send, Sync traits

{{#include send.incl.md}}{{hi:Send}}{{hi:Sync}{{hi:'static}}}

## `Send` {#send}

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/send.rs:example}}
```

## `Sync` {#sync}

```rust,editable
{{#include ../../../deps/tests/categories/concurrency/sync.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P0 add 'static and Send constraints
Send + Sync handling
</div>
