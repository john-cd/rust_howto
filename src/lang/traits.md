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

// Implement Trait on a Type
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let na = NewsArticle { headline: "headline".to_string(), location: "location".to_string(), author: "author".to_string(), content:  "...".to_string()};
    println!("Summary: {}", na.summarize());
}
```

Trait methods are in scope only when their trait is.

## Default implementation

```rust,ignore
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

// String implements Display. That would not work otherwise.
impl OutlinePrint for String {}

fn main() { String::from("test").outline_print(); }
```

## Newtype pattern

Unlike interfaces in languages like Java, C# or Scala, new traits can be implemented for _existing_ types.

```rust,ignore
trait MyHash {
    fn hash(&self) -> u64;
}

impl MyHash for i64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the Newtype pattern:

```rust
use std::fmt;

struct Wrapper(Vec<String>); // tuple struct wrapping the type we want to add a non-local trait to.

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// If we wanted the new type to have every method the inner type has, implement the `Deref` trait.

fn main() { println!("{}", Wrapper(vec!["example".to_string(), "example 2".to_string()])); }
```

## Trait as parameter

```rust,ignore
// Accepts any type that implements the specified trait:
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (mostly equivalent):
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

## Multiple traits

```rust,ignore
// Note the `+`
fn notify(item: &(impl Summary + Display)) { }

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone, // note the `+`
    U: Clone + Debug,
{
}
```

## Return-position impl Trait

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let f = returns_closure();
    println!("{}", f(1));
}
```

## Generic traits

```rust,ignore
trait Test<T> {
    fn test(t: T);
}

impl<T, U> Test<T> for U { // note the <> in two places
    fn test(t: T) {}
}
```

## Associated types

```rust,ignore
pub trait Iterator {
    type Item;   // in Impl, use e.g. `Iterator<Item = u32>`

    fn next(&mut self) -> Option<Self::Item>;
}
```

```rust,ignore
trait Add<Rhs=Self> {  // default generic type
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

## Trait bounds

```rust,ignore
// Trait bounds: the `print_hash` function is generic over an unknown type `T`,
// but requires that `T` implements the `Hash` trait.
fn print_hash<T: Hash>(t: &T) {
    println!("The hash is {}", t.hash())
}

struct Pair<A, B> { first: A, second: B }

// Generics make it possible to implement a trait conditionally
// Here, the Pair type implements Hash if, and only if, its components do
impl<A: Hash, B: Hash> Hash for Pair<A, B> {
    fn hash(&self) -> u64 {
        self.first.hash() ^ self.second.hash()
    }
}
```

## Constants in traits

```rust,ignore
trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i32 = 99;
}
```

## Async and traits

See [Async](../topics/async.md)

## See also

[Traits (blog)]( https://blog.rust-lang.org/2015/05/11/traits.html )
