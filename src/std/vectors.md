# Vectors

Vectors can only store values that are the same type. 

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);

    let mut v = vec![1, 2, 3];  // or vec!(1, 2, 3)


    let third: &i32 = &v[2];    // read

    let third: Option<&i32> = v.get(2);

    for i in &v {
        println!("{i}"); 
    }

    for i in &mut v {
        *i += 50;     // dereference operator
    }
}
```
