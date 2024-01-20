## Perform checked date and time calculations

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Calculates and displays the date and time two weeks from now using
[`DateTime::checked_add_signed`] and the date of the day before that using
[`DateTime::checked_sub_signed`]. The methods return None if the date and time
cannot be calculated.

Escape sequences that are available for the
[`DateTime::format`] can be found at [`chrono::format::strftime`].

```rust,editable
{#include ../../../deps/examples/checked.rs}
```

[`chrono::format::strftime`]: https://docs.rs/chrono/*/chrono/format/strftime/index.html
[`DateTime::checked_add_signed`]: https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_add_signed
[`DateTime::checked_sub_signed`]: https://docs.rs/chrono/*/chrono/struct.Date.html#method.checked_sub_signed
[`DateTime::format`]: https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.format
