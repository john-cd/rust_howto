//! \ Generated\ database\ models,\ query\ bindings,\ and\ client\ helpers\ for\ this\ crate\.
//!
//! TODO Reviewed ??-??-2026.

#![expect(
    clippy::as_underscore,
    clippy::cast_possible_wrap,
    clippy::return_and_then,
    clippy::self_named_module_files,
    clippy::unwrap_used,
    clippy::use_self,
    reason = "Cornucopia 1.0 emits these patterns in its checked-in generated output."
)]

/// This module provides the `array_iterator` implementation used by its parent.
mod array_iterator;
/// This module provides the `client` implementation used by its parent.
pub mod client;
/// This module provides the `domain` implementation used by its parent.
mod domain;
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
/// This module provides the `queries` implementation used by its parent.
#[allow(dead_code)]
pub mod queries;
/// This module provides the `type_traits` implementation used by its parent.
mod type_traits;
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
/// This module provides the `types` implementation used by its parent.
#[allow(dead_code)]
pub mod types;
/// This module provides the `utils` implementation used by its parent.
mod utils;
pub use array_iterator::ArrayIterator;
#[cfg(feature = "deadpool")]
pub use deadpool_postgres;
pub use domain::Domain;
pub use domain::DomainArray;
#[cfg(feature = "postgres")]
pub use postgres;
#[cfg(all(
    feature = "postgres",
    not(any(feature = "deadpool", feature = "wasm-async"))
))]
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
