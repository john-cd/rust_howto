//! Rather than running all tests in separate processes (by keeping them in
//! separate files in `tests`), we group all the tests in each section of the
//! book in the same crate.
//!
//! All examples in subfolders of `tests` MUST be added to `mod.rs` in their
//! folder, all modules / subfolders MUST be listed below, or tests won't be
//! discovered and won't be run.
//!
//! Note that tests that should run in separate processes (e.g. because of a
//! global variable, logger for example) are allowed to do so via a `rust_fork`
//! macro.

mod accessibility;
mod aerospace;
mod aerospace_drones;
mod aerospace_protocols;
mod aerospace_simulation;
mod aerospace_space_protocols;
mod aerospace_unmanned_aerial_vehicles;
mod algorithms;
mod api_bindings;
mod asynchronous;
mod authentication;
