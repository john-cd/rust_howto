<!-- TODO Reviewed ??-??-2026 -->

# Model-Checking Concurrent Code with Loom

{{#include loom.incl.md}}

## Check a Publication and Cancellation Protocol with Loom {#loom-model-checking}

[![loom][c~loom~docs~badge]][c~loom~docs] [![loom~crates.io][c~loom~crates.io~badge]][c~loom~crates.io] [![loom~repo][c~loom~repo~badge]][c~loom~repo] [![loom~lib.rs][c~loom~lib.rs~badge]][c~loom~lib.rs]{{hi:loom}}{{hi:Concurrency}}{{hi:Model checking}}{{hi:Interleavings}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Development tools::Testing}}

An ordinary test runs one schedule of your threads and usually passes even when a rare interleaving is wrong. [`loom`][c~loom~docs]↗{{hi:loom}} is a permutation tester: [`loom::model`][c~loom::model~docs]↗{{hi:loom::model}} runs a closure many times and, within a bound, explores the thread schedules and the values that [atomic operations][ex~concurrency~atomics] may observe under the C11 memory model. If any execution fails an assertion, deadlocks, or races on a `loom::cell::UnsafeCell`, the test fails and reports the schedule.

The model below has two threads. A producer publishes a value and then raises a `ready` flag; the consumer may cancel the producer and reads the value only after it observes `ready`. The invariant is that a reader that sees `ready` always sees the published value, and `ready` is set exactly when the producer reports that it published. The correct protocol pairs a `Release` store with an `Acquire` load. The second test uses `Relaxed` for both and expects Loom to find the stale read.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/loom_model.rs:example}}
```

### Run the Model Separately from Ordinary Tests

The model tests are named `loom_*` and live in their own example, so you can run just the model (from `bk`, the example crates live in `crates`):

```sh
just ntt loom
cargo test -p concurrency --example loom_model
```

Tune the exploration with environment variables instead of editing the code: `LOOM_MAX_PREEMPTIONS` bounds the number of forced thread switches, `LOOM_MAX_BRANCHES` bounds the branches per execution, and `LOOM_LOG=1` prints the failing schedule. Loom is a test-only dependency, so it never ships in your binaries.

In a real library, do not depend on `loom` in production code. Define the sync types behind a small module that re-exports `std::sync` normally and `loom::sync` under `#[cfg(loom)]`, declare `cfg(loom)` in `[lints.rust] unexpected_cfgs`, and run the model with `RUSTFLAGS="--cfg loom" cargo test --release`. This keeps the normal test run fast and reserves the exhaustive exploration for a dedicated CI job.

### What the Model Does Not Cover

- Only code that uses `loom::sync`, `loom::thread` and `loom::cell` is explored. Uninstrumented `std::sync` primitives, and the internals of external libraries (allocators, channels, `tokio`, `parking_lot`), run as ordinary sequential code inside the model and their interleavings are not examined.
- The state space grows exponentially with the number of threads and operations. The preemption bound makes the run finish but means Loom checks every schedule _within the bound_, not every possible schedule; keep models to two or three threads and a handful of operations.
- Loom models the C11 memory model approximately. It does not support every ordering (for example, `SeqCst` fences and some relaxed behaviors are approximated), and it cannot test hardware effects, timing, I/O, or `unsafe` code that bypasses its types.
- A passing model shows that the _model_ is correct within its bounds. It does not prove that arbitrary production code is correct; keep the model small, mirror the real protocol faithfully, and combine it with ordinary tests, Miri and review.

See [Atomics][ex~concurrency~atomics] and [Shared-state Concurrency][ex~concurrency~shared-state] for the primitives being modeled, and [Testing][ex~development-tools_testing~testing] for the wider testing toolbox.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  fix
</div>
