#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Generating a GraphQL server from a database with Seaography.
//!
//! Seaography is an automated GraphQL framework for SeaORM.
//!
//! This example demonstrates building an executable GraphQL schema from
//! SeaORM entities and executing GraphQL queries against an in-memory SQLite database.
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

use sea_orm::{
    ActiveValue::NotSet, ActiveValue::Set, ConnectionTrait, Database, DbBackend, EntityTrait,
    Schema,
};
use seaography::{Builder, BuilderContext};

/// The `posts` SeaORM entity.
mod post {
    use sea_orm::entity::prelude::*;

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

    impl ActiveModelBehavior for ActiveModel {}
}

/// Generates a GraphQL schema for an in-memory SQLite table and queries it.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Connect to an in-memory SQLite database using SeaORM.
    let db = Database::connect("sqlite::memory:").await?;

    // 2. Create table schema from entity definition.
    let stmt = db
        .get_database_backend()
        .build(&Schema::new(DbBackend::Sqlite).create_table_from_entity(post::Entity));
    db.execute(stmt).await?;

    // 3. Insert sample data.
    post::Entity::insert(post::ActiveModel {
        id: NotSet,
        title: Set("Seaography Example".to_string()),
        content: Set("GraphQL framework for SeaORM".to_string()),
    })
    .exec(&db)
    .await?;

    // 4. Build GraphQL schema using Seaography.
    let builder_ctx = Box::leak(Box::new(BuilderContext::default()));
    let mut builder = Builder::new(builder_ctx, db.clone());
    builder.register_entity::<post::Entity>(vec![]);
    let schema = builder.schema_builder().data(db).finish()?;

    // 5. Query the generated GraphQL schema.
    let query = r#"{
        post {
            nodes {
                id
                title
                content
            }
        }
    }"#;
    let res = schema.execute(query).await;

    println!("GraphQL response: {:?}", res.data);

    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn require_external_svc() {
        run().unwrap();
    }
}
