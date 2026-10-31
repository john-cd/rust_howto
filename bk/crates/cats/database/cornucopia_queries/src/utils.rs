// This file was generated with `cornucopia`. Do not modify.

use postgres_types::Kind;
use postgres_types::ToSql;
use postgres_types::Type;
pub fn escape_domain(ty: &Type) -> &Type {
    match ty.kind() {
        Kind::Domain(ty) => ty,
        _ => ty,
    }
}
pub fn slice_iter<'a>(
    s: &'a [&'a (dyn ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
    s.iter().map(|s| *s as _)
}
