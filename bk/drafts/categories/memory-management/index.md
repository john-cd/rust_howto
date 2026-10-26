<!-- TODO Reviewed ??-??-2026 -->

# Memory Management

[![cat~memory-management][cat~memory-management~badge]][cat~memory-management]

Rust's ownership, borrowing, and lifetimes handle most memory management without a garbage collector or manual allocation calls. This index links to the cases where choosing a pointer, initialization strategy, allocator, or diagnostic tool matters.

## Choose an Ownership Strategy

| Need                                            | Starting point                                                                                        | Notes                                                                                                                             |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Store a value on the heap with one owner        | [`Box`][c~std::boxed::Box~docs]↗{{hi:Box}}                                                            | Use when a value has a dynamic size, needs indirection, or must be owned by a trait object.                                       |
| Share read-only data within one thread          | [`Rc`][c~std::rc::Rc~docs]↗{{hi:Rc}}                                                                  | Reference counting does not make the value thread-safe.                                                                           |
| Share data across threads                       | [`Arc`][c~std::sync::Arc~docs]↗{{hi:Arc}}                                                             | The inner type must also satisfy the thread-safety bounds. Pair with a lock or atomic type only when shared mutation is required. |
| Mutate through a shared reference in one thread | [`Cell`][c~std::cell::Cell~docs]↗{{hi:Cell}} or [`RefCell`][c~std::cell::RefCell~docs]↗{{hi:RefCell}} | `Cell` offers interior mutation for suitable values; `RefCell` checks borrows at runtime and panics on a conflicting borrow.      |
| Coordinate shared mutation across threads       | [`Mutex`][c~std::sync::Mutex~docs]↗{{hi:Mutex}} or [`RwLock`][c~std::sync::RwLock~docs]↗{{hi:RwLock}} | Locking adds contention and possible poisoning; prefer message passing or owned data when that is simpler.                        |

See [[smart_pointers | Smart Pointers]] for examples and trade-offs. Use raw pointers and [`unsafe`][keyword~unsafe]↗{{hi:unsafe}} code only when a safe abstraction cannot express the requirement, such as a carefully designed FFI boundary.

## Static Data and Lazy Initialization

A constant is inlined as a value; a static item has one fixed storage location. Prefer immutable statics for process-wide constants. Avoid mutable global state because it hides ownership and synchronization requirements.

For values that should be initialized on first use, use the standard library's lazy initialization types where they fit. See [[lazy_initialization | Lazy Initialization]] and [[shared_state | Shared State]].

{{#include lazy_initialization.incl.md}}

## Allocation and Reclamation

Use Rust's normal collection and ownership types before choosing a custom allocator. The [`alloc`][c~alloc~docs]↗{{hi:alloc}} APIs are low-level building blocks; most applications should allocate through collections or libraries that own their storage.

A bump or arena allocator can help when many objects share one lifetime. Pools and slabs can fit workloads with repeated allocations of similarly sized objects. Consider these techniques only after profiling shows allocation cost or fragmentation is a real constraint, and account for the lifetime and memory-retention trade-offs.

Rust does not provide a tracing garbage collector by default. If an application needs tracing collection, it must choose a library and design its ownership and interoperability explicitly.

{{#include memory_allocation.incl.md}}

{{#include arena_allocation.incl.md}}

{{#include string_interning.incl.md}}

## Diagnose Memory Use and Safety

Measure allocation volume and retained memory with a profiler before optimizing. [`Valgrind`][c~valgrind~docs]↗{{hi:Valgrind}} and [`heaptrack`][c~heaptrack~docs]↗{{hi:heaptrack}} can help inspect memory behavior on supported platforms.

AddressSanitizer and MemorySanitizer can find classes of memory errors in supported toolchain and target configurations. They do not replace tests or explain whether an allocation pattern is efficient. Check the compiler and platform requirements before enabling them.

See [[memory_usage_analysis | Memory Usage Analysis]] and [[development-tools_profiling | Development Tools: Profiling]].

## Related Topics

- [[smart_pointers | Smart Pointers]]
- [[lazy_initialization | Lazy Initialization]]
- [[shared_state | Shared State]]
- [[development-tools_profiling | Development Tools: Profiling]]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  memory-management/index: organize; align table and sections; write missing sections; cross link
</div>
