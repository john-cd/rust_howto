<!-- TODO Reviewed ??-??-2026 -->

# Localization

{{#include localization.incl.md}}

## Localize Your App {#localization}

Localization (l10n) adapts software for a particular language and locale. Keep translated messages separate from application logic, and let the interface select complete messages for the user's locale.

## Write Messages with Fluent

[Project Fluent][c~fluent~website]↗{{hi:Fluent}} keeps each message as a translatable resource. Its FTL format supports variables and language-specific variants, including plurals.

For example, an `en-US.ftl` resource can define a greeting and a plural message:

```fluent
greeting = Hello, { $name }!
items = { $count ->
    [one] You have one item.
   *[other] You have { $count } items.
}
```

The same message identifiers can have different wording and plural rules in `fr.ftl`:

```fluent
greeting = Bonjour, { $name } !
items = { $count ->
    [one] Vous avez { $count } article.
   *[other] Vous avez { $count } articles.
}
```

Pass values such as a person's name or an item count to the message formatter. Do not build a sentence by concatenating translated fragments: word order and grammar vary by language.

## Load Translation Resources

Use [`fluent-bundle`][c~fluent-bundle~docs]↗{{hi:fluent-bundle}} when the application should load `.ftl` files at runtime and manage message lookup directly. [`fluent-templates`][c~fluent-templates~docs]↗{{hi:fluent-templates}} provides a `static_loader!` macro that reads Fluent resources at compile time and makes them available through a static loader. For embedded catalogs with runtime locale selection, [`i18n-embed`][c~i18n-embed~docs]↗{{hi:i18n-embed}} works with [`rust-embed`][c~rust-embed~docs]↗{{hi:rust-embed}} to package localization assets with the application.

## Select a Locale

[`sys-locale`][c~sys-locale~docs]↗{{hi:sys-locale}} can report the locale preferences provided by the operating system. Treat those values as preferences: parse them as language identifiers, then use [`fluent-langneg`][c~fluent-langneg~docs]↗{{hi:fluent-langneg}} to match them against the locales your application actually provides. Define a fallback locale and let users choose a different language when that is appropriate for the application.

The included example uses inline resources for a fixed English/French comparison, so it runs without system-locale or network dependencies. It formats greetings and singular/plural counts with `fluent-bundle`; use [`unic-langid`][c~unic-langid~docs]↗{{hi:unic-langid}} to parse or construct locale identifiers in an application. For locale-sensitive number and date formatting with ICU4X, see [[internationalization | Internationalization]] and [issue #796](https://github.com/john-cd/rust_howto/issues/796).

```rust,editable
{{#include ../../../crates/cats/localization/examples/localization/fluent.rs:example}}
```

## Use Gettext Catalogs

[`gettext-rs`][c~gettext-rs~docs]↗{{hi:gettext-rs}} is a Rust binding for the GNU gettext workflow. It is a practical alternative when a project already uses gettext tools and catalogs, commonly authored as `.po` files and compiled to `.mo` files.

## Related Topics {#related-topics .skip}

- [[internationalization | Internationalization]].
- [[gui | GUI]].
- [[web-programming | Web Programming]].
- [[accessibility | Accessibility]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
