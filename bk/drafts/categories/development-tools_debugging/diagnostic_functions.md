<!-- TODO Reviewed ??-??-2026 -->

# Diagnostic Functions

{{#include diagnostic_functions.incl.md}}

These helpers belong in the debugging chapter: they expose runtime type information and source locations while inspecting a program. `std::any::type_name` and `type_name_of_val` are diagnostic hints, not stable type identifiers; their returned spelling is unspecified and must not be used for serialization or program logic. Use `dbg!` for temporary value inspection, `Backtrace::capture` for a backtrace when supported, and `file!`, `line!`, and `module_path!` for compile-time source context.

## Inspect a Type Name and Source Location {#type-name-of-val}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/other/type_name_of_val.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Logging Configuration](config_log.md) explains filtering and output destinations.
- [Log Messages](log.md) covers the logging facade.
- [Tracing](tracing.md) adds structured events and spans to diagnostics.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
- The debugging category is the intended location because these functions assist runtime inspection; `type_name` is unsuitable as a stable identifier.

</div>
