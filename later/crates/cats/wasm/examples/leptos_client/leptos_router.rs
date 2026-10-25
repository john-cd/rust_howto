#![allow(dead_code)]
#![allow(clippy::same_name_method)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Nested routes, a route parameter and a not-found route with
//! [`leptos_router`].

use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::components::ParentRoute;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use leptos_router::hooks::use_params_map;
use leptos_router::path;

/// Shared layout: a header, then the matched child route.
#[component]
fn Layout() -> impl IntoView {
    view! {
        <header>"My dashboard"</header>
        <main><Outlet/></main>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <h1>"Home"</h1> }
}

#[component]
fn UserList() -> impl IntoView {
    view! { <h1>"Users"</h1> }
}

/// Reads the `:id` segment of the URL.
#[component]
fn UserDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();
    view! { <h1>"User " {id}</h1> }
}

#[component]
fn NotFound() -> impl IntoView {
    view! { <h1>"Not found"</h1> }
}

/// The application: routes are declared with the `path!` macro.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                <ParentRoute path=path!("") view=Layout>
                    <Route path=path!("") view=Home/>
                    <Route path=path!("users") view=UserList/>
                    <Route path=path!("users/:id") view=UserDetail/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use leptos_router::location::RequestUrl;

    use super::*;

    /// Renders the app to HTML as a server would for `url`; no browser is
    /// needed.
    fn render(url: &str) -> String {
        let _ = any_spawner::Executor::init_tokio();
        let owner = Owner::new();
        owner.set();
        provide_context(RequestUrl::new(url));
        App().into_view().to_html()
    }

    #[tokio::test]
    async fn routes_render() {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                assert!(render("/").contains("Home"));
                assert!(render("/users").contains("Users"));
                let detail = render("/users/42");
                assert!(detail.contains("User"));
                assert!(detail.contains("42"));
                assert!(detail.contains("My dashboard"));
                assert!(render("/nope").contains("Not found"));
            })
            .await;
    }
}
