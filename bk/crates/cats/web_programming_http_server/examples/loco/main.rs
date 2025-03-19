// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
use anyhow::Result;
// use loco_rs::app::AppContext;
// use loco_rs::app::Hooks;
// use loco_rs::boot::BootResult;
// use loco_rs::boot::StartMode;
// use loco_rs::boot::create_app;
// use loco_rs::controller::AppRoutes;
// use loco_rs::db::DataPool;
// use loco_rs::model::Model;
// use loco_rs::model::NewModel;
// use loco_rs::task::Tasks;
// use serde::Deserialize;
// use serde::Serialize;

// // Uses #[derive(Model)] and #[derive(NewModel)] to define the User model and
// // its creation counterpart NewUser.

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Model)]
// #[table = "users"] // The #[table = "users"] attribute specifies the database
// table name. pub struct User {
//     #[primary_key]
//     pub id: i64,
//     pub name: String,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, NewModel)]
// pub struct NewUser {
//     pub name: String,
// }

// // Define routes for fetching all users (GET) and creating a new user (POST).
// async fn routes(ctx: &AppContext, routes: &mut AppRoutes) -> Result<()> {
//     routes.add("/", get_users);
//     routes.add("/users", create_user).post();
//     Ok(())
// }

// // Use User::all(&ctx.db).await? to fetch all users and User::create(&ctx.db,
// // new_user).await? to create a new user.

// async fn get_users(ctx: &AppContext) -> Result<String> {
//     let users = User::all(&ctx.db).await?;
//     let json = serde_json::to_string(&users)?;
//     Ok(json)
// }

// async fn create_user(ctx: &AppContext, new_user: NewUser) -> Result<String> {
//     let user = User::create(&ctx.db, new_user).await?;
//     let json = serde_json::to_string(&user)?; // Serialize the user data to
// JSON for the responses.     Ok(json)
// }

// async fn boot_app(mode: StartMode) -> Result<BootResult> {
//     create_app(mode).await
// }

#[tokio::main]
async fn main() -> Result<()> {
    // // Use create_app, boot.start, and boot_app for proper application
    // // bootstrapping and startup.
    // let boot = boot_app(StartMode::Server).await?;
    // let routes = routes(&boot.app_context, &mut boot.router).await?;

    // boot.start(routes).await?;

    Ok(())
}

// pub async fn app_hooks(_ctx: &AppContext, _hooks: &mut Hooks) -> Result<()> {
//     Ok(())
// }

// pub async fn tasks(_ctx: &AppContext, _tasks: &mut Tasks) -> Result<()> {
//     Ok(())
// }

// // [WIP finish](https://github.com/john-cd/rust_howto/issues/868)
