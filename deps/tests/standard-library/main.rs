//! Rather than running all tests in separate processes (by keeping them in
//! separate files in deps/tests), we group all the tests in each section of the
//! book in the same crate.
//!
//! All examples in this folder MUST be listed as module below or they won't be
//! discovered / run.

mod cow1;
mod cow2;
mod cow3;
mod cow4;
mod cow5;
mod cow6;
mod cow7;
mod derive;
mod derive_more;
mod hashmaps;
mod options1;
mod options2;
mod options3;
mod strings;
mod strings2;

mod vectors;
