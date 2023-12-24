# Traits

```rust,editable
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

```rust,editable,ignore
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {   // default implementation
        format!("(Read more from {}...)", self.summarize_author())  // can call a non-default method
    }
}
```

## Supertraits

```rust,editable
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

```rust,editable
trait MyHash {
    fn myhash(&self) -> u64;
}

impl MyHash for i64 {
    fn myhash(&self) -> u64 {
        *self as u64
    }
}

fn main() {
    let x = 1i64;
    println!("{}", x.myhash());
}
```

One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate. If neither are, use the Newtype pattern:

```rust,editable
use std::fmt;

struct Wrapper(Vec<String>); // tuple struct wrapping the type we want to add a non-local trait to.

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// If we wanted the new type to have every method the inner type has, implement the `Deref` trait.

fn main() {
    println!("{}", Wrapper(vec!["example".to_string(), "example 2".to_string()]));
}
```

## Trait as parameter

```rust,editable
// Accepts any type that implements the specified trait:
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (mostly equivalent):
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    txt: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        self.txt.clone()
    }
}

fn main() {
    let a = Article { txt: String::from("some text") };
    notify(&a);
    notify2(&a);
}
```

## Multiple traits

```rust,editable
#![allow(dead_code)]
use std::fmt::Debug;
use std::clone::Clone;

// Note the `+`
fn a_function(item: &(impl Debug + Clone)) {
    println!("{:?}", item.clone());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Debug + Clone, // note the `+`
    U: Debug + Clone,
{
    42
}

#[derive(Debug, Clone)]
struct S;

fn main() {
    let s = S;
    a_function(&s);
}
```

## Return-position impl Trait

```rust,editable
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let f = returns_closure();
    println!("{}", f(1));
}
```

## Generic traits

```rust,editable
trait Test<T> {
    fn test(_t: T);
}

struct SomeStruct;

impl<T> Test<T> for SomeStruct { // note the <> in two places
    fn test(_t: T) {
        println!("test");
    }
}

fn main() {
    SomeStruct::test(1);
    SomeStruct::test(true);
}
```

## Associated types

```rust,editable,ignore
trait Iterator {
    type Item;   // in Impl, use e.g. `Iterator<Item = u32>`

    fn next(&mut self) -> Option<Self::Item>;
}
```

```rust,editable,ignore
trait Add<Rhs=Self> {  // default generic type
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

## Trait bounds

```rust,editable
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Trait bounds: the `print_hash` function is generic over an unknown type `T`,
// but requires that `T` implements the `Hash` trait.
fn print_hash<T: Hash>(t: &T) {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    println!("The hash is {:x}", hasher.finish());
}

struct Pair<A, B> { first: A, second: B }

// Generics make it possible to implement a trait conditionally
// Here, the Pair type implements Hash if, and only if, its components do
impl<A: Hash, B: Hash> Hash for Pair<A, B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first.hash(state);
        self.second.hash(state);
    }
}

fn main() {
    let p = Pair { first: 1, second: "2" };
    print_hash(&p);
}
```

## Constants in traits

```rust,editable
trait Example {
    const CONST_NO_DEFAULT: i32;
    const CONST_WITH_DEFAULT: i32 = 99;
}

struct S;

impl Example for S {
    const CONST_NO_DEFAULT: i32 = 0;
}

fn main() {
    println!("{} {}", S::CONST_NO_DEFAULT, S::CONST_WITH_DEFAULT);
}
```

## Async and traits

See [Async](../concurrency/async.md)

## See also

[Traits (blog)]( https://blog.rust-lang.org/2015/05/11/traits.html )
