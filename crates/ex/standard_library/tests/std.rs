//! Rather than running all tests in separate processes (by keeping them in
//! separate files in `tests`), we group all the tests in each section of the
//! book in the same crate.
//!
//! All examples in this folder MUST be listed as module below (or in submodules) or they won't be
//! discovered / run.
mod cow;
mod option;
mod other;
