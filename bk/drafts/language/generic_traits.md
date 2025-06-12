
Generic traits allow you to define traits that can work with different types:

```rust,editable
{{#include ../../crates/language/tests/traits/generic_traits.rs:example}}
```

Trait bounds are often used with generics to specify that a generic type must implement a particular trait:

```rust,editable
{{#include ../../crates/language/tests/traits/trait_bounds.rs:example}}
```

## Define Traits that Work with Multiple Types (Generic Traits) {#write-generic-traits}

Traits can be generic, meaning they can have type parameters.

Generic traits allow you to define a shared set of behaviors that can be implemented by multiple, unknown types, enabling code reuse and polymorphism.

```rust,editable
{{#include ../../crates/language/tests/traits/generic_traits.rs:example}}
```

## Trait Bounds {#trait-bounds}

Generic items may use traits as bounds on their type parameters.

```rust,editable
{{#include ../../crates/language/tests/traits/trait_bounds.rs:example}}
```

Traits can be used as bounds to generic parameters of a functions, allowing the function to accept any type that implements the specified trait.

```rust,editable
{{#include ../../crates/language/tests/traits/traits_as_parameters.rs:example}}
```

### Implement Trait Bounds for Multiple Traits {#multiple-traits}

Trait bounds with multiple traits TODO

```rust,editable
{{#include ../../crates/language/tests/traits/multiple_traits.rs:example}}
```
