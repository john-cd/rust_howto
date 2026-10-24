<!-- TODO Reviewed ??-??-2026 -->
# Value Formatting

Format values for display, potentially adapting output to a user's language and region. Keep serialization formats separate from presentation formats: localized display text is generally not a stable interchange format.

| Task | Crates | Notes |
|---|---|---|
| Integer and floating-point conversion | [![itoa][c~itoa~docs~badge]][c~itoa~docs]↗{{hi:itoa}}, [![ryu][c~ryu~docs~badge]][c~ryu~docs]↗{{hi:ryu}} | Fast primitive-to-string conversion; both support allocation-free buffer formatting. See [[number_formatting | Number Formatting]]. |
| Number separators and locale | [`num-format`][c~num-format~docs]↗{{hi:num-format}}, [`icu`][c~icu~docs]↗{{hi:icu}} | Use locale-aware formatting when grouping and decimal separators vary by locale. |
| Dates and durations | [`chrono`][c~chrono~docs]↗{{hi:chrono}}, [`time`][c~time~docs]↗{{hi:time}}, [`humantime`][c~humantime~docs]↗{{hi:humantime}} | Format calendar values or human-readable durations. |
| Human-readable sizes | [`bytesize`][c~bytesize~docs]↗{{hi:bytesize}}, [`humansize`][c~humansize~docs]↗{{hi:humansize}} | Choose decimal or binary units and make that choice clear to users. |
| Case conversion | [`heck`][c~heck~docs]↗{{hi:heck}} | Convert identifiers between common case styles. Prefer `heck` for new code over older `Inflector` patterns. |
| Optional values | [`show-option`][c~show-option~lib.rs]↗{{hi:ShowOption}} or a small `Display` wrapper | Decide explicitly whether `None` is omitted, shown as a placeholder, or represented as a domain-specific value. |
| Dates, currency, and messages | [`icu`][c~icu~docs]↗{{hi:icu}}, [`fluent`][c~fluent~docs]↗{{hi:fluent}} | Use locale-aware APIs for dates, currencies, plurals, and translated messages. |

## Examples

{{#include number_formatting.incl.md}}

- [Convert identifiers between case styles][ex~value-formatting~case-conversion].

## Related Topics

- [[internationalization | Internationalization]].
- [[localization | Localization]].
- [[text-processing | Text Processing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: category index content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1903)
</div>
