// // ANCHOR: example
// //COMING SOON
// // ANCHOR_END: example

// use sea_orm::prelude::*;
// use sea_orm::{Database, EntityTrait, ModelTrait, Set};
// use tokio::runtime::Runtime;

// mod entity;

// use entity::prelude::*;

// #[tokio::main]
// async fn main() -> Result<(), DbErr> {
//     // Connect to the database
//     let db = Database::connect("sqlite://example.db").await?;

//     // Create a new record
//     let new_post = post::ActiveModel {
//         title: Set("Hello World".to_owned()),
//         content: Set(Some("This is a new post".to_owned())),
//         ..Default::default()
//     };

//     let inserted_post = Post::insert(new_post).exec(&db).await?;

//     println!("Inserted post: {:?}", inserted_post);

//     // Read records
//     let posts: Vec<post::Model> = Post::find().all(&db).await?;
//     println!("All posts: {:?}", posts);

//     // Update a record
//     let mut post_to_update: post::ActiveModel =
// Post::find_by_id(1).one(&db).await?.unwrap().into();     post_to_update.
// content = Set(Some("Updated content".to_owned()));

//     let updated_post = post_to_update.update(&db).await?;
//     println!("Updated post: {:?}", updated_post);

//     // Delete a record
//     let result = Post::delete_by_id(1).exec(&db).await?;
//     println!("Deleted post: {:?}", result);

//     Ok(())
// }

// #[test]
// fn require_external_svc() {
//     main();
// }
// // [WIP finish SOON](https://github.com/john-cd/rust_howto/issues/715)
