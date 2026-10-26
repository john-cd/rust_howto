<!-- TODO Reviewed ??-??-2026 -->

# Internationalization

[![cat~internationalization][cat~internationalization~badge]][cat~internationalization]{{hi:Internationalization}}

Internationalization (i18n) is the design of software so it can be adapted to different languages and regions. Localization (l10n) is the work of adapting a product for a particular audience, including translated messages, formats, and cultural conventions. Plan for both when designing user-facing text and data.

## Choose Locale and Fallbacks

A locale identifies language and may also specify a region or script, such as en-US or zh-Hant. A user may choose a locale directly; an application can also consult operating-system preferences, environment variables, browser settings, or configuration. Treat detection as a starting preference, not an unchangeable fact, and let users override it where appropriate.

[`whoami`][c~whoami~docs]↗{{hi:whoami}} can query a user's preferred languages on supported platforms. Its values are one input to locale selection; validate and canonicalize language tags with a locale library such as [`unic-langid`][c~unic-langid~docs]↗{{hi:unic-langid}}. Define a fallback chain for missing translations instead of assuming a particular operating system setting exists.

## Translate Messages, Not Fragments

Keep user-visible messages in translation resources instead of scattering them through application logic. [`Fluent`][c~fluent~docs]↗{{hi:Fluent}} supports message patterns for variables, plural categories, and other language-specific grammar. [`gettext`][c~gettext~docs]↗{{hi:gettext}} fits projects that use gettext catalogs and its established translation workflow.

Pass typed values into complete messages so translators can reorder words and handle plural forms. Avoid assembling a sentence by concatenating translated fragments: word order and grammar vary across languages. Keep stable identifiers separate from their displayed translations.

## Format Locale-Sensitive Data

Date and time libraries such as [`chrono`][c~chrono~docs]↗{{hi:chrono}} and [`time`][c~time~docs]↗{{hi:time}} provide types and operations for dates and times. Localized display also needs locale data and formatting rules. The [`icu`][c~icu~docs]↗{{hi:icu}} crate family (ICU4X) provides locale-aware formatting and other internationalization services, including numbers, dates, calendars, and plural rules.

Use locale-aware formatting for numbers, currencies, dates, and times rather than hard-coding separators or patterns. Store machine-readable values in a stable representation and format them for display at the user interface.

## Unicode Text

Rust strings are UTF-8, but a user-perceived character may span multiple Unicode scalar values. Do not assume that byte length or scalar-value count equals displayed character count. For user-facing sorting, text boundaries, normalization, or case mapping, use Unicode-aware APIs with the intended locale and operation rather than ad hoc byte comparisons.

ICU4X exposes utilities for collation, segmentation, normalization, and locale-sensitive case mapping. Choose the specific operation required; normalization and case folding can change text and should not be applied blindly to identifiers or authenticated data.

## Test Localized Behavior

Test messages with missing translations and fallback locales, long translated strings, plural categories, dates around calendar boundaries, and right-to-left text when supported. Pseudo-localization can expose hard-coded strings and layout assumptions before all translations are available. Ask native-language reviewers to assess grammar and meaning; automated checks cannot replace that review.

## Code Examples

{{#include internationalization.incl.md}}

## Related Topics

- [[localization | Localization]] (l10n)
- [[accessibility | Accessibility]]
- [[date-and-time | Date and Time]]
- [[value-formatting | Value Formatting]]
- [[gui | GUI]]
- [[web-programming | Web Programming]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  write
  Review `whoami`
</div>
