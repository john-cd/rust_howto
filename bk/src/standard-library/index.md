<!-- TODO Reviewed ??-??-2026. -->

# Standard Library

The Standard Library provides essential functionality for Rust programs and includes modules for memory management, concurrency, collections, and more. It is available to the standard Rust prelude and linked by default; crates that use `#![no_std]` opt out of `std` and use `core` instead (and may use `alloc` when available).

The table below lists stable modules of the Standard Library, highlights the most important modules, and lists key types. Consult the complete documentation for the [Standard Library][c~std~docs]↗ as well.

| Modules | Description | See Also |
| --- | --- | --- |
| [`std::alloc`][c~std::alloc~docs]↗{{hi:std::alloc}} | Memory allocation APIs. For advanced users. | [Memory Management](../categories/memory-management/index.md) |
| [`std::any`][c~std::any~docs]↗{{hi:std::any}} | Utilities for dynamic typing or type reflection. Home of [`Any`][c~std::any::Any~docs]↗. Use for plugins, extensibility, scripting. Advanced. | [Development Tools - Cargo Plugins](../categories/development-tools_cargo-plugins/index.md) [Scripting](https://github.com/john-cd/rust_howto/blob/develop/later/src/other/scripting/index.md) |
| [`std::arch`][c~std::arch~docs]↗{{hi:std::arch}} | SIMD and vendor intrinsics module. Advanced topic. | [`std::simd`][c~std::simd~docs]↗ is experimental and requires nightly Rust. |
| [`std::array`][c~std::array~docs]↗{{hi:std::array}} | Utilities for the array primitive type `[T; N]`.{{hi:std::array}} | [[data-structures | Data Structures]], [[vectors | Vectors]], [[slices | Slices]] |
| [`std::ascii`][c~std::ascii~docs]↗{{hi:std::ascii}} | Operations on ASCII strings and characters. You may use [`escape_default`][c~std::ascii::escape_default~docs]↗{{hi:std::ascii::escape_default}} | [Encoding](../categories/encoding/index.md) |
| [`std::backtrace`][c~std::backtrace~docs]↗{{hi:std::backtrace}} | Support for capturing a stack backtrace of an OS thread. | [[recoverable_error_handling | Error Handling]] |
| [`std::borrow`][c~std::borrow~docs]↗{{hi:std::borrow}} | A module for working with borrowed data. Home of [`Borrow`][c~std::borrow::Borrow~docs]↗ and [`BorrowMut`][c~std::borrow::BorrowMut~docs]↗. | [[reference-to-reference_conversions | Borrow]], [[ownership_and_borrowing | Ownership and Borrowing]] |
| [**`std::boxed`**][c~std::boxed~docs]↗{{hi:std::boxed}} | The [`Box<T>`][c~std::boxed::Box~docs]↗ type for heap allocation. | [[heap_storage | Box]] |
| [**`std::cell`**][c~std::cell~docs]↗{{hi:std::cell}} | Shareable mutable containers. Home of [`RefCell<T>`][c~std::cell::RefCell~docs]↗. | [[interior_mutability | Interior Mutability]] |
| [`std::char`][c~std::char~docs]↗{{hi:std::char}} | Utilities for the [char][primitive~char]↗ primitive type. | |
| [`std::clone`][c~std::clone~docs]↗{{hi:std::clone}} | The [`Clone`][c~std::clone::Clone~docs]↗ trait for types that cannot be implicitly copied. | [`ToOwned`][c~std::borrow::ToOwned~docs]↗, [[clone-on-write | Cow]] |
| [**`std::cmp`**][c~std::cmp~docs]↗{{hi:std::cmp}} | Utilities for comparing and ordering values. Home of [`PartialEq`][c~std::cmp::PartialEq~docs]↗, [`Eq`][c~std::cmp::Eq~docs]↗, [`PartialOrd`][c~std::cmp::PartialOrd~docs]↗, [`Ord`][c~std::cmp::Ord~docs]↗. | [[value_comparison | Equality and Ordering]], [[algorithms | Algorithms]], [[sorting | Sorting]] |
| [**`std::collections`**][c~std::collections~docs]↗{{hi:std::collections}} | Collection types. Home of [`HashMap`][c~std::collections::HashMap~docs]↗, [`HashSet`][c~std::collections::HashSet~docs]↗ and others. | [[hashmaps | HashMap]], [[data-structures | Data Structures]] |
| [**`std::convert`**][c~std::convert~docs]↗{{hi:std::convert}} | Traits for conversions between types: [`AsRef`][c~std::convert::AsRef~docs]↗, [`From`][c~std::convert::From~docs]↗ and others. | [[reference-to-reference_conversions | AsRef]] |
| [**`std::default`**][c~std::default~docs]↗{{hi:std::default}} | The [`Default`][c~std::default::Default~docs]↗ trait for types with a default value. | [[conversion_traits | Conversion Traits]], [[automatic_trait_derivation | Derive]] |
| [`std::env`][c~std::env~docs]↗{{hi:std::env}} | Inspection and manipulation of the process's environment. | [Environment Variables](../categories/config/environment_variables.md) |
| [**`std::error`**][c~std::error~docs]↗{{hi:std::error}} | [`Error`][c~std::error::Error~docs]↗ Interfaces. | [[recoverable_error_handling | Error Handling]] |
| [`std::f32`][c~std::f32~docs]↗{{hi:std::f32}} | Constants for the `f32` single-precision floating point type. | [[data_types | Data Types]] |
| [`std::f64`][c~std::f64~docs]↗{{hi:std::f64}} | Constants for the `f64` double-precision floating point type. | [[data_types | Data Types]] |
| [`std::ffi`][c~std::ffi~docs]↗{{hi:std::ffi}} | Utilities related to FFI bindings. Home of [`OsString`][c~std::ffi::OsString~docs]↗ and [`CString`][c~std::ffi::CString~docs]↗. | [Other Strings](../categories/text-processing/other_strings.md) |
| [**`std::fmt`**][c~std::fmt~docs]↗{{hi:std::fmt}} | Utilities for formatting and printing Strings. Home of the [`Write`][c~std::fmt::Write~docs]↗ trait. | [[strings | Strings]], [`format!`][c~std::format~docs]↗ macro |
| [**`std::fs`**][c~std::fs~docs]↗{{hi:std::fs}} | Filesystem manipulation operations. Home of [`File`][c~std::fs::File~docs]↗ and many useful functions. | [[filesystem | Filesystem]], [[directories | Directories]] |
| [`std::future`][c~std::future~docs]↗{{hi:std::future}} | Asynchronous basic functionality. Home of the [`Future`][c~std::future::Future~docs]↗ trait. | [Async](../categories/asynchronous/async.md) |
| [`std::hash`][c~std::hash~docs]↗{{hi:std::hash}} | Generic hashing support. Home of the [`Hash`][c~std::hash::Hash~docs]↗ trait. | [Cryptography](../categories/cryptography/index.md), [[data_structures | Data Structures]] |
| [`std::hint`][c~std::hint~docs]↗{{hi:std::hint}} | Hints to compiler that affect how code should be emitted or optimized. Advanced. | [Benchmarking](../categories/development-tools_profiling/benchmarking.md) |
| [**`std::io`**][c~std::io~docs]↗{{hi:std::io}} | Traits, helpers, and type definitions for core I/O functionality. Home of the [`Read`][c~std::io::Read~docs]↗ and [`Write`][c~std::io::Write~docs]↗ traits and useful functions. | [[reading_and_writing_files | Read & Write from Files]] |
| [**`std::iter`**][c~std::iter~docs]↗{{hi:std::iter}} | Composable external iteration. Home of the [`Iterator`][c~std::iter::Iterator~docs]↗ and [`IntoIterator`][c~std::iter::IntoIterator~docs]↗ traits. | [[iterators | Iterators]] |
| [`std::marker`][c~std::marker~docs]↗{{hi:std::marker}} | Primitive traits and types representing basic properties of types. Home of [`Send`][c~std::marker::Send~docs]↗, [`Sync`][c~std::marker::Sync~docs]↗, [`Sized`][c~std::marker::Sized~docs]↗, [`Copy`][c~std::marker::Copy~docs]↗, and [`PhantomData`][c~std::marker::PhantomData~docs]↗. | [Send & Sync](../categories/concurrency/send_sync.md), [[ownership_and_borrowing | Ownership & Borrowing]], [[automatic_trait_derivation | Derive]] |
| [`std::mem`][c~std::mem~docs]↗{{hi:std::mem}} | Basic functions for dealing with memory, such as [`drop`][c~std::mem::drop~docs]↗{{hi:std::mem::drop}}. | [Memory Management](../categories/memory-management/index.md) |
| [`std::net`][c~std::net~docs]↗{{hi:std::net}} | Networking primitives for TCP/UDP communication. Home of [`IpAddr`][c~std::net::IpAddr~docs]↗ and [`TcpListener`][c~std::net::TcpListener~docs]↗. | [Network Programming](../categories/network-programming/index.md) |
| [`std::num`][c~std::num~docs]↗{{hi:std::num}} | Additional functionality for numerics. Home of [`NonZero`][c~std::num::NonZero~docs]↗, [`Saturating`][c~std::num::Saturating~docs]↗, and [`Wrapping`][c~std::num::Wrapping~docs]↗. | [[data_types | Data Types]], [[mathematics | Mathematics]] |
| [`std::ops`][c~std::ops~docs]↗{{hi:std::ops}} | Overloadable operators, e.g., [`Add`][c~std::ops::Add~docs]↗, [`Mul`][c~std::ops::Mul~docs]↗ and [`Index`][c~std::ops::Index~docs]↗. Also home of [`Fn`][c~std::ops::Fn~docs]↗ and related traits; [`Deref`][c~std::ops::Deref~docs]↗ and [`Drop`][c~std::ops::Drop~docs]↗. | [[operators | Ops]], [[resource_cleanup | Drop]], [[closures | Closures]], [Memory Management](../categories/memory-management/index.md) |
| [**`std::option`**][c~std::option~docs]↗{{hi:std::option}} | Optional values via [`Option`][c~std::option::Option~docs]↗. | [[optional_values | Option]] |
| [`std::os`][c~std::os~docs]↗{{hi:std::os}} | OS-specific functionality.{{hi:OS}} | [Operating Systems](../categories/os/index.md) |
| [`std::panic`][c~std::panic~docs]↗{{hi:std::panic}} | Panic support in the standard library. | [[recoverable_error_handling | Error Handling]] |
| [**`std::path`**][c~std::path~docs]↗{{hi:std::path}} | Cross-platform path manipulation with [`PathBuf`][c~std::path::PathBuf~docs]↗ and [`Path`][c~std::path::Path~docs]↗ | [[filesystem | Filesystem]] |
| [`std::pin`][c~std::pin~docs]↗{{hi:std::pin}} | Types that pin data to a location in memory, via [`Pin`][c~std::pin::Pin~docs]↗. | [[memory_address_pinning | Pin]], [Rust-specific Patterns](../categories/rust-patterns/rust_specific_patterns.md) |
| [`std::prelude`][c~std::prelude~docs]↗{{hi:std::prelude}} | The prelude that Rust automatically imports into every Rust program. | [[code_organization | Code Organization]] |
| [`std::primitive`][c~std::primitive~docs]↗{{hi:std::primitive}} | This module reexports the primitive types to allow usage that is not possibly shadowed by other declared types. | [[data_types | Data Types]] |
| [**`std::process`**][c~std::process~docs]↗{{hi:std::process}} | A module for working with processes. Home of [`Command`][c~std::process::Command~docs]↗ and [`exit()`][c~std::process::exit~docs]↗. | [External Commands](../categories/os/external_commands.md) |
| [`std::ptr`][c~std::ptr~docs]↗{{hi:std::ptr}} | Manually manage memory through raw pointers, e.g. [`NonNull`][c~std::ptr::NonNull~docs]↗ and many unsafe functions. Advanced topic. | [Pointer Provenance at an FFI Boundary](pointer_provenance.md) |
| [**`std::rc`**][c~std::rc~docs]↗{{hi:std::rc}} | Single-threaded reference-counting pointers. 'Rc' stands for "Reference Counted". See [`Rc`][c~std::rc::Rc~docs]↗. | [[reference_counting | Reference Counting]] |
| [**`std::result`**][c~std::result~docs]↗{{hi:std::result}} | Error handling with the [`Result`][c~std::result::Result~docs]↗ type. | [[recoverable_error_handling | Result]] |
| [`std::slice`][c~std::slice~docs]↗{{hi:std::slice}} | Utilities for the [slice][primitive~slice]↗ primitive type. | [[slices | Slices]] |
| [`std::str`][c~std::str~docs]↗{{hi:std::str}} | Utilities for the [`str`][primitive~str]↗ primitive type. | [[slices | Slices]], [[strings | String]] |
| [**`std::string`**][c~std::string~docs]↗{{hi:std::string}} | A UTF-8–encoded, growable [`String`][c~std::string::String~docs]↗. | [[strings | String]] |
| [**`std::sync`**][c~std::sync~docs]↗{{hi:std::sync}} | Useful synchronization primitives: [`Arc`][c~std::sync::Arc~docs]↗, [`Mutex`][c~std::sync::Mutex~docs]↗, [`LazyLock`][c~std::sync::LazyLock~docs]↗. | [Concurrency](../categories/concurrency/index.md), [[smart_pointers | Smart Pointers]], [Sync](https://doc.rust-lang.org/std/sync/) |
| [`std::task`][c~std::task~docs]↗{{hi:std::task}} | Types and Traits for working with asynchronous tasks. For advanced users. | [Asynchronous](../categories/asynchronous/index.md) |
| [**`std::thread`**][c~std::thread~docs]↗{{hi:std::thread}} | Native threads with [`Thread`][c~std::thread::Thread~docs]↗. | [Explicit Thread](../categories/concurrency/explicit_threads.md) |
| [`std::time`][c~std::time~docs]↗{{hi:std::time}} | Temporal quantification with [`Duration`][c~std::time::Duration~docs]↗ and [`Instant`][c~std::time::Instant~docs]↗. | [Date & Time](../categories/date-and-time/index.md) |
| [**`std::vec`**][c~std::vec~docs]↗{{hi:std::vec}} | A contiguous growable array type with heap-allocated contents, written [`Vec<T>`][c~std::vec::Vec~docs]↗. | [[vectors | Vectors]] |

The following covers portions of the Rust Standard Library that are not otherwise covered by another chapter: core types like [`Option`][c~std::option::Option~docs]↗{{hi:std::option::Option}}, [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}}; smart pointers; and traits for conversions.

## Optional Values: `Option`

{{#include optional_values.incl.md}}

## Recoverable Error Handling: `Result`

{{#include recoverable_error_handling.incl.md}}

## Default Values: `Default`

{{#include default_values.incl.md}}

## Value Comparison: Equality and Ordering

{{#include value_comparison.incl.md}}

## Smart Pointers

{{#include smart_pointers.incl.md}}

### Heap Storage with `Box`

{{#include heap_storage.incl.md}}

### Reference Counting: `Rc`, `Arc`

{{#include reference_counting.incl.md}}

### Interior Mutability: `RefCell`, `Cell`, `OnceCell`

{{#include interior_mutability.incl.md}}

### Clone-on-Write: `Cow`

{{#include clone-on-write.incl.md}}

### Memory Address Pinning: `Pin`

{{#include memory_address_pinning.incl.md}}

## Resource Cleanup: `Drop`

{{#include resource_cleanup.incl.md}}

## Conversion Traits: `From`, `Into`, `TryFrom`, `TryInto`

{{#include conversion_traits.incl.md}}

## Reference-to-reference Conversion: `AsRef`

{{#include reference-to-reference_conversions.incl.md}}

## Borrowed Types: `Borrow`

{{#include borrowed_types.incl.md}}

## Automatic Trait Derivation: `derive`

{{#include automatic_trait_derivation.incl.md}}

## Operators

{{#include operators.incl.md}}

## Dynamic Typing: `Any`

{{#include dynamic_typing.incl.md}}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [Tour of Rust's Standard Library Traits][tour-of-rusts-standard-library-traits~repo]↗.
- [`black_box` in `core::hint`][c~core::hint::black_box~docs]↗.
- [`std::pin`][c~std::pin~projections-and-structural-pinning~docs]↗.
- [`core::iter`][c~core::iter~docs]↗.

</div>
