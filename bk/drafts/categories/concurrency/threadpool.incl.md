<!-- TODO Reviewed ??-??-2026 -->

Use a thread pool for many jobs whose worker threads should be reused. For CPU-bound collection work, prefer Rayon and its parallel iterators; use `std::thread::scope` when workers need to borrow local data but a persistent pool is unnecessary. The `threadpool` crate offers a simple job queue, while async applications should use Tokio's `spawn_blocking` for blocking work and check the crate's maintenance fit before adding a dependency.

| Recipe                                                                        | Crates                                                                       | Categories                                                                  |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| [Hash ISO Files in Parallel][ex~concurrency~sha256-of-iso-files-concurrently] | {{#crate threadpool}} {{#crate walkdir}} {{#crate num_cpus}} {{#crate sha2}} | {{#categories concurrency=Concurrency filesystem=Filesystem}}               |
| [Render a Fractal with Rayon][ex~concurrency~draw-fractal-rayon]              | {{#crate rayon}} {{#crate num}} {{#crate image}}                             | {{#categories concurrency=Concurrency science=Science rendering=Rendering}} |
