# Standard Library

The Standard Library provides essential functionality for Rust programs and includes modules for memory management, concurrency, collections, and more. The Standard Library is automatically included in every Rust project.

We listed below the (stable) modules of the Standard Library, highlighted the most important modules, and listed key types. Consult the complete documentation for the [Standard Library][c~std~docs]↗ as well.

| Modules | Description | See Also |
| --- | --- | --- |
| [`std::alloc`][c~std::alloc~docs]↗{{hi:std::alloc}} | Memory allocation APIs. For advanced users. | [[memory-management | Memory Management]] |
| [`std::any`][c~std::any~docs]↗{{hi:std::any}} | Utilities for dynamic typing or type reflection. Home of [`Any`][c~std::any::Any~docs]↗. Use for plugins, extensibility, scripting. Advanced. | [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]] [[scripting | Scripting]] |
| [`std::arch`][c~std::arch~docs]↗{{hi:std::arch}} | SIMD and vendor intrinsics module. Advanced topic. | [`std::simd`][c~std::simd~docs]↗ module |
| [`std::array`][c~std::array~docs]↗{{hi:std::array}} | Utilities for the array primitive type `[T; N]`.{{hi:std::array}} | [[data-structures | Data Structures]], [[vectors | Vectors]], [[slices | Slices]] |
| [`std::ascii`][c~std::ascii~docs]↗{{hi:std::ascii}} | Operations on ASCII strings and characters. You may use [`escape_default`][c~std::ascii::escape_default~docs]↗{{hi:std::ascii::escape_default}} | [[encoding | Encoding]] |
| [`std::backtrace`][c~std::backtrace~docs]↗{{hi:std::backtrace}} | Support for capturing a stack backtrace of an OS thread. | [[error_handling | Error Handling]] |
| [`std::borrow`][c~std::borrow~docs]↗{{hi:std::borrow}} | A module for working with borrowed data. Home of [`Borrow`][c~std::borrow::Borrow~docs]↗ and [`BorrowMut`][c~std::borrow::BorrowMut~docs]↗. | [[borrow | Borrow]], [[ownership_borrowing | Ownership and Borrowing]] |
| [**`std::boxed`**][c~std::boxed~docs]↗{{hi:std::boxed}} | The [`Box<T>`][c~std::boxed::Box~docs]↗ type for heap allocation. | [[box | Box]] |
| [**`std::cell`**][c~std::cell~docs]↗{{hi:std::cell}} | Shareable mutable containers. Home of [`RefCell<T>`][c~std::cell::RefCell~docs]↗. | [[interior_mutability | Interior Mutability]] |
| [`std::char`][c~std::char~docs]↗{{hi:std::char}} | Utilities for the [char][primitive~char]↗ primitive type. |  |
| [`std::clone`][c~std::clone~docs]↗{{hi:std::clone}} | The [`Clone`][c~std::clone::Clone~docs]↗ trait for types that cannot be implicitly copied. | [`ToOwned`][c~std::borrow::ToOwned~docs]↗, [[cow | Cow]] |
| [**`std::cmp`**][c~std::cmp~docs]↗{{hi:std::cmp}} | Utilities for comparing and ordering values. Home of [`PartialEq`][c~std::cmp::PartialEq~docs]↗, [`Eq`][c~std::cmp::Eq~docs]↗, [`PartialOrd`][c~std::cmp::PartialOrd~docs]↗, [`Ord`][c~std::cmp::Ord~docs]↗. | [[cmp | Equality and Ordering]], [[algorithms | Algorithms]], [[sorting | Sorting]] |
| [**`std::collections`**][c~std::collections~docs]↗{{hi:std::collections}} | Collection types. Home of [`HashMap`][c~std::collections::HashMap~docs]↗, [`HashSet`][c~std::collections::HashSet~docs]↗ and others. | [[hashmap | HashMap]], [[data-structures | Data Structures]] |
| [**`std::convert`**][c~std::convert~docs]↗{{hi:std::convert}} | Traits for conversions between types: [`AsRef`][c~std::convert::AsRef~docs]↗, [`From`][c~std::convert::From~docs]↗ and others. | [[asref | AsRef]] |
| [**`std::default`**][c~std::default~docs]↗{{hi:std::default}} | The [`Default`][c~std::default::Default~docs]↗ trait for types with a default value. | [[conversion_traits | Conversion Traits]], |[[derive | Derive]] |
| [`std::env`][c~std::env~docs]↗{{hi:std::env}} | Inspection and manipulation of the process's environment. | [[environment_variables | Environment Variables]] |
| [**`std::error`**][c~std::error~docs]↗{{hi:std::error}} | [`Error`][c~std::error::Error~docs]↗ Interfaces. | [[error_handling | Error Handling]] |
| [`std::f32`][c~std::f32~docs]↗{{hi:std::f32}} | Constants for the `f32` single-precision floating point type. | [[data_types | Data Types]] |
| [`std::f64`][c~std::f64~docs]↗{{hi:std::f64}} | Constants for the `f64` double-precision floating point type. | [[data_types | Data Types]] |
| [`std::ffi`][c~std::ffi~docs]↗{{hi:std::ffi}} | Utilities related to FFI bindings. Home of [`OsString`][c~std::ffi::OsString~docs]↗ and [`CString`][c~std::ffi::CString~docs]↗. | [[other_strings | Other Strings]] |
| [**`std::fmt`**][c~std::fmt~docs]↗{{hi:std::fmt}} | Utilities for formatting and printing Strings. Home of the [`Write`][c~std::fmt::Write~docs]↗ trait. | [[strings | Strings]], [`format!`][c~std::format~docs]↗ macro |
| [**`std::fs`**][c~std::fs~docs]↗{{hi:std::fs}} | Filesystem manipulation operations. Home of [`File`][c~std::fs::File~docs]↗ and many useful functions. | [[filesystem | Filesystem]], [[directories | Directories]] |
| [`std::future`][c~std::future~docs]↗{{hi:std::future}} | Asynchronous basic functionality. Home of the [`Future`][c~std::future::Future~docs]↗ trait. | [[async| Async]] |
| [`std::hash`][c~std::hash~docs]↗{{hi:std::hash}} | Generic hashing support. Home of the [`Hash`][c~std::hash::Hash~docs]↗ trait. | [[cryptography | Cryptography]], [[data_structures | Data Structures]] |
| [`std::hint`][c~std::hint~docs]↗{{hi:std::hint}} | Hints to compiler that affects how code should be emitted or optimized. Advanced. | [[benchmarking | Benchmarking]] |
| [**`std::io`**][c~std::io~docs]↗{{hi:std::io}} | Traits, helpers, and type definitions for core I/O functionality. Home of the [`Read`][c~std::io::Read~docs]↗ and [`Write`][c~std::io::Write~docs]↗ traits and useful functions. | [[read-write | Read & Write from Files]] |
| [**`std::iter`**][c~std::iter~docs]↗{{hi:std::iter}} | Composable external iteration. Home of the [`Iterator`][c~std::iter::Iterator~docs]↗ and [`IntoIterator`][c~std::iter::IntoIterator~docs]↗ traits. | [[iterators | Iterators]] |
| [`std::marker`][c~std::marker~docs]↗{{hi:std::marker}} | Primitive traits and types representing basic properties of types. Home of [`Send`][c~std::marker::Send~docs]↗, [`Sync`][c~std::marker::Sync~docs]↗, [`Sized`][c~std::marker::Sized~docs]↗, [`Copy`][c~std::marker::Copy~docs]↗, and [`PhantomData`][c~std::marker::PhantomData~docs]↗. | [[send_sync | Send & Sync]], [[ownership_borrowing | Ownership & Borrowing]], [[derive | Derive]] |
| [`std::mem`][c~std::mem~docs]↗{{hi:std::mem}} | Basic functions for dealing with memory, such as [`drop`][c~std::mem::drop~docs]↗{{hi:std::mem::drop}}. | [[memory-management | Memory Management]] |
| [`std::net`][c~std::net~docs]↗{{hi:std::net}} | Networking primitives for TCP/UDP communication. Home of [`IpAddr`][c~std::net::IpAddr~docs]↗ and [`TcpListener`][c~std::net::TcpListener~docs]↗. | [[network-programming | Network Programming]] |
| [`std::num`][c~std::num~docs]↗{{hi:std::num}} | Additional functionality for numerics. Home of [`NonZero`][c~std::num::NonZero~docs]↗, [`Saturating`][c~std::num::Saturating~docs]↗, and [`Wrapping`][c~std::num::Wrapping~docs]↗. | [[data_types | Data Types]], [[mathematics | Mathematics]] |
| [`std::ops`][c~std::ops~docs]↗{{hi:std::ops}} | Overloadable operators, e.g., [`Add`][c~std::ops::Add~docs]↗, [`Mul`][c~std::ops::Mul~docs]↗ and [`Index`][c~std::ops::Index~docs]↗. Also home of [`Fn`][c~std::ops::Fn~docs]↗ and related traits; [`Deref`][c~std::ops::Deref~docs]↗ and [`Drop`][c~std::ops::Drop~docs]↗. | [[ops | Ops]], [[drop | Drop]], [[closures | Closures]], [[memory-management | Memory Management]] |
| [**`std::option`**][c~std::option~docs]↗{{hi:std::option}} | Optional values via [`Option`][c~std::option::Option~docs]↗. | [[option | Option]] |
| [`std::os`][c~std::os~docs]↗{{hi:std::os}} | OS-specific functionality.{{hi:OS}} | [[os | OS]] |
| [`std::panic`][c~std::panic~docs]↗{{hi:std::panic}} | Panic support in the standard library. | [[error_handling | Error Handling]] |
| [**`std::path`**][c~std::path~docs]↗{{hi:std::path}} | Cross-platform path manipulation with [`PathBuf`][c~std::path::PathBuf~docs]↗ and [`Path`][c~std::path::Path~docs]↗ | [[filesystem | Filesystem]] |
| [`std::pin`][c~std::pin~docs]↗{{hi:std::pin}} | Types that pin data to a location in memory, via [`Pin`][c~std::pin::Pin~docs]↗. | [[pin | Pin]], [[rust_specific_patterns | Rust-specific Patterns]] |
| [`std::prelude`][c~std::prelude~docs]↗{{hi:std::prelude}} | The prelude that Rust automatically imports into every Rust program. | [[code_organization | Code Organization]] |
| [`std::primitive`][c~std::primitive~docs]↗{{hi:std::primitive}} | This module reexports the primitive types to allow usage that is not possibly shadowed by other declared types. | [[data_types | Data Types]] |
| [**`std::process`**][c~std::process~docs]↗{{hi:std::process}} | A module for working with processes. Home of [`Command`][c~std::process::Command~docs]↗ and [`exit()`][c~std::process::exit~docs]↗. | [[external_commands | External Commands]] |
| [`std::ptr`][c~std::ptr~docs]↗{{hi:std::ptr}} | Manually manage memory through raw pointers, e.g. [`NotNull`][c~std::ptr::NonNull~docs]↗ and many unsafe functions. Advanced topic. | [[ownership_borrowing | Ownership & Borrowing]] |
| [**`std::rc`**][c~std::rc~docs]↗{{hi:std::rc}} | Single-threaded reference-counting pointers. 'Rc' stands for "Reference Counted". See [`Rc`][c~std::rc::Rc~docs]↗. | [[reference_counting | Reference Counting]] |
| [**`std::result`**][c~std::result~docs]↗{{hi:std::result}} | Error handling with the [`Result`][c~std::result::Result~docs]↗ type. | [[result | Result]] |
| [`std::slice`][c~std::slice~docs]↗{{hi:std::slice}} | Utilities for the [slice][primitive~slice]↗ primitive type. | [[slices | Slices]] |
| [`std::str`][c~std::str~docs]↗{{hi:std::str}} | Utilities for the [`str`][primitive~str]↗ primitive type. | [[slices | Slices]], [[string | String]] |
| [**`std::string`**][c~std::string~docs]↗{{hi:std::string}} | A UTF-8–encoded, growable [`String`][c~std::string::String~docs]↗. | [[string | String]] |
| [**`std::sync`**][c~std::sync~docs]↗{{hi:std::sync}} | Useful synchronization primitives: [`Arc`][c~std::sync::Arc~docs]↗, [`Mutex`][c~std::sync::Mutex~docs]↗, [`LazyLock`][c~std::sync::LazyLock~docs]↗. | [[concurrency | Concurrency]], [[smart_pointers | Smart Pointers]], [[sync | Sync]] |
| [`std::task`][c~std::task~docs]↗{{hi:std::task}} | Types and Traits for working with asynchronous tasks. For advanced users. | [[asynchronous | Asynchronous]] |
| [**`std::thread`**][c~std::thread~docs]↗{{hi:std::thread}} | Native threads with [`Thread`][c~std::thread::Thread~docs]↗. | [[explicit_thread | Explicit Thread]] |
| [`std::time`][c~std::time~docs]↗{{hi:std::time}} | Temporal quantification with [`Duration`][c~std::time::Duration~docs]↗ and [`Instant`][c~std::time::Instant~docs]↗. | [[date-and-time | Date & Time]] |
| [**`std::vec`**][c~std::vec~docs]↗{{hi:std::vec}} | A contiguous growable array type with heap-allocated contents, written [`Vec<T>`][c~std::vec::Vec~docs]↗. | [[vectors | Vectors]] |

The following covers portions of the Rust Standard Library that are not otherwise covered by another chapter: core types like [`Option`][c~std::option::Option~docs]↗{{hi:std::option::Option}}, [`Result`][c~std::result::Result~docs]↗{{hi:std::result::Result}}; smart pointers; and traits for conversions.

## `Option`

{{#include option.incl.md}}

## `Result`

{{#include result.incl.md}}

## `Default`

{{#include default.incl.md}}

## Equality and Ordering

{{#include cmp.incl.md}}

## Smart Pointers

{{#include smart_pointers.incl.md}}

### `Box`

{{#include box.incl.md}}

### Reference Counting: `Rc`, `Arc`

{{#include reference_counting.incl.md}}

### Interior Mutability: `RefCell`, `Cell`

{{#include interior_mutability.incl.md}}

### Clone-On-Write: `Cow`

{{#include cow.incl.md}}

### `Pin`

{{#include pin.incl.md}}

## `Drop`

{{#include drop.incl.md}}

## Conversion Traits: `From`, `Into`, `TryFrom`, `TryInto`

{{#include conversion_traits.incl.md}}

## `AsRef`

{{#include asref.incl.md}}

## `Borrow`

{{#include borrow.incl.md}}

## Automatic Trait Derivation

{{#include derive.incl.md}}

## Overloading Operators

{{#include ops.incl.md}}

## Dynamic Typing

{{#include dynamic_typing.incl.md}}

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

- [tour-of-rusts-standard-library-traits][tour-of-rusts-standard-library-traits~github]↗.
- [black_box in `core::hint`][c~core::hint::black_box~docs]↗.
- [`std::pin`][c~std::pin~projections-and-structural-pinning~docs]↗.
- [`core::iter`][c~core::iter~docs]↗.

</div>
