//! Rather than running all tests in separate processes (by keeping them in
//! separate files in deps/tests), we group all the tests in each section of the
//! book in the same crate.
//!
//! All examples in this folder MUST be listed as module below or they won't be
//! discovered / run.

mod cloud;
mod cross_platform;
mod data_processing;
mod gpu;
mod scripting;
mod written_in_rust;
