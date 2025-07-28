# Standard Library

The Standard Library provides essential functionality for Rust programs and includes modules for memory management, concurrency, collections, and more. The Standard Library is automatically included in every Rust project.

We listed below the (stable) modules of the Standard Library, highlighted the most important modules, and listed key types. Consult the complete documentation for the [Standard Library](https://doc.rust-lang.org/std/) as well.

| Modules | Description | See Also |
| --- | --- | --- |
| [alloc](https://doc.rust-lang.org/std/alloc/index.html)⮳{{hi:alloc}} | Memory allocation APIs. For advanced users. | [[memory-management | Memory Management]] |
| [any](https://doc.rust-lang.org/std/any/index.html)⮳{{hi:any}} | Utilities for dynamic typing or type reflection. Home of [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html)⮳. Use for plugins, extensibility, scripting. Advanced. | [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]] [[scripting | Scripting]] |
| [arch](https://doc.rust-lang.org/std/arch/index.html)⮳{{hi:arch}} | SIMD and vendor intrinsics module. Advanced. | [simd](https://doc.rust-lang.org/std/simd/index.html)⮳ module |
| [array](https://doc.rust-lang.org/std/array/index.html)⮳{{hi:array}} | Utilities for the array primitive type [[T; N]](https://doc.rust-lang.org/std/primitive.array.html)⮳. | [[data-structures | Data Structures]], [[vectors | Vectors]], [[slices | Slices]] |
| [ascii](https://doc.rust-lang.org/std/ascii/index.html)⮳{{hi:ascii}} | Operations on ASCII strings and characters. You may use [`escape_default`](https://doc.rust-lang.org/std/ascii/fn.escape_default.html)⮳{{hi:escape_default}} | [[encoding | Encoding]] |
| [backtrace](https://doc.rust-lang.org/std/backtrace/index.html)⮳{{hi:backtrace}} | Support for capturing a stack backtrace of an OS thread. | [[error_handling | Error Handling]] |
| [borrow](https://doc.rust-lang.org/std/borrow/index.html)⮳{{hi:borrow}} | A module for working with borrowed data. Home of [`Borrow`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)⮳ and [`BorrowMut`](https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html)⮳. | [[borrow | Borrow]], [[ownership_borrowing | Ownership and Borrowing]] |
| [**boxed**](https://doc.rust-lang.org/std/boxed/index.html)⮳{{hi:boxed}} | The [`Box<T>`](https://doc.rust-lang.org/std/boxed/struct.Box.html)⮳ type for heap allocation. | [[box | Box]] |
| [**cell**](https://doc.rust-lang.org/std/cell/index.html)⮳{{hi:cell}} | Shareable mutable containers. Home of [`RefCell<T>`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)⮳. | [[interior_mutability | Interior Mutability]] |
| [char](https://doc.rust-lang.org/std/char/index.html)⮳{{hi:char}} | Utilities for the [char](https://doc.rust-lang.org/std/primitive.char.html)⮳ primitive type. |  |
| [clone](https://doc.rust-lang.org/std/clone/index.html)⮳{{hi:clone}} | The [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)⮳ trait for types that cannot be implicitly copied. | [`ToOwned`](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html)⮳, [[cow | Cow]] |
| [**cmp**](https://doc.rust-lang.org/std/cmp/index.html)⮳{{hi:cmp}} | Utilities for comparing and ordering values. Home of [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)⮳, [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html)⮳, [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)⮳, [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)⮳. | [[cmp | Equality and Ordering]], [[algorithms | Algorithms]], [[sorting | Sorting]] |
| [**collections**](https://doc.rust-lang.org/std/collections/index.html)⮳{{hi:collections}} | Collection types. Home of [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)⮳, [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)⮳ and others. | [[hashmap | HashMap]], [[data-structures | Data Structures]] |
| [**convert**](https://doc.rust-lang.org/std/convert/index.html)⮳{{hi:convert}} | Traits for conversions between types: [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html), [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)⮳ and others. | [[asref | AsRef]] |
| [**default**](https://doc.rust-lang.org/std/default/index.html)⮳{{hi:default}} | The [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)⮳ trait for types with a default value. | [[conversion_traits | Conversion Traits]], |[[derive | Derive]] |
| [env](https://doc.rust-lang.org/std/env/index.html)⮳{{hi:env}} | Inspection and manipulation of the process's environment. | [[environment_variables | Environment Variables]] |
| [**error**](https://doc.rust-lang.org/std/error/index.html)⮳{{hi:error}} | [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html)⮳ Interfaces. | [[error_handling | Error Handling]] |
| [f32](https://doc.rust-lang.org/std/f32/index.html)⮳{{hi:f32}} | Constants for the `f32` single-precision floating point type. | [[data_types | Data Types]] |
| [f64](https://doc.rust-lang.org/std/f64/index.html)⮳{{hi:f64}} | Constants for the `f64` double-precision floating point type. | [[data_types | Data Types]] |
| [ffi](https://doc.rust-lang.org/std/ffi/index.html)⮳{{hi:ffi}} | Utilities related to FFI bindings. Home of [`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html)⮳ and [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html)⮳. | [[other_strings | Other Strings]] |
| [**fmt**](https://doc.rust-lang.org/std/fmt/index.html)⮳{{hi:fmt}} | Utilities for formatting and printing Strings. Home of the [`Write`](https://doc.rust-lang.org/std/fmt/trait.Write.html)⮳ trait. | [[strings | Strings]], [`format!`](https://doc.rust-lang.org/std/macro.format.html)⮳ macro |
| [**fs**](https://doc.rust-lang.org/std/fs/index.html)⮳{{hi:fs}} | Filesystem manipulation operations. Home of [`File`](https://doc.rust-lang.org/std/fs/struct.File.html)⮳ and many useful functions. | [[filesystem | Filesystem]], [[directories | Directories]] |
| [future](https://doc.rust-lang.org/std/future/index.html)⮳{{hi:future}} | Asynchronous basic functionality. Home of the [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html)⮳ trait. | [[async| Async]] |
| [hash](https://doc.rust-lang.org/std/hash/index.html)⮳{{hi:hash}} | Generic hashing support. Home of the [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html)⮳ trait. | [[cryptography | Cryptography]], [[data_structures | Data Structures]] |
| [hint](https://doc.rust-lang.org/std/hint/index.html)⮳{{hi:hint}} | Hints to compiler that affects how code should be emitted or optimized. Advanced. | [[benchmarking | Benchmarking]] |
| [**io**](https://doc.rust-lang.org/std/io/index.html)⮳{{hi:io}} | Traits, helpers, and type definitions for core I/O functionality. Home of the [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html)⮳ and [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html)⮳ traits and useful functions. | [[read-write | Read & Write from Files]] |
| [iter](https://doc.rust-lang.org/std/iter/index.html)⮳{{hi:iter}} | Composable external iteration. Home of the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)⮳ and [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)⮳ traits. | [[iterators | Iterators]] |
| [marker](https://doc.rust-lang.org/std/marker/index.html)⮳{{hi:marker}} | Primitive traits and types representing basic properties of types. Home of [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html)⮳, [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html)⮳, [`Sized`](https://doc.rust-lang.org/std/marker/trait.Sized.html)⮳, [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)⮳, and [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)⮳. | [[send_sync | Send & Sync]], [[ownership_borrowing | Ownership & Borrowing]], [[derive | Derive]] |
| [mem](https://doc.rust-lang.org/std/mem/index.html)⮳{{hi:mem}} | Basic functions for dealing with memory, such as [`drop()`](https://doc.rust-lang.org/std/mem/fn.drop.html)⮳. | [[memory-management | Memory Management]] |
| [net](https://doc.rust-lang.org/std/net/index.html)⮳{{hi:net}} | Networking primitives for TCP/UDP communication. Home of [`IpAddr`](https://doc.rust-lang.org/std/net/enum.IpAddr.html)⮳ and [`TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html)⮳. | [[network-programming | Network Programming]] |
| [num](https://doc.rust-lang.org/std/num/index.html)⮳{{hi:num}} | Additional functionality for numerics. Home of [`NonZero`](https://doc.rust-lang.org/std/num/struct.NonZero.html)⮳, [`Saturating`](https://doc.rust-lang.org/std/num/struct.Saturating.html)⮳, and [`Wrapping`](https://doc.rust-lang.org/std/num/struct.Wrapping.html)⮳ | [[data_types | Data Types]], [[mathematics | Mathematics]] |
| [ops](https://doc.rust-lang.org/std/ops/index.html)⮳{{hi:ops}} | Overloadable operators, e.g., [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)⮳, [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)⮳ and [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)⮳. Also home of [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html)⮳ and related traits; [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)⮳ and [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)⮳. | [[ops | Ops]], [[drop | Drop]], [[closures | Closures]], [[memory-management | Memory Management]] |
| [**option**](https://doc.rust-lang.org/std/option/index.html)⮳{{hi:option}} | Optional values via [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)⮳. | [[option | Option]] |
| [os](https://doc.rust-lang.org/std/os/index.html)⮳{{hi:os}} | OS-specific functionality.{{hi:OS}} | [[os | OS]] |
| [panic](https://doc.rust-lang.org/std/panic/index.html)⮳{{hi:panic}} | Panic support in the standard library. | [[error_handling | Error Handling]] |
| [**path**](https://doc.rust-lang.org/std/path/index.html)⮳{{hi:path}} | Cross-platform path manipulation with [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html)⮳ and [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html)⮳ | [[filesystem | Filesystem]] |
| [pin](https://doc.rust-lang.org/std/pin/index.html)⮳{{hi:pin}} | Types that pin data to a location in memory, via [`Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html)⮳. | [[pin | Pin]], [[rust_specific_patterns | Rust-specific Patterns]] |
| [prelude](https://doc.rust-lang.org/std/prelude/index.html)⮳{{hi:prelude}} | The prelude that Rust automatically imports into every Rust program. | [[code_organization | Code Organization]] |
| [primitive](https://doc.rust-lang.org/std/primitive/index.html)⮳{{hi:primitive}} | This module reexports the primitive types to allow usage that is not possibly shadowed by other declared types. | [[data_types | Data Types]] |
| [**process**](https://doc.rust-lang.org/std/process/index.html)⮳{{hi:process}} | A module for working with processes. Home of [`Command`](https://doc.rust-lang.org/std/process/struct.Command.html)⮳ and [`exit()`](https://doc.rust-lang.org/std/process/fn.exit.html)⮳. | [[external_commands | External Commands]] |
| [ptr](https://doc.rust-lang.org/std/ptr/index.html)⮳{{hi:ptr}} | Manually manage memory through raw pointers, e.g. [`NotNull`](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)⮳ and many unsafe functions. Advanced topic. | [[ownership_borrowing | Ownership & Borrowing]] |
| [**rc**](https://doc.rust-lang.org/std/rc/index.html)⮳{{hi:rc}} | Single-threaded reference-counting pointers. 'Rc' stands for "Reference Counted". See [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)⮳. | [[reference_counting | Reference Counting]] |
| [**result**](https://doc.rust-lang.org/std/result/index.html)⮳{{hi:result}} | Error handling with the [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)⮳ type. | [[result | Result]] |
| [slice](https://doc.rust-lang.org/std/slice/index.html)⮳{{hi:slice}} | Utilities for the [slice](https://doc.rust-lang.org/std/primitive.slice.html)⮳ primitive type. | [[slices | Slices]] |
| [str](https://doc.rust-lang.org/std/str/index.html)⮳{{hi:str}} | Utilities for the [`str`](https://doc.rust-lang.org/std/primitive.str.html)⮳ primitive type. | [[slices | Slices]], [[string | String]] |
| [**string**](https://doc.rust-lang.org/std/string/index.html)⮳{{hi:string}} | A UTF-8–encoded, growable [`String`](https://doc.rust-lang.org/std/string/struct.String.html)⮳. | [[string | String]] |
| [**sync**](https://doc.rust-lang.org/std/sync/index.html)⮳{{hi:sync}} | Useful synchronization primitives: [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)⮳, [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)⮳, [`LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)⮳. | [[concurrency | Concurrency]], [[smart_pointers | Smart Pointers]], [[sync | Sync]] |
| [task](https://doc.rust-lang.org/std/task/index.html)⮳{{hi:task}} | Types and Traits for working with asynchronous tasks. For advanced users. | [[asynchronous | Asynchronous]] |
| [**thread**](https://doc.rust-lang.org/std/thread/index.html)⮳{{hi:thread}} | Native threads with [`Thread`](https://doc.rust-lang.org/std/thread/struct.Thread.html)⮳. | [[explicit_thread | Explicit Thread]] |
| [time](https://doc.rust-lang.org/std/time/index.html)⮳{{hi:time}} | Temporal quantification with [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html)⮳ and [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html)⮳. | [[date-and-time | Date & Time]] |
| [**vec**](https://doc.rust-lang.org/std/vec/index.html)⮳{{hi:vec}} | A contiguous growable array type with heap-allocated contents, written [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)⮳. | [[vectors | Vectors]] |

The following covers portions of the Rust Standard Library that are not otherwise covered by another chapter: core types like `Option`, `Result`; smart pointers; and traits for conversions.

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

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
