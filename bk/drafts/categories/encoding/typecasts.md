<!-- TODO Reviewed ??-??-2026 -->

# Typecasts

{{#include typecasts.incl.md}}

Rust does not perform general implicit conversions between different types. Some limited [coercions][book~rust-reference~type-coercions]↗ happen from context, but numeric casts, parsing, and byte reinterpretation are explicit operations with different rules.

## Use `as` for Explicit Primitive Casts {#type-conversion-using-as .skip}

The [`as`][keyword~as]↗ keyword performs a primitive cast. It is concise, but may discard information: integer-to-integer casts truncate to the destination width, signedness changes reinterpret the resulting bits, and floating-point-to-integer casts saturate at the integer bounds (with `NaN` becoming zero). Floating-point casts can also lose precision. The [Rust Reference][book~rust-reference~casts]↗ lists the supported casts and their exact behavior.

Use `as` when its behavior is intended and clear at the call site. For checked numeric conversions, use `TryFrom`:

```rust,editable
{{#include ../../../crates/cats/encoding/examples/typecasts/primitive.rs:example}}
```

### Numeric casts {#Casting Between Numeric Types .skip}

Integer-to-integer casts truncate to the destination width, signedness changes reinterpret the resulting bits, and floating-point-to-integer casts saturate at the integer bounds (`NaN` becomes zero). Floating-point casts can lose precision. The [Rust Reference][book~rust-reference~casts]↗ lists the exact rules.

### Pointer casts {#Casting Between Pointers .skip4}

Raw-pointer casts do not dereference memory and do not themselves require an `unsafe` block. Dereferencing a raw pointer does require `unsafe`, and a valid pointer's provenance, alignment, and lifetime still matter. Do not use integer casts as a substitute for a sound pointer or FFI design. There is no standalone pointer-cast example because a safe example needs a concrete API that establishes the pointer's validity and lifetime.

## Prefer Conversion Traits {#Conversion Traits: `From`, `Into`, `TryFrom` and `TryInto` .skip}

Use [`From`][c~std::convert::From~docs]↗ and [`Into`][c~std::convert::Into~docs]↗ for conversions that cannot fail. Implement `From<T> for U`; callers also get `Into<U> for T` through the standard blanket implementation. Use [`TryFrom`][c~std::convert::TryFrom~docs]↗ and [`TryInto`][c~std::convert::TryInto~docs]↗ when the input may not fit or otherwise may be rejected. These traits return a `Result`, so callers can handle failure rather than silently truncating.

## Parse Text with `FromStr` {#Parsing Strings with the `FromStr` Trait .skip}

Parsing text is different from converting a value that already has a typed representation. Many standard types implement [`FromStr`][c~std::str::FromStr~docs]↗ and can be parsed with `.parse()`. Parsing returns a result; propagate or handle the error instead of unwrapping untrusted input.

```rust
fn parse_port(text: &str) -> Result<u16, std::num::ParseIntError> {
    text.parse()
}
```

## Distinguish Coercions from Casts {#Coercions (Implicit Conversions) .skip6}

Rust permits a small set of context-driven coercions, such as borrowing a value, dereferencing references, and unsizing an array reference to a slice reference. These do not use `as` and have separate rules. See the [Rust Reference on type coercions][book~rust-reference~type-coercions]↗. No standalone coercion example is shown because the compiler infers these conversions from the expected type; an explicit cast would teach the wrong mechanism.

A trait object such as `&dyn Display` is a way to use dynamic dispatch; creating one is not a primitive cast. See [trait objects][p~trait-objects].

### Dynamic dispatch {#Polymorphism with Traits (Dynamic Dispatch) .skip5}

Use trait objects such as `&dyn Display` when dynamic dispatch is appropriate. This is a trait-object coercion, not a primitive cast.

## Reinterpret Bytes with `bytemuck` or `zerocopy`

These crates provide checked, trait-constrained operations for viewing compatible in-memory values as bytes or constructing values from bytes. This is not general serialization: layout and validity rules apply, and byte order must be handled explicitly when defining a portable file or network format.

### `bytemuck` {#bytemuck}

[![bytemuck][c~bytemuck~docs~badge]][c~bytemuck~docs]{{hi:bytemuck}}
[![bytemuck~crates.io][c~bytemuck~crates.io~badge]][c~bytemuck~crates.io]
[![bytemuck~repo][c~bytemuck~repo~badge]][c~bytemuck~repo]
[![bytemuck~lib.rs][c~bytemuck~lib.rs~badge]][c~bytemuck~lib.rs]
[![cat~encoding][cat~encoding~badge]][cat~encoding]
[![cat~no-std][cat~no-std~badge]][cat~no-std]

The `bytemuck` example demonstrates casts guarded by its plain-data marker traits:

```rust,editable
{{#include ../../../crates/cats/encoding/examples/typecasts/bytemuck.rs:example}}
```

### `zerocopy` {#zerocopy}

[![zerocopy][c~zerocopy~docs~badge]][c~zerocopy~docs]{{hi:zerocopy}}
[![zerocopy~crates.io][c~zerocopy~crates.io~badge]][c~zerocopy~crates.io]
[![zerocopy~repo][c~zerocopy~repo~badge]][c~zerocopy~repo]
[![zerocopy~lib.rs][c~zerocopy~lib.rs~badge]][c~zerocopy~lib.rs]
[![cat~encoding][cat~encoding~badge]][cat~encoding]
[![cat~parsing][cat~parsing~badge]][cat~parsing]
[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]
[![cat~embedded][cat~embedded~badge]][cat~embedded]
[![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]

Use the crate's derived traits and conversion methods to make layout and byte-validity requirements explicit:

```rust,editable
{{#include ../../../crates/cats/encoding/examples/typecasts/zerocopy.rs:example}}
```

## Choose an Explicit Byte Order {#endianness}

An integer's in-memory byte order is platform-dependent. For a portable binary format, choose an order in the format and encode or decode with that order. The standard integer methods `to_le_bytes` and `from_le_bytes` handle fixed-width little-endian values directly:

```rust
let value = 0x1234_u16;
let bytes = value.to_le_bytes();
assert_eq!(u16::from_le_bytes(bytes), value);
```

For reading and writing structured streams, [`byteorder`][c~byteorder~docs]↗ offers explicit endian-aware operations. This example writes and reads a field as little-endian:

```rust,editable
{{#include ../../../crates/cats/encoding/examples/endian_byte.rs:example}}
```

Do not copy a struct's memory to define a portable format: padding, layout, validity, and byte order need explicit handling.

## Related Topics {#related-topics .skip}

- [Binary encoders](_binary_encoders.md).
- [Serialization and encoding][p~encoding].
- [Language data types][p~data-types].
- [Parsing text][p~parsing].
- [Trait objects][p~trait-objects].
- [Strings][p~strings].
  {{#include refs.incl.md}}
  {{#include ../../refs/link-refs.md}}
