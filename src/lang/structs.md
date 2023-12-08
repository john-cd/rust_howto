# Structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Struct fields follow the general rule of everything being private by default unless annotated with `pub`.

```rust,ignore
// create an instance 
let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,       // instead of username: username - field init shorthand
        email,
        sign_in_count: 1,
    }
}

// struct update
let user2 = User {
    email: String::from("another@example.com"),
    ..user1    // the remaining fields not explicitly set should have the same value as the fields in the given instance.
};
```

```rust
// Tuple struct
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

```rust
// Unit-like struct
struct AlwaysEqual;  // no data
```

```rust
// Methods
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {                    // implementation block (multiple allowed for a given struct)
    fn area(&self) -> u32 {         // short for self: &Self, an alias for the type that the impl block is for
        self.width * self.height
    }
}

// Associated Functions - NO &self
// often use for constructors: SomeType::new(...)
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
