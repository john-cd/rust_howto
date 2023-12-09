# Error Handling

## Irrecoverable panics

```rust,ignore
panic!("crash and burn"); 
```


## Recoverable errors with `Result`

```rust,ignore
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess) // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value.
    .expect("Failed to read line"); // If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. 

let greeting_file = File::open("hello.txt").unwrap(); // alternative: panics if error - no message
```

**unwrap_or_else**

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```


## A Shortcut for Propagating Errors: the ? Operator

```rust,ignore
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {}
```

If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

This error points out that we’re only allowed to use the `?` operator in a function that returns Result, Option, or another type that implements `FromResidual`.

Another example:

```rust
use std::error::Error;

fn parse_port(s: &str) -> Result<u16, Box<dyn Error>> {  // needed to use Box<dyn Error>, because the returned error type cannot be determined during compile time: It will either contain an instance of std::num::ParseIntError (from the parse method, when parsing fails), or a string (when the port is zero).
    let port: u16 = s.parse()?;
    if port == 0 {
        Err(Box::from(format!("invalid: {}", port)))
    } else {
        Ok(port)
    }
}

fn main() {
    match parse_port("123") {
        Ok(port) => println!("port: {}", port),
        Err(err) => panic!("{}", err),
    }
}
```

`std::io` defines the type alias `type Result<T> = std::result::Result<T, std::io::Error>;`
