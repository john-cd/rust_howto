## Convert a local time to another timezone

[![chrono-badge]][chrono] [![cat-date-and-time-badge]][cat-date-and-time]

Gets the local time and displays it using [`offset::Local::now`] and then converts it to the UTC standard using the [`DateTime::from_utc`] struct method. A time is then converted using the [`offset::FixedOffset`] struct and the UTC time is then converted to UTC+8 and UTC-2.

```rust,editable
{#include ../../../deps/examples/timezone.rs}
```

[`DateTime::from_utc`]:https://docs.rs/chrono/*/chrono/struct.DateTime.html#method.from_utc
[`offset::FixedOffset`]: https://docs.rs/chrono/*/chrono/offset/struct.FixedOffset.html
[`offset::Local::now`]: https://docs.rs/chrono/*/chrono/offset/struct.Local.html#method.now
