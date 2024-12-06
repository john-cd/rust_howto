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
mod caching;
mod command_line_interface;
mod compression;
mod computer_vision;
mod concurrency;
mod config;
mod cryptography;
mod cryptography_cryptocurrencies;
mod data_structures;
mod database;
mod database_implementations;
mod date_and_time;
mod development_tools;
mod development_tools_build_utils;
mod development_tools_debugging;
mod development_tools_ffi;
mod development_tools_procedural_macro_helpers;
mod development_tools_profiling;
mod development_tools_testing;
mod email;
mod embedded;
mod emulators;
mod encoding;
mod external_ffi_bindings;
mod filesystem;
mod finance;
mod game_development;
mod game_engines;
mod games;
mod graphics;
mod gui;
mod hardware_support;
mod internationalization;
mod localization;
mod mathematics;
mod memory_management;
mod multimedia;
mod multimedia_audio;
mod multimedia_encoding;
mod multimedia_images;
mod multimedia_video;
mod network_programming;
mod no_std;
mod no_std_no_alloc;
mod os;
mod os_freebsd_apis;
mod os_linux_apis;
mod os_macos_apis;
mod os_unix_apis;
mod os_windows_apis;
mod parser_implementations;
mod parsing;
mod rendering;
mod rendering_data_formats;
mod rendering_engine;
mod rendering_graphics_api;
mod rust_patterns;
mod science;
mod science_geo;
mod science_neuroscience;
mod science_robotics;
mod simulation;
mod template_engine;
mod text_processing;
mod value_formatting;
mod virtualization;
mod visualization;
mod wasm;
mod web_programming;
mod web_programming_http_client;
mod web_programming_http_server;
mod web_programming_websocket;
