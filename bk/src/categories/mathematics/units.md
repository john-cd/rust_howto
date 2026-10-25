<!-- TODO Reviewed ??-??-2026 -->

# Units of Measurement

{{#include units.incl.md}}

## Convert Units with `uom` {#convert-units-with-uom}

[![uom][c~uom~docs~badge]][c~uom~docs]{{hi:uom}} [![cat~science][cat~science~badge]][cat~science]

`uom` represents measurements as typed quantities. It converts values at unit boundaries and uses dimensions to prevent operations such as adding a length to a time. This example divides a distance by a duration to calculate a speed, then displays the distance in meters.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/units/quantities_and_conversions.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[trigonometry | Trigonometry]].
- [[linear_algebra | Linear Algebra]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
</div>
