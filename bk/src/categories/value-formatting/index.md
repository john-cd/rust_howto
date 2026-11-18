<!-- TODO Reviewed ??-??-2026. -->

# Value Formatting

[![cat~value-formatting][cat~value-formatting~badge]][cat~value-formatting]{{hi:Value formatting}}

Use Rust's [formatting syntax][c~std::fmt~docs] for padding, precision, and alignment. Implement `Display` when a type needs a default human-readable representation. Use a locale-aware formatter when grouping, decimal separators, or currency conventions depend on the reader's locale.

| Display task | Where to start |
| --- | --- |
| Format numbers, money stored in minor units, or byte counts | [Number Formatting](number_formatting.md) |
| Format dates, times, and durations | [Date and Time](../date-and-time/index.md) |
| Localize numbers, dates, and messages | [Internationalization](../internationalization/index.md) and [Localization](../localization/index.md) |
| Convert names between casing conventions | [String Manipulation](../text-processing/string_manipulation.md#heck) |

Decide how to display missing values. Depending on the context, omit the field, use a placeholder, or show a domain-specific label. For interchange between programs, use a documented [serialization format](../encoding/index.md) and preserve the original values.

{{#include ../../refs/link-refs.md}}
