# Threads

{{#include threads.incl.md}}

## Spawn a short-lived thread

[![crossbeam][c-crossbeam-badge]][c-crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency] {{hi:short-lived thread}}

The example uses the {{hi:crossbeam}}[`crossbeam`][c-crossbeam]⮳ crate, which provides data structures and functions for concurrent and {{i:parallel programming}}. {{hi:Scope::spawn}}[`Scope::spawn`][c-crossbeam::thread::Scope::spawn]⮳ spawns a new scoped thread that is guaranteed to terminate before returning from the closure that passed into {{hi:crossbeam::scope}}[`crossbeam::scope`][c-crossbeam::scope]⮳ function, meaning that you can reference data from the calling function.

This example splits the array in half and performs the work in separate threads.

```rust,editable
{{#include ../../../deps/tests/crossbeam-spawn.rs}}
```

## Create a parallel pipeline

[![crossbeam][c-crossbeam-badge]][c-crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example uses the {{hi:crossbeam}}[`crossbeam`][c-crossbeam]⮳ and {{hi:crossbeam_channel}}[`crossbeam_channel`][c-crossbeam_channel]⮳ crates to create a {{i:parallel pipeline}}, similar to that described in the ZeroMQ [guide][zero-mq-guide-website]⮳. There is a data source and a data sink, with data being processed by two worker threads in parallel on its way from the source to the sink.

We use bounded channels with a capacity of one using
{{hi:crossbeam_channel::bounded}}[`crossbeam_channel::bounded`][c-crossbeam_channel::bounded]⮳. The producer must be on its own thread because it produces messages faster than the workers can process them (since they sleep for half a second) - this means the producer blocks on the call to
{{hi:crossbeam_channel::Sender::send}}[`crossbeam_channel::Sender::send`][c-crossbeam_channel::Sender::send]⮳ for half a second until one of the workers processes the data in the channel. Also note that the data in the channel is consumed by whichever worker calls receive first, so each message is delivered to a single worker rather than both workers.

Reading from the channels via the iterator
{{hi:crossbeam_channel::Receiver::iter}}[`crossbeam_channel::Receiver::iter`][c-crossbeam_channel::Receiver::iter]⮳ method will block, either waiting for new messages or until the channel is closed. Because the channels were created within the {{hi:crossbeam::scope}}[`crossbeam::scope`][c-crossbeam::scope]⮳ we must manually close them via {{hi:drop}}[`drop`][c-std::ops::Drop]⮳ to prevent the entire program from blocking on the worker for-loops. You can think of the calls to {{hi:drop}}[`drop`][c-std::ops::Drop]⮳ as signaling that no more messages will be sent.

```rust
{{#include ../../../deps/tests/crossbeam-complex.rs}}
```

## Pass data between two threads

[![crossbeam][c-crossbeam-badge]][c-crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example demonstrates the use of {{hi:crossbeam_channel}}[`crossbeam_channel`][c-crossbeam_channel]⮳ in a {{i:single producer, single consumer}} (SPSC) setting. We build off the {{hi:crossbeam spawn}}[`crossbeam spawn`][ex-crossbeam-spawn]⮳ example by using {{hi:crossbeam::scope}}[`crossbeam::scope`][c-crossbeam::scope]⮳ and {{hi:Scope::spawn}}[`Scope::spawn`][c-crossbeam::thread::Scope::spawn]⮳ to manage the producer thread. Data is exchanged between the two threads using a {{hi:crossbeam_channel::unbounded}}[`crossbeam_channel::unbounded`][c-crossbeam::scope]⮳ channel, meaning there is no limit to the number of storable {{i:messages}}. The producer thread sleeps for half a second in between messages.

```rust,editable
{{#include ../../../deps/tests/crossbeam-spsc.rs}}
```

## Maintain global mutable state

[![lazy_static][c-lazy_static-badge]][c-lazy_static]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency] {{hi:global mutable state}}

Declare global state using {{hi:lazy static}}[`lazy static`][c-lazy_static]. {{hi:lazy static}}[`lazy static`][c-lazy_static]⮳ creates a globally available `static ref` which requires a {{hi:Mutex}}[`Mutex`][c-std::sync::Mutex]⮳ to allow mutation (also see {{hi:RwLock}}[`RwLock`][c-std::sync::RwLock]⮳). The {{hi:Mutex}}[`Mutex`][c-std::sync::Mutex]⮳ wrap ensures the state cannot be simultaneously accessed by multiple threads, preventing race conditions. A {{hi:MutexGuard}}[`MutexGuard`][c-std::sync::MutexGuard]⮳ must be acquired to read or mutate the value stored in a {{hi:Mutex}}[`Mutex`][c-std::sync::Mutex]⮳.

```rust,editable
{{#include ../../../deps/tests/global-mut-state.rs}}
```

## Calculate SHA256 sum of iso files concurrently

[![threadpool][c-threadpool-badge]][c-threadpool]  [![num_cpus][c-num_cpus-badge]][c-num_cpus]  [![walkdir][c-walkdir-badge]][c-walkdir]  [![ring][c-ring-badge]][c-ring]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency][![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

This example calculates the {{i:SHA256}} for every file with iso extension in the current directory. A {{i:threadpool}} generates threads equal to the number of cores present in the system found with {{hi:num_cpus::get}}[`num_cpus::get`][c-num_cpus::get]⮳. {{hi:Walkdir::new}}[`Walkdir::new`][c-walkdir::Walkdir::new]⮳ iterates the current directory and calls {{hi:execute}}[`execute`][c-walkdir::Walkdir::new]⮳ to perform the operations of reading and computing SHA256 hash.

```rust,editable,no_run
{{#include ../../../deps/tests/threadpool-walk.rs}}
```

## Draw fractal dispatching work to a thread pool

[![threadpool][c-threadpool-badge]][c-threadpool]  [![num][c-num-badge]][c-num]  [![num_cpus][c-num_cpus-badge]][c-num_cpus]  [![image][c-image-badge]][c-image]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency][![cat-science][cat-science-badge]][cat-science][![cat-rendering][cat-rendering-badge]][cat-rendering]

This example generates an image by drawing a fractal from the [Julia set][julia-set]⮳ with a {{i:thread pool}} for distributed computation.

[![julia-set]( https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png )]( https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png )

Allocate memory for output image of given width and height with {{hi:ImageBuffer::new}}[`ImageBuffer::new`][c-image::ImageBuffer::new]⮳.
{{hi:Rgb::from_channels}}[`Rgb::from_channels`][c-image::Rgb::from_channels]⮳ calculates RGB pixel values. Create {{hi:ThreadPool}}[`ThreadPool`][c-threadpool::ThreadPool]⮳ with thread count equal to number of cores with {{hi:num_cpus::get}}[`num_cpus::get`][c-num_cpus::get]⮳.
{{hi:ThreadPool::execute}}[`ThreadPool::execute`][c-threadpool::ThreadPool::execute]⮳ receives each pixel as a separate job.

{{hi:mpsc::channel}}[`mpsc::channel`][c-mpsc::channel]⮳ receives the jobs and {{hi:Receiver::recv}}[`Receiver::recv`][c-std::sync::mpsc::Receiver::recv]⮳ retrieves them.
{{hi:ImageBuffer::put_pixel}}[`ImageBuffer::put_pixel`][c-image::ImageBuffer::put_pixel]⮳ uses the data to set the pixel color.
{{hi:ImageBuffer::save}}[`ImageBuffer::save`][c-image::ImageBuffer::save]⮳ writes the image to `output.png`.

```rust,editable,no_run
{{#include ../../../deps/tests/threadpool-fractal.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
