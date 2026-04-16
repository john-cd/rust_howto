#import "../template.typ": *
= Chapter 1

#emoji.fire

```rust
fn main() {
      let user = ("Adrian", 38);
      println!("User {} is {} years old", user.0, user.1);

      // tuples within tuples
      let employee = (("Adrian", 38), "die Mobiliar");
      println!("User {} is {} years old and works for {}", employee.0.1, employee.0.1, employee.1);
}
```

#lorem(50)

#infoblock[Some information]

#smallcaps[smallcaps]

#upper[upper]
