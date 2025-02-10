# Threadpools

{{#include threadpool.incl.md}}

## Calculate the SHA256 of ISO files concurrently {#sha256-of-iso-files-concurrently}

[![threadpool][c-threadpool-badge]][c-threadpool]{{hi:threadpool}} [![num_cpus][c-num_cpus-badge]][c-num_cpus]{{hi:num_cpus}} [![walkdir][c-walkdir-badge]][c-walkdir]{{hi:walkdir}} [![ring][c-ring-badge]][c-ring]{{hi:ring}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency] [{{hi:Concurrency}}![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

This example calculates the SHA256{{hi:SHA256}} for every file with iso extension in the current directory. A threadpool{{hi:Thread pools}} generates threads equal to the number of cores present in the system found with [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳. [`walkdir::WalkDir::new`][c-walkdir::WalkDir::new]{{hi:walkdir::WalkDir::new}}⮳ iterates the current directory and calls [`walkdir::WalkDir::new`][c-walkdir::WalkDir::new]{{hi:walkdir::WalkDir::new}}⮳ to perform the operations of reading and computing SHA256 hash.

```rust,editable
{{#include ../../../crates/ex/cats/concurrency/tests/threadpool/threadpool_walk.rs:example}}
```

## Draw a fractal, dispatching work to a thread pool {#draw-fractal-threadpool}

[![threadpool][c-threadpool-badge]][c-threadpool]{{hi:threadpool}} [![num][c-num-badge]][c-num]{{hi:num}} [![num_cpus][c-num_cpus-badge]][c-num_cpus]{{hi:num_cpus}} [![image][c-image-badge]][c-image]{{hi:image}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-science][cat-science-badge]][cat-science]{{hi:science}} [![cat-rendering][cat-rendering-badge]][cat-rendering]{{hi:Rendering}}

This example generates an image by drawing a fractal from the [Julia set][web-julia-set]⮳ with a thread pool{{hi:Thread pools}} for distributed computation.

[![julia-set][web-julia-set]][web-julia-set]

[web-julia-set]: https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png

Allocate memory for output image of given width and height with [`image::ImageBuffer::new`][c-image::ImageBuffer::new]{{hi:image::ImageBuffer::new}}⮳.
[`image::Rgb::from_channels`][c-image::Rgb::from_channels]{{hi:image::Rgb::from_channels}}⮳ calculates RGB pixel values. Create [`threadpool::ThreadPool`][c-threadpool::ThreadPool]{{hi:threadpool::ThreadPool}}⮳ with thread count equal to number of cores with [`num_cpus::get`][c-num_cpus::get]{{hi:num_cpus::get}}⮳.
[`threadpool::ThreadPool::execute`][c-threadpool::ThreadPool::execute]{{hi:threadpool::ThreadPool::execute}}⮳ receives each pixel as a separate job.

[`std::sync::mpsc::channel`][c-std::sync::mpsc::channel]{{hi:std::sync::mpsc::channel}}⮳ receives the jobs and [`std::sync::mpsc::Receiver::recv`][c-std::sync::mpsc::Receiver::recv]{{hi:std::sync::mpsc::Receiver::recv}}⮳ retrieves them.
[`image::ImageBuffer::put_pixel`][c-image::ImageBuffer::put_pixel]{{hi:image::ImageBuffer::put_pixel}}⮳ uses the data to set the pixel color.
[`image::ImageBuffer::save`][c-image::ImageBuffer::save]{{hi:image::ImageBuffer::save}}⮳ writes the image to `output.png`.

```rust,editable,noplayground
{{#include ../../../crates/ex/cats/concurrency/tests/threadpool/threadpool_fractal.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[threadpool: polish (P1)](https://github.com/john-cd/rust_howto/issues/267) threadpool_fractal.rs is noplayground - linking with `cc` failed: exit status: 1 - fix?
</div>
