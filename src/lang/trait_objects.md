# Trait Objects

In Rust, traits are types, but they are "unsized", which roughly means that they are only allowed to show up behind a pointer like `Box` (which points onto the heap) or `&` (which can point anywhere).

A type like `&ClickCallback` or `Box<dyn ClickCallback>` where `ClickCallback` is a Trait is called a "trait object", and includes a pointer to an instance of a type T implementing ClickCallback, and a vtable: a pointer to T's implementation of each method in the trait

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,  // trait object
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {}
```


## See also

[Trait Objects]( https://doc.rust-lang.org/book/ch17-02-trait-objects.html )
