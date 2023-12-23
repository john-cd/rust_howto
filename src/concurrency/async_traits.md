# Async traits

As of Rust 1.75, it is possible to have `async` functions in traits:

```rust,editable,ignore
trait HealthCheck {
    async fn check(&mut self) -> bool;   // <- async fn defined in a Trait
}

impl HealthCheck for MyHealthChecker {
    async fn check(&mut self) -> bool {  // async fn implementation in the associated impl block
        do_async_op().await
    }
}

async fn do_health_check(hc: impl HealthCheck) {
    if !hc.check().await {               // use as normal
        log_health_check_failure().await;
    }
}
```

[Stabilizing async fn in traits in 2023]( https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html )

This is in turn enabled by return-position `impl Trait` in traits, since `async fn` is sugar for functions that return `-> impl Future`.

```rust,editable,ignore
trait Container {
    fn items(&self) -> impl Iterator<Item = Widget>;
}

impl Container for MyContainer {
    fn items(&self) -> impl Iterator<Item = Widget> {
        self.items.iter().cloned()
    }
}
```

Note that there are still caveats for public traits - see [Announcing `async fn` and return-position `impl Trait` in traits]( https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html# ).

In addition, traits that use `-> impl Trait` and `async fn` are not object-safe, which means they lack support for dynamic dispatch. In the meanwhile, use the [Async trait crate]( https://github.com/dtolnay/async-trait ).

```rust,editable,ignore
use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4u16 {
            remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }
}
```
