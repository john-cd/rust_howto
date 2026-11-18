<!-- Reviewed 2026-11-17. -->

# Localization

{{#include localization.incl.md}}

Localization adapts a product for a particular audience. Keep translated messages separate from application logic, then select complete messages for the user's locale.

## Localize Your App {#localization}

### Write Complete Messages

Store user-facing text in translation resources. Do not assemble sentences by concatenating translated fragments: word order, agreement, and punctuation vary by language. Pass values such as a person's name or an item count to the message formatter so translators can place them where each language requires.

[Project Fluent][c~fluent~website]↗{{hi:Fluent}} uses message patterns to keep variables and language-specific variants together. For example, an `en-US.ftl` resource can define a greeting and a plural message:

```fluent
greeting = Hello, { $name }!
items = { $count ->
    [one] You have one item.
   *[other] You have { $count } items.
}
```

The same message identifiers can have different wording in `fr.ftl`:

```fluent
greeting = Bonjour, { $name } !
items = { $count ->
    [one] Vous avez { $count } article.
   *[other] Vous avez { $count } articles.
}
```

Fluent selects the plural variant using the message's locale rules. Avoid assuming that a language has only singular and plural forms.

### Load Translation Resources

Use [`fluent-bundle`][c~fluent-bundle~docs]↗{{hi:fluent-bundle}} when the application should load `.ftl` resources and manage message lookup at runtime. [`fluent-templates`][c~fluent-templates~docs]↗{{hi:fluent-templates}} offers a `static_loader!` macro that embeds Fluent resources at compile time. For embedded catalogs with runtime locale selection, [`i18n-embed`][c~i18n-embed~docs]↗{{hi:i18n-embed}} works with [`rust-embed`][c~rust-embed~docs]↗{{hi:rust-embed}} to package localization assets with the application.

### Select a Locale and Fallback

[`sys-locale`][c~sys-locale~docs]↗{{hi:sys-locale}} can report locale preferences provided by the operating system. Treat those values as preferences: parse and validate them as language identifiers, then use [`fluent-langneg`][c~fluent-langneg~docs]↗{{hi:fluent-langneg}} to match them to the locales your application supports. Define a fallback locale and let users choose another language when appropriate.

The example below uses inline English and French resources, so it runs without system-locale or network dependencies. It formats greetings and singular and plural counts with `fluent-bundle`; use [`unic-langid`][c~unic-langid~docs]↗{{hi:unic-langid}} to parse or construct locale identifiers in your application. For locale-aware number and date formatting with ICU4X, see [Internationalization](../internationalization/index.md).

```rust,editable
{{#include ../../../crates/cats/localization/examples/localization/fluent.rs:example}}
```

### Use Gettext Catalogs

[`gettext-rs`][c~gettext-rs~docs]↗{{hi:gettext-rs}} is a Rust binding for the GNU gettext workflow. It is a practical option when a project already uses gettext tools and catalogs, commonly authored as `.po` files and compiled to `.mo` files.

## Related Topics {#related-topics .skip}

- [Command-line Interface](../command-line-interface/index.md).
- [Date and Time](../date-and-time/index.md).
- [Internationalization](../internationalization/index.md).
- [Localize API Errors](../web-programming_http-server/i18n.md).
- [Template Engines](../template-engine/index.md).
- [Value Formatting](../value-formatting/index.md).
- [Web Programming](../web-programming/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
