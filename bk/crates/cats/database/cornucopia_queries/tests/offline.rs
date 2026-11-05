//! Offline tests for the checked-in, generated `cornucopia` output.
//!
//! No PostgreSQL server is needed: they exercise the (de)serialization helpers
//! and the query constructors only.
//!
//! TODO Reviewed ??-??-2026.

use database_cornucopia_queries::ArrayIterator;
use database_cornucopia_queries::ArraySql;
use database_cornucopia_queries::Domain;
use database_cornucopia_queries::IterSql;
use database_cornucopia_queries::StringSql;
use database_cornucopia_queries::queries::greetings;
use postgres_types::FromSql;
use postgres_types::IsNull;
use postgres_types::ToSql;
use postgres_types::Type;
use postgres_types::private::BytesMut;

fn is_string_sql<T: StringSql>(_: &T) {}

#[test]
fn string_sql_is_implemented_for_common_string_types() {
    is_string_sql(&String::new());
    is_string_sql(&"str");
    is_string_sql(&std::borrow::Cow::Borrowed("cow"));
    is_string_sql(&Box::<str>::from("boxed"));
}

#[test]
fn vec_serializes_as_text_array_and_iterator_reads_it_back()
-> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let values = vec!["hello", "world"];
    let mut buf = BytesMut::new();
    let is_null = values.escape_domain_to_sql(&Type::TEXT_ARRAY, &mut buf)?;
    assert!(matches!(is_null, IsNull::No));

    let iter = ArrayIterator::<&str>::from_sql(&Type::TEXT_ARRAY, &buf)?;
    assert_eq!(iter.collect::<Vec<_>>(), ["hello", "world"]);
    Ok(())
}

#[test]
fn iter_sql_round_trips_through_postgres_wire_format()
-> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let numbers = [1_i32, 2, 3];
    let wrapped = IterSql(|| numbers.iter().copied());
    let mut buf = BytesMut::new();
    wrapped.to_sql(&Type::INT4_ARRAY, &mut buf)?;

    let decoded: Vec<i32> = ArrayIterator::<i32>::from_sql(&Type::INT4_ARRAY, &buf)?.collect();
    assert_eq!(decoded, numbers);
    Ok(())
}

#[test]
fn array_iterator_accepts_only_matching_array_types() {
    assert!(<ArrayIterator<'_, i32> as FromSql>::accepts(
        &Type::INT4_ARRAY
    ));
    assert!(!<ArrayIterator<'_, i32> as FromSql>::accepts(
        &Type::TEXT_ARRAY
    ));
    assert!(!<ArrayIterator<'_, i32> as FromSql>::accepts(&Type::INT4));
}

#[test]
fn domain_wrapper_delegates_to_inner_type() -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    assert!(<Domain<String> as ToSql>::accepts(&Type::TEXT));
    assert!(!<Domain<String> as ToSql>::accepts(&Type::INT4));

    let mut buf = BytesMut::new();
    Domain("abc").to_sql(&Type::TEXT, &mut buf)?;
    assert_eq!(&*buf, b"abc");
    assert_eq!(format!("{:?}", Domain(1_i32)), "DomainWrapper(1)");
    Ok(())
}

#[test]
fn greeting_statements_are_constructible_without_a_connection() {
    // Constructors only build the (unprepared) statement; nothing is sent.
    let _add = greetings::add_greeting();
    let _list = greetings::list_greetings();
}
