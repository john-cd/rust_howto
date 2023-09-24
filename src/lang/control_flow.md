# Control Flow

**If else** 

```rust
    let result = if number < 5 {         // condition must return a bool; `if` is an expression
        println!("condition was true");
        5
    } else {
        println!("condition was false");
        6
    }
```

Also `else if <cond> { ... }`

**Loop**

```rust
let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // also `continue` and Loop labels https://doc.rust-lang.org/book/ch03-05-control-flow.html 
        }
    };
```    

**While** 

```rust
while number != 0 {
        println!("{number}!");
        number -= 1;
    }
```   

**For** 

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}

// range - generates all numbers in sequence starting from one number and ending before another number.
for number in (1..4).rev() {  // reverse enumeration
    println!("{number}!");
}
```