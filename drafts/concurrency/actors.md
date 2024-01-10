# Actors

## Stakker

## Riker

[Riker][riker]

```rust,editable,ignore
struct MyActor;

impl Actor for MyActor {
    type Msg = String;

    fn recv(&mut self,
            ctx: &Context<String>,
            msg: String,
            sender: Sender) {

        println!("received {}", msg);
    }
}


fn main() {
    let sys = SystemBuilder::new()
                            .name("my-app")
                            .create()
                            .unwrap();
    // Every actor has a name that is required to be unique among its singlings (those actors sharing the same parent actor).
    let my_actor = sys.actor_of::<MyActor>("my-actor").unwrap();
    my_actor.tell("Hello!".to_string(), None);

    // force main to wait before exiting program
    std::thread::sleep(Duration::from_millis(500));
}
```

## Alternatives

[Actix][Actix]

[Ractor][Ractor]

## Utilities

[Await tree][Await tree]

## Reference

[Actors with Tokio][Actors with Tokio]

[Actix]: https://github.com/actix/actix
[Actors with Tokio]: https://ryhl.io/blog/actors-with-tokio/
[Await tree]: https://crates.io/crates/await-tree
[Ractor]: https://crates.io/crates/ractor
[Riker]: https://riker.rs/

{{#include ../refs/link-refs.md}}
