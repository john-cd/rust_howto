# Query builders and ORMs

{{#include query_builders_orms.incl.md}}

## Sqlx {#sqlx}

[![sqlx][c-sqlx-badge]][c-sqlx]{{hi:sqlx}} [![sqlx-github][c-sqlx-github-badge]][c-sqlx-github] [![sqlx-lib.rs][c-sqlx-lib.rs-badge]][c-sqlx-lib.rs] [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

[`sqlx`][c-sqlx]{{hi:sqlx}}⮳ is the Rust SQL Toolkit. An async, pure Rust SQL crate featuring compile-time checked queries without a DSL. Supports PostgreSQL{{hi:PostgreSQL}}, MySQL{{hi:MySQL}}, SQLite{{hi:SQLite}}, and MSSQL{{hi:MSSQL}}.

Works with Postgres, MySQL, SQLite, and MS SQL.
Supports compile time checking of queries. Async: supports both tokio and async-std.

## SeaORM {#sea-orm}

[![sea-orm][c-sea_orm-badge]][c-sea_orm]{{hi:sea-orm}} [![sea_orm-website][c-sea_orm-website-badge]][c-sea_orm-website] [![sea_orm-cookbook][c-sea_orm-cookbook-badge]][c-sea_orm-cookbook] [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

[Seaography GraphQL server][c-seaography-website]{{hi:seaography}}⮳

Built on top of sqlx (see above). There is also a related sea-query crate that provides a query builder without full ORM functionality.

## Diesel {#diesel}

[![diesel][c-diesel-badge]][c-diesel]{{hi:diesel}} [![diesel-lib.rs][c-diesel-lib.rs-badge]][c-diesel-lib.rs] [![cat-database][cat-database-badge]][cat-database]{{hi:Databases}}

Has excellent performance and takes an approach of strict compile time guarantees. The main crate is Sync only, but diesel-async provides an async connection implementation.

## Toasty {#toasty}

[![toasty][c-toasty-badge]][c-toasty]{{hi:toasty}}
[![toasty-crates.io][c-toasty-crates.io-badge]][c-toasty-crates.io]
[![toasty-github][c-toasty-github-badge]][c-toasty-github]
[![toasty-lib.rs][c-toasty-lib.rs-badge]][c-toasty-lib.rs]

Toasty is an ORM for the Rust programming language that prioritizes ease-of-use. It supports both SQL datases as well as some NoSQL databases, including DynamoDB and Cassandra. Note that Toasty does not hide the database capabilities. Instead, Toasty exposes features based on the target database.

It is currently in active development and not yet published to crates.io. You can try using it directly from Github.

Using the example in the [Toasty announcement blog][c-toasty-blog], projects that use Toasty start by creating a schema file to define the application's data model.

```text
model User {
  #[key]
  #[auto]
  id: Id,

  name: String,

  #[unique]
  email: String,

  todos: [Todo],

  moto: Option<String>,
}

model Todo {
  #[key]
  #[auto]
  id: Id,

  #[index]
  user_id: Id<User>,

  #[relation(key = user_id, references = id)]
  user: User,

  title: String,
}
```

Use the Toasty CLI tool to generate all necessary Rust code for working with this data model.

```rust,editable,ignore
// Create a new user and give them some todos.
User::create()
  .name("John Doe")
  .email("john@example.com")
  .todo(Todo::create().title("Make pizza"))
  .todo(Todo::create().title("Finish Toasty"))
  .todo(Todo::create().title("Sleep"))
  .exec(&db)
  .await?;

// Load the user from the database
let user = User::find_by_email("john@example.com").get(&db).await?

// Load and iterate the user's todos
let mut todos = user.todos().all(&db).await.unwrap();

while let Some(todo) = todos.next().await {
  let todo = todo.unwrap();
  println!("{:#?}", todo);
}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
