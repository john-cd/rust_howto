// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generating a GraphQL server from a database with Seaography.
//!
//! Seaography is an automated GraphQL framework for SeaORM.
//!
//! This example demonstrates building an executable GraphQL schema from
//! SeaORM entities and executing GraphQL queries against an in-memory SQLite
//! database.
//!
//! 1. Generate SeaORM entities from your existing database:
//! ```sh
//! sea-orm-cli generate entity -o src/entities -u sqlite://sakila.db --seaography
//! ```
//!
//! 2. Generate the GraphQL server project:
//! ```sh
//! seaography-cli ./ src/entities sqlite://sakila.db seaography-sqlite-example
//! ```
//!
//! 3. Run the generated server:
//! ```sh
//! cd seaography-sqlite-example && cargo run
//! ```

use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::ConnectionTrait;
use sea_orm::Database;
use sea_orm::DbBackend;
use sea_orm::EntityTrait;
use sea_orm::Schema;
use seaography::Builder;
use seaography::BuilderContext;
use seaography::RelatedEntityFilter;

/// The `posts` SeaORM entity.
mod post {
    use sea_orm::entity::prelude::*;
    use seaography::BuilderContext;

    /// A blog post.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        /// The primary key.
        #[sea_orm(primary_key)]
        pub id: i32,
        /// The title of the post.
        pub title: String,
        /// The body of the post.
        pub content: String,
    }

    /// The relations of the entity; there are none.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl seaography::RelationBuilder for Relation {
        /// Returns no relation name because this enum has no variants.
        fn get_relation_name(&self, _: &'static BuilderContext) -> String {
            unreachable!("the posts entity defines no relations")
        }

        /// Returns no relation field because this enum has no variants.
        fn get_relation(
            &self,
            _: &'static BuilderContext,
        ) -> seaography::async_graphql::dynamic::Field {
            unreachable!("the posts entity defines no relations")
        }

        /// Returns no related-entity filter because this enum has no variants.
        fn get_related_entity_filter(
            &self,
            _: &'static BuilderContext,
        ) -> seaography::RelatedEntityFilterField {
            unreachable!("the posts entity defines no relations")
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

/// Generates a GraphQL schema for an in-memory SQLite table and queries it.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Connect to an in-memory SQLite database using SeaORM.
    let db = Database::connect("sqlite::memory:").await?;

    // 2. Create table schema from entity definition.
    let create_table =
        Schema::new(DbBackend::Sqlite).create_table_from_entity(post::Entity);
    db.execute(&create_table).await?;

    // 3. Insert sample data.
    post::Entity::insert(post::ActiveModel {
        id: NotSet,
        title: Set("Seaography Example".to_owned()),
        content: Set("GraphQL framework for SeaORM".to_owned()),
    })
    .exec(&db)
    .await?;

    // 4. Build GraphQL schema using Seaography.
    let builder_ctx = Box::leak(Box::new(BuilderContext::default()));
    let mut builder = Builder::new(builder_ctx, db.clone());
    let related_entity_filter = RelatedEntityFilter::<post::Entity>::build::<
        post::Relation,
    >(builder_ctx);
    builder.register_entity::<post::Entity>(vec![], &related_entity_filter);
    let schema = builder
        .register_related_entity_filter::<post::Entity>(related_entity_filter)
        .schema_builder()
        .data(db)
        .finish()?;

    // 5. Query the generated GraphQL schema.
    let query = "{
        posts {
            nodes {
                id
                title
                content
            }
        }
    }";
    let res = schema.execute(query).await;

    anyhow::ensure!(
        res.errors.is_empty(),
        "GraphQL query failed: {:?}",
        res.errors
    );
    let data = serde_json::to_value(res.data)?;
    anyhow::ensure!(
        data.pointer("/posts/nodes/0/title")
            .and_then(serde_json::Value::as_str)
            == Some("Seaography Example"),
        "GraphQL query did not return the inserted post: {data}"
    );

    println!("GraphQL response: {data}");

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_graphql_query_returns_inserted_post() -> anyhow::Result<()> {
        run()
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
