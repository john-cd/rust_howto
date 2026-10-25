<!-- TODO Reviewed ??-??-2026 -->

# Optimization

{{#include optimization.incl.md}}

## Minimize a Function with Brent's Method {#minimize-a-function-with-brents-method}

[![argmin][c~argmin~docs~badge]][c~argmin~docs]{{hi:argmin}} [![cat~science][cat~science~badge]][cat~science]

Brent's minimization method combines golden section search with parabolic interpolation. It searches within a bounded interval without requiring a derivative. This example finds the minimum of a quadratic function on `[-5, 8]`.

```rust,editable
{{#include ../../../crates/cats/mathematics/examples/optimization/brent_minimization.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[numerical_analysis | Numerical Analysis]].
- [[linear_algebra | Linear Algebra]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [accuracy review: mathematics](https://github.com/john-cd/rust_howto/issues/1894)
</div>
