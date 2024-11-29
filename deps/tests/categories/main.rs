//! Rather than running all tests in separate processes (by keeping them in
//! separate files in deps/tests), we group all the tests in each section of the
//! book in the same crate.
//!
//! All examples in subfolders of `tests` MUST be added to `mod.rs` in their
//! folder, all modules / subfolders MUST be listed below, or tests won't be
//! discovered and won't be run.
//!
//! Note that tests that should run in separate processes (e.g. because of a
//! global variable, logger for example) are allowed to do so via a `rust_fork`
//! macro.

mod algorithms;
mod api_bindings;
mod asynchronous;
mod authentication;
mod caching;
mod command_line_interface;
mod compression;
mod concurrency;
mod config;
mod cryptography;
mod data_structures;
mod database;
mod date_and_time;
mod development_tools;
mod development_tools_build_utils;
mod development_tools_debugging;
mod development_tools_testing;
mod email;
mod encoding;
mod filesystem;
mod hardware_support;
mod mathematics;
mod memory_management;
mod network_programming;
mod os;
mod rust_patterns;
mod text_processing;
mod web_programming;
mod web_programming_http_client;
mod web_programming_http_server;
