// This file was generated with `cornucopia`. Do not modify.

#![expect(
    clippy::as_underscore,
    clippy::cast_possible_wrap,
    clippy::return_and_then,
    clippy::self_named_module_files,
    clippy::unwrap_used,
    clippy::use_self,
    reason = "Cornucopia 1.0 emits these patterns in its checked-in generated output."
)]

mod array_iterator;
pub mod client;
mod domain;
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries;
mod type_traits;
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types;
mod utils;
pub use array_iterator::ArrayIterator;
#[cfg(feature = "deadpool")]
pub use deadpool_postgres;
pub use domain::Domain;
pub use domain::DomainArray;
#[cfg(not(any(feature = "deadpool", feature = "wasm-async")))]
pub use postgres;
#[cfg(not(any(feature = "deadpool", feature = "wasm-async")))]
pub use postgres::fallible_iterator;
#[cfg(any(feature = "deadpool", feature = "wasm-async"))]
pub use tokio_postgres;
#[cfg(any(feature = "deadpool", feature = "wasm-async"))]
pub use tokio_postgres::fallible_iterator;
pub use type_traits::ArraySql;
pub use type_traits::BytesSql;
pub use type_traits::IterSql;
pub use type_traits::StringSql;
pub(crate) use utils::slice_iter;
