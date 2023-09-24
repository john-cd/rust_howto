# Slices


```rust
    let s = String::from("hello world");

    let hello : &str = &s[0..5];  // or &s[..5];
    let world = &s[6..11]; // or &s[6..];
```
