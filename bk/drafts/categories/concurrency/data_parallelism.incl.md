<!-- TODO Reviewed ??-??-2026 -->

Data parallelism applies an operation across independent items or tasks. These recipes use Rayon; there is no separate `multithreading.md` page in the current draft tree, so the Rayon guidance has one home here.

| Recipe                                                                             | Crates                                            | Categories                              |
| ---------------------------------------------------------------------------------- | ------------------------------------------------- | --------------------------------------- |
| [Use Rayon for Parallel Iteration][ex~concurrency~rayon]                           | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Apply a Parallel Iterator][ex~concurrency~par-iter]                               | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Split Independent Tasks with `rayon::join`][ex~concurrency~custom-parallel-tasks] | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Mutate a Collection in Parallel][ex~concurrency~mutate-array-in-parallel]         | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Test a Predicate Across a Collection][ex~concurrency~any-or-all-in-parallel]      | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Find a Matching Item in Parallel][ex~concurrency~search-in-parallel]              | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Sort Collections in Parallel][ex~concurrency~parallel-sorting]                    | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Map and Reduce Parallel Results][ex~concurrency~map-reduce-in-parallel]           | {{#crate rayon}}                                  | {{#categories concurrency=Concurrency}} |
| [Generate JPEG Thumbnails in Parallel][ex~concurrency~thumbnails-in-parallel]      | {{#crate rayon}} {{#crate glob}} {{#crate image}} | {{#categories concurrency=Concurrency}} |
