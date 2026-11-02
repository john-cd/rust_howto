# Local patch

This source is `proc-macro-error2` 2.0.1 from crates.io. The compatibility fix
makes the `proc_macro` extern crate public, as required to avoid Rust's
future-incompatibility error (`E0365`).

Keep this patch while active dependencies require `proc-macro-error2`. Remove
it after Alloy, Shuttle, and Wasmer can resolve compatible versions using the
maintained `proc-macro-error3` crate together.
