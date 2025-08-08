# Internationalization

{{#include internationalization.incl.md}}

- [`gettext`][c~gettext~docs]↗{{hi:gettext}} is a popular crate for using 'gettext', a widely used localization system. It's a good choice if you're already familiar with gettext or need its specific features.
- [`fluent`][c~fluent~docs]↗{{hi:fluent}} is a localization system developed by Mozilla. The [`fluent-rs`][c~fluent~docs]↗{{hi:fluent-rs}} crate provides bindings to Fluent. It is often a preferred option for new projects.
- [`intl-rs`][c~intl-rs~docs]↗{{hi:intl-rs}} provides some internationalization utilities, but it's not a full localization solution on its own.

## Internationalize Your App {#internationalization}

```rust,editable
{{#include ../../../crates/cats/internationalization/examples/internationalization/internationalization1.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/401)
</div>
