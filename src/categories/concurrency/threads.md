# Threads

{{#include threads.incl.md}}

## Spawn a short-lived thread

[![crossbeam][c-crossbeam-badge]][c-crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency] {{hi:short-lived thread}}

The example uses the [`crossbeam`][c-crossbeam]{{hi:crossbeam}}⮳ crate, which provides data structures and functions for concurrent and parallel programming{{hi:parallel programming}}. [`crossbeam::thread::Scope::spawn`][c-crossbeam::thread::Scope::spawn]{{hi:crossbeam::thread::Scope::spawn}}⮳ spawns a new scoped thread that is guaranteed to terminate before returning from the closure that passed into [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ function, meaning that you can reference data from the calling function.

This example splits the array in half and performs the work in separate threads.

```rust,editable
{{#include ../../../deps/tests/crossbeam-spawn.rs}}
```

## Create a parallel pipeline

[![crossbeam][c-crossbeam-badge]][c-crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example uses the [`crossbeam`][c-crossbeam]{{hi:crossbeam}}⮳ and [`crossbeam_channel`][c-crossbeam_channel]{{hi:crossbeam_channel}}⮳ crates to create a parallel pipeline{{hi:parallel pipeline}}, similar to that described in the ZeroMQ [guide][zero-mq-guide-website]⮳. There is a data source and a data sink, with data being processed by two worker threads in parallel on its way from the source to the sink.

We use bounded channels with a capacity of one using
[`crossbeam_channel::bounded`][c-crossbeam_channel::bounded]{{hi:crossbeam_channel::bounded}}⮳. The producer must be on its own thread because it produces messages faster than the workers can process them (since they sleep for half a second) - this means the producer blocks on the call to
[`crossbeam_channel::Sender::send`][c-crossbeam_channel::Sender::send]{{hi:crossbeam_channel::Sender::send}}⮳ for half a second until one of the workers processes the data in the channel. Also note that the data in the channel is consumed by whichever worker calls receive first, so each message is delivered to a single worker rather than both workers.

Reading from the channels via the iterator
[`crossbeam_channel::Receiver::iter`][c-crossbeam_channel::Receiver::iter]{{hi:crossbeam_channel::Receiver::iter}}⮳ method will block, either waiting for new messages or until the channel is closed. Because the channels were created within the [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ we must manually close them via [`std::ops::Drop`][c-std::ops::Drop]{{hi:std::ops::Drop}}⮳ to prevent the entire program from blocking on the worker for-loops. You can think of the calls to [`std::ops::Drop`][c-std::ops::Drop]{{hi:std::ops::Drop}}⮳ as signaling that no more messages will be sent.

```rust
{{#include ../../../deps/tests/crossbeam-complex.rs}}
```

## Pass data between two threads

[![crossbeam][c-crossbeam-badge]][c-crossbeam]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

This example demonstrates the use of [`crossbeam_channel`][c-crossbeam_channel]{{hi:crossbeam_channel}}⮳ in a single producer, single consumer{{hi:single producer, single consumer}} (SPSC) setting. We build off the [`crossbeam spawn`][ex-crossbeam-spawn]{{hi:crossbeam spawn}}⮳ example by using [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ and [`crossbeam::thread::Scope::spawn`][c-crossbeam::thread::Scope::spawn]{{hi:crossbeam::thread::Scope::spawn}}⮳ to manage the producer thread. Data is exchanged between the two threads using a [`crossbeam::scope`][c-crossbeam::scope]{{hi:crossbeam::scope}}⮳ channel, meaning there is no limit to the number of storable messages{{hi:messages}}. The producer thread sleeps for half a second in between messages.

```rust,editable
{{#include ../../../deps/tests/crossbeam-spsc.rs}}
```

## Maintain global mutable state

[![lazy_static][c-lazy_static-badge]][c-lazy_static]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency] {{hi:global mutable state}}

Declare global state using [`lazy static`][c-lazy_static]{{hi:lazy static}}. [`lazy static`][c-lazy_static]⮳ creates a globally available `static ref` which requires a [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳ to allow mutation (also see [`std::sync::RwLock`][c-std::sync::RwLock]{{hi:std::sync::RwLock}}⮳). The [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳ wrap ensures the state cannot be simultaneously accessed by multiple threads, preventing race conditions. A [`std::sync::MutexGuard`][c-std::sync::MutexGuard]{{hi:std::sync::MutexGuard}}⮳ must be acquired to read or mutate the value stored in a [`std::sync::Mutex`][c-std::sync::Mutex]{{hi:std::sync::Mutex}}⮳.

```rust,editable
{{#include ../../../deps/tests/global-mut-state.rs}}
```

## Calculate SHA256 sum of iso files concurrently

[![threadpool][c-threadpool-badge]][c-threadpool]  [![num_cpus][c-num_cpus-badge]][c-num_cpus]  [![walkdir][c-walkdir-badge]][c-walkdir]  [![ring][c-ring-badge]][c-ring]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency][![cat-filesystem][cat-filesystem-badge]][cat-filesystem]

This example calculates the SHA256{{hi:SHA256}} for every file with iso extension in the current directory. A threadpool{{hi:threadpool}} generates threads equal to the number of cores present in the system found with [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳. [`walkdir::Walkdir::new`][c-walkdir::Walkdir::new]{{hi:walkdir::Walkdir::new}}⮳ iterates the current directory and calls [`walkdir::Walkdir::new`][c-walkdir::Walkdir::new]{{hi:walkdir::Walkdir::new}}⮳ to perform the operations of reading and computing SHA256 hash.

```rust,editable,no_run
{{#include ../../../deps/tests/threadpool-walk.rs}}
```

## Draw fractal dispatching work to a thread pool

[![threadpool][c-threadpool-badge]][c-threadpool]  [![num][c-num-badge]][c-num]  [![num_cpus][c-num_cpus-badge]][c-num_cpus]  [![image][c-image-badge]][c-image]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency][![cat-science][cat-science-badge]][cat-science][![cat-rendering][cat-rendering-badge]][cat-rendering]

This example generates an image by drawing a fractal from the [Julia set][julia-set]⮳ with a thread pool{{hi:thread pool}} for distributed computation.

[![julia-set]( https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png )]( https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png )

Allocate memory for output image of given width and height with [`image::ImageBuffer::new`][c-image::ImageBuffer::new]{{hi:image::ImageBuffer::new}}⮳.
[`image::Rgb::from_channels`][c-image::Rgb::from_channels]{{hi:image::Rgb::from_channels}}⮳ calculates RGB pixel values. Create [`threadpool::ThreadPool`][c-threadpool::ThreadPool]{{hi:threadpool::ThreadPool}}⮳ with thread count equal to number of cores with [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳.
[`threadpool::ThreadPool::execute`][c-threadpool::ThreadPool::execute]{{hi:threadpool::ThreadPool::execute}}⮳ receives each pixel as a separate job.

[`mpsc::channel`][c-mpsc::channel]{{hi:mpsc::channel}}⮳ receives the jobs and [`std::sync::mpsc::Receiver::recv`][c-std::sync::mpsc::Receiver::recv]{{hi:std::sync::mpsc::Receiver::recv}}⮳ retrieves them.
[`image::ImageBuffer::put_pixel`][c-image::ImageBuffer::put_pixel]{{hi:image::ImageBuffer::put_pixel}}⮳ uses the data to set the pixel color.
[`image::ImageBuffer::save`][c-image::ImageBuffer::save]{{hi:image::ImageBuffer::save}}⮳ writes the image to `output.png`.

```rust,editable,no_run
{{#include ../../../deps/tests/threadpool-fractal.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
