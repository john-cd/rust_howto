//! \ Generated\ conversions\ and\ trait\ implementations\ for\ database\ query\ values\.
//!
//! TODO Reviewed ??-??-2026.

use std::borrow::Cow;

use postgres_protocol::types::ArrayDimension;
use postgres_protocol::types::{self};
use postgres_types::IsNull;
use postgres_types::Kind;
use postgres_types::ToSql;
use postgres_types::Type;
use postgres_types::private::BytesMut;
use postgres_types::to_sql_checked;

use super::domain::escape_domain_to_sql;

/// Specifies the operations implementations provide to cornucopia queries callers.
pub trait StringSql: std::fmt::Debug + ToSql + Sync {}
impl<T: StringSql> StringSql for &T {}
impl StringSql for String {}
impl StringSql for &str {}
impl StringSql for Cow<'_, str> {}
impl StringSql for Box<str> {}

/// Specifies the operations implementations provide to cornucopia queries callers.
pub trait BytesSql: std::fmt::Debug + ToSql + Send + Sync {}
impl<T: BytesSql> BytesSql for &T {}
impl BytesSql for Vec<u8> {}
impl BytesSql for &[u8] {}

/// Specifies the operations implementations provide to cornucopia queries callers.
pub trait ArraySql: std::fmt::Debug + ToSql + Send + Sync {
    /// Type of the item values produced or consumed by this trait.
    type Item;
    /// Escapes domain to sql characters so the value is safe in its output context.
    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>>;
}
impl<T: std::fmt::Debug + ToSql + Sync, A: ArraySql<Item = T>> ArraySql for &A {
    /// Alias for `T` used to represent item.
    type Item = T;

    /// Escapes domain to sql characters so the value is safe in its output context.
    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        A::escape_domain_to_sql(self, ty, w)
    }
}
impl<T: std::fmt::Debug + ToSql + Send + Sync> ArraySql for Vec<T> {
    /// Alias for `T` used to represent item.
    type Item = T;

    /// Escapes domain to sql characters so the value is safe in its output context.
    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        escape_domain_to_sql(ty, w, self.iter())
    }
}
impl<T: std::fmt::Debug + ToSql + Sync> ArraySql for &[T] {
    /// Alias for `T` used to represent item.
    type Item = T;

    /// Escapes domain to sql characters so the value is safe in its output context.
    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        escape_domain_to_sql(ty, w, self.iter())
    }
}
impl<
    T: std::fmt::Debug + ToSql + Send + Sync,
    I: Iterator<Item = T> + ExactSizeIterator,
    F: Fn() -> I + Send + Sync,
> ArraySql for IterSql<T, I, F>
{
    /// Alias for `T` used to represent item.
    type Item = T;

    /// Escapes domain to sql characters so the value is safe in its output context.
    fn escape_domain_to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        escape_domain_to_sql(ty, w, (self.0)())
    }
}

/// Groups the data required by cornucopia queries operations.
pub struct IterSql<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync>(
    /// Factory that creates a fresh iterator for each SQL query.
    pub F,
);
impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> std::fmt::Debug
    for IterSql<T, I, F>
{
    /// Formats the value for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ArrayFn").finish()
    }
}
impl<T: ToSql, I: Iterator<Item = T> + ExactSizeIterator, F: Fn() -> I + Sync> ToSql
    for IterSql<T, I, F>
{
    to_sql_checked!();

    /// Converts between the array sql representation and sql while preserving the invariants
    /// required by callers.
    ///
    /// # Errors
    ///
    /// Returns an error if the iterator length cannot be represented or an element cannot be
    /// serialized.
    ///
    /// # Panics
    ///
    /// Panics if `ty` is not an array type.
    fn to_sql(
        &self,
        ty: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let member_type = match *ty.kind() {
            Kind::Array(ref member) => member,
            _ => panic!("expected array type"),
        };
        let iter = (self.0)();
        let dimension = ArrayDimension {
            len: downcast(iter.len())?,
            lower_bound: 1,
        };
        types::array_to_sql(
            Some(dimension),
            member_type.oid(),
            iter,
            |e, w| match e.to_sql(member_type, w)? {
                IsNull::No => Ok(postgres_protocol::IsNull::No),
                IsNull::Yes => Ok(postgres_protocol::IsNull::Yes),
            },
            w,
        )?;
        Ok(IsNull::No)
    }

    /// Reports whether the peer accepted this extension or message.
    fn accepts(ty: &Type) -> bool {
        match *ty.kind() {
            Kind::Array(ref member) => T::accepts(member),
            _ => false,
        }
    }
}
/// Handles the downcast behavior for this component.
fn downcast(len: usize) -> Result<i32, Box<dyn std::error::Error + Sync + Send>> {
    if len > i32::MAX as usize {
        Err("value too large to transmit".into())
    } else {
        Ok(len as i32)
    }
}
