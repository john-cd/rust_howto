# Query Builders and ORMs

{{#include query_builders_orms.incl.md}}

## `sqlx` {#sqlx}

[![sqlx][c~sqlx~docs~badge]][c~sqlx~docs] [![sqlx~crates.io][c~sqlx~crates.io~badge]][c~sqlx~crates.io] [![sqlx~github][c~sqlx~github~badge]][c~sqlx~github] [![sqlx~lib.rs][c~sqlx~lib.rs~badge]][c~sqlx~lib.rs]{{hi:sqlx}} [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

[`sqlx`][c~sqlx~docs]⮳ is a low-level, [asynchronous][p~asynchronous] SQL library for Rust. It supports various [databases][p~databases] like PostgreSQL{{hi:PostgreSQL}}, MySQL{{hi:MySQL}}, SQLite{{hi:SQLite}}, and MSSQL{{hi:MSSQL}}, and both [`tokio`][c~tokio~docs]⮳{{hi:tokio}} and [`async-std`][c~async_std~docs]⮳{{hi:async-std}} async runtimes. It features compile-time checked queries without a DSL. SQLx is not an ORM.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/query_builders_orms/sqlx.rs:example}}
```

## SeaORM {#sea-orm}

[![sea-orm][c~sea_orm~docs~badge]][c~sea_orm~docs]{{hi:sea-orm}} [![sea_orm~website][c~sea_orm~website~badge]][c~sea_orm~website] [![sea_orm~cookbook][c~sea_orm~cookbook~badge]][c~sea_orm~cookbook] [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

[Seaography GraphQL server][c~seaography~website]{{hi:seaography}}⮳.

Built on top of [`sqlx`][c~sqlx~docs]⮳{{hi:sqlx}} (see above). There is also a related sea-query crate that provides a query builder without full ORM functionality.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/query_builders_orms/sea_orm.rs:example}}
```

## `diesel` {#diesel}

[![diesel][c~diesel~docs~badge]][c~diesel~docs]{{hi:diesel}} [![diesel~lib.rs][c~diesel~lib.rs~badge]][c~diesel~lib.rs] [![cat~database][cat~database~badge]][cat~database]{{hi:Databases}}

The [`diesel`][c~diesel~docs]⮳{{hi:diesel}} crate is a powerful ORM (object-relational mapper) and query builder for Rust. It allows you to interact with databases in a type-safe and efficient manner. ORMs help object-oriented programmers abstract the details of relational databases, and do not require writing raw SQL queries.

[`diesel`][c~diesel~docs]⮳{{hi:diesel}} supports PostgreSQL, MySQL, and SQLite. [`diesel`][c~diesel~docs]⮳{{hi:diesel}} has excellent performance and takes an approach of strict compile time guarantees. The main crate is synchronous only, but [`diesel-async`][c~diesel-async~docs]⮳{{hi:diesel-async}} provides an async connection implementation.

To create a new [`diesel`][c~diesel~docs]⮳{{hi:diesel}} project targeting [`sqlite`][c~sqlite~docs]⮳{{hi:sqlite}}, follow these steps:

- Add the necessary dependencies to your Cargo.toml file (update the versions as needed):

```toml
diesel = { version = "2.2.6", features = ["sqlite"] }
diesel_migrations = { version = "2.2.0", features = ["sqlite"] }
dotenvy = "0.15.7"
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.138"
```

- Run the following commands in your terminal to create a `.env` file with the [database][p~database] [URL][p~url]:
set up your schema.rs file and migrations.

```bash
echo DATABASE_URL=sqlite::memory: > .env
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
diesel migration generate create_users
```

`DATABASE_URL` can be set to a file path, or `sqlite::memory:` for an in-memory database. For other [databases][p~databases], the URL format is `protocol://user:password@host/database`. For MySQL, the URL would be as follows:

```bash
echo DATABASE_URL=mysql://<username>:<password>@localhost/<database> >> .env
```

- Add the following SQL to the generated `up.sql` file:

```sql
{{#include ../../../crates/cats/database/examples/query_builders_orms/migrations/2024-12-29-173417_create_users/up.sql}}
```

- Leave the `down.sql` file empty for simplicity.

```sql
{{#include ../../../crates/cats/database/examples/query_builders_orms/migrations/2024-12-29-173417_create_users/down.sql}}
```

- Create a `schema.rs` file by running:

```bash
diesel print-schema > src/schema.rs
```

- Write the Rust code to interact with the [database][p~database]:

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/query_builders_orms/diesel1.rs:example}}
```

## `toasty` {#toasty}

[![toasty][c~toasty~docs~badge]][c~toasty~docs]{{hi:toasty}}
[![toasty~crates.io][c~toasty~crates.io~badge]][c~toasty~crates.io]
[![toasty~github][c~toasty~github~badge]][c~toasty~github]
[![toasty~lib.rs][c~toasty~lib.rs~badge]][c~toasty~lib.rs]

[`toasty`][c~toasty~docs]⮳{{hi:toasty}} is an ORM for the Rust programming language that prioritizes ease-of-use. It supports both SQL databases as well as some NoSQL databases, including DynamoDB and Cassandra. Note that Toasty does not hide the database capabilities. Instead, Toasty exposes features based on the target database.

It is currently in active development and not yet published to crates.io. You can try using it directly from Github.

Using the example in the [Toasty announcement blog][c~toasty~blog], projects that use Toasty start by creating a schema file to define the application's data model.

```text
model User {
  #[key]
  #[auto]
  id: Id,

  name: String,

  #[unique]
  email: String,

  todos: [Todo],

  moto: [Option][p~option]<String>,
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

```rust,editable,ignore,noplayground
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
  println!("{todo:#?}");
}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; move toasty example to a file](https://github.com/john-cd/rust_howto/issues/912)
</div>
