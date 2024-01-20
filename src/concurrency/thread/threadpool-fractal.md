## Draw fractal dispatching work to a thread pool

[![threadpool-badge]][threadpool] [![num-badge]][num] [![num-cpus-badge]][num-cpus] [![image-badge]][image] [![cat-concurrency-badge]][cat-concurrency][![cat-science-badge]][cat-science][![cat-rendering-badge]][cat-rendering]

This example generates an image by drawing a fractal from the [Julia set]
with a thread pool for distributed computation.

<a href="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png"><img src="https://cloud.githubusercontent.com/assets/221000/26546700/9be34e80-446b-11e7-81dc-dd9871614ea1.png" width="150" /></a>

Allocate memory for output image of given width and height with [`ImageBuffer::new`].
[`Rgb::from_channels`] calculates RGB pixel values.
Create [`ThreadPool`] with thread count equal to number of cores with [`num-cpus::get`].
[`ThreadPool::execute`] receives each pixel as a separate job.

[`mpsc::channel`] receives the jobs and [`Receiver::recv`] retrieves them.
[`ImageBuffer::put_pixel`] uses the data to set the pixel color.
[`ImageBuffer::save`] writes the image to `output.png`.

```rust,editable,no_run
{#include ../../../deps/examples/threadpool-fractal.rs}
```

[`ImageBuffer::new`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.new
[`ImageBuffer::put_pixel`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.put_pixel
[`ImageBuffer::save`]: https://docs.rs/image/*/image/struct.ImageBuffer.html#method.save
[`mpsc::channel`]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
[`num-cpus::get`]: https://docs.rs/num-cpus/*/num-cpus/fn.get.html
[`Receiver::recv`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv
[`Rgb::from_channels`]: https://docs.rs/image/*/image/struct.Rgb.html#method.from_channels
[`ThreadPool`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html
[`ThreadPool::execute`]: https://docs.rs/threadpool/*/threadpool/struct.ThreadPool.html#method.execute

[Julia set]: https://en.wikipedia.org/wiki/Julia_set
