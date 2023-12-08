# Traits

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {          // implement Trait on a Type
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

## Default implementation

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {   // default implementation
        format!("(Read more from {}...)", self.summarize_author())  // can call a non-default method
    }
}
```

## Supertraits

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        println!("* {} *", self);  // can use println! because self is guaranteed to implement Display 
    }
}
```

## Newtype pattern

Unlike interfaces in languages like Java, C# or Scala, new traits can be implemented for existing types. One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the Newtype pattern:

```rust
use std::fmt;

struct Wrapper(Vec<String>); // tuple struct wrapping the type we want to add a non-local trait to.

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// If we wanted the new type to have every method the inner type has, implement the `Deref` trait. 
```


## Trait as parameter

```rust,ignore
pub fn notify(item: &impl Summary) {  // accepts any type that implements the specified trait. 
    println!("Breaking news! {}", item.summarize());
}

// or trait bound syntax:

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

## Multiple traits

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) { }

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone, // note the +
    U: Clone + Debug,
{ 
}
```

## Generic traits

```rust
trait Test<T> {
    fn test(t: T);
}

impl<T, U> Test<T> for U {
    fn test(t: T) {}
}
```

## Associated types

```rust
pub trait Iterator {
    type Item;   // im mpl, use e.g. type Item = u32;

    fn next(&mut self) -> Option<Self::Item>;
}
```

```rust
trait Add<Rhs=Self> {  // default generic type 
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```


## See also

[Traits]( https://blog.rust-lang.org/2015/05/11/traits.html )
