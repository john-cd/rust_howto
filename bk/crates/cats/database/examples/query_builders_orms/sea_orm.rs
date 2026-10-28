#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Build `SeaORM` queries from an inline entity definition.
//!
//! This example creates a simple active model and prints the generated SQL
//! for an insert and select statement.

use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait as _;
use sea_orm::DbBackend;
use sea_orm::DbErr;
use sea_orm::EntityTrait as _;
use sea_orm::QueryFilter as _;
use sea_orm::QueryTrait as _;

/// The entity of the `posts` table.
mod post {
    use sea_orm::entity::prelude::*;

    /// A row of the `posts` table.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        /// The identifier of the post.
        #[sea_orm(primary_key)]
        pub id: i32,
        /// The title of the post.
        pub title: String,
        /// The content of the post.
        pub content: String,
    }

    /// The relations of the entity, which has none.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Builds insert and select queries and prints the SQL.
fn main() -> Result<(), DbErr> {
    let (insert_sql, select_sql) = build_queries();

    println!("INSERT SQL: {insert_sql}");
    println!("SELECT SQL: {select_sql}");
    Ok(())
}

/// Builds representative insert and filtered select SQL for the posts entity.
fn build_queries() -> (String, String) {
    let insert_sql = post::Entity::insert(post::ActiveModel {
        id: NotSet,
        title: Set("Hello SeaORM".to_owned()),
        content: Set("Building SQL without opening a database".to_owned()),
    })
    .build(DbBackend::Sqlite)
    .to_string();

    let select_sql = post::Entity::find()
        .filter(post::Column::Title.contains("SeaORM"))
        .build(DbBackend::Sqlite)
        .to_string();

    (insert_sql, select_sql)
}
// ANCHOR_END: example

pub fn run() -> Result<(), DbErr> {
    main()
}

/// Verifies the generated SQL without requiring a database connection.
#[cfg(test)]
mod tests {
    use super::*;

    /// Inserts the configured title and content into the posts table.
    #[test]
    fn insert_query_contains_post_fields_and_values() {
        let (insert_sql, _) = build_queries();

        assert!(insert_sql.starts_with("INSERT INTO \"posts\""));
        assert!(insert_sql.contains("\"title\""));
        assert!(insert_sql.contains("\"content\""));
        assert!(insert_sql.contains("'Hello SeaORM'"));
        assert!(
            insert_sql.contains("'Building SQL without opening a database'")
        );
    }

    /// Selects posts whose title contains the requested text.
    #[test]
    fn select_query_filters_by_title() {
        let (_, select_sql) = build_queries();

        assert!(select_sql.starts_with("SELECT "));
        assert!(select_sql.contains("FROM \"posts\""));
        assert!(select_sql.contains("LIKE '%SeaORM%'"));
    }
    // finish

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
