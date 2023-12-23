# Trait Objects

In Rust, traits are types, but they are "unsized", which roughly means that they are only allowed to show up behind a pointer like `Box` (which points onto the heap) or `&` (which can point anywhere).

A type like `&ClickCallback` or `Box<dyn ClickCallback>` where `ClickCallback` is a Trait is called a "trait object", and includes a pointer to an instance of a type `T` implementing `ClickCallback`, and a vtable: a pointer to `T`'s implementation of each method in the trait.

```rust,editable
trait Draw {
    fn draw(&self);
}

struct Button;

impl Draw for Button {
    fn draw(&self) { println!("Button"); }
}

struct Text;

impl Draw for Text {
    fn draw(&self) { println!("Text"); }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,  // <-- trait object
}

impl Screen {

    fn new() -> Self {
        Screen { components: vec![Box::new(Button), Box::new(Text), Box::new(Text)] }
    }

    fn run(&self) {
        for component in self.components.iter() {
            // The purpose of trait objects is to permit "late binding" of methods.
            // Calling a method on a trait object results in virtual dispatch at runtime.
            // Here, `components` is a mix of `Button` and `Text` structs.
            component.draw();

        }
    }
}

fn main() {
    let s = Screen::new();
    s.run();
}
```

The set of traits after `dyn` is made up of an [object-safe]( https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety ) base trait plus any number of autotraits (one of `Send`, `Sync`, `Unpin`, `UnwindSafe`, and `RefUnwindSafe` - see [special traits]( https://doc.rust-lang.org/nightly/reference/special-types-and-traits.html )).

```rust,editable,ignore
dyn Trait
dyn Trait + Send
dyn Trait + Send + Sync
dyn Trait + 'static
```

## See also

[Trait Objects (docs)]( https://doc.rust-lang.org/book/ch17-02-trait-objects.html )
