# Variables and Constants

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  // set only to a constant expression; type must be annotated
let apples = 5;                 // immutable
let mut guess = String::new();  // mutable
```

Shadowing:

```rust
let x = 5;
let x = x + 1;  // redefines x; type can change
```

Destructuring:

```rust
// destructuring tuples
 let (x, y, z) = (1, 2, 3);

// destructuring structs
let p = Point { x: 0, y: 7 };
let Point { x: a, y: b } = p;   // a = 0, b = 7
let Point { x, y } = p;         // simpler
```

Starting the name of a variable with an underscore silences unused variable warnings.
