// ANCHOR: example
use thiserror::Error;

#[derive(Error, Debug)]
pub struct MyError {
    msg: String,
    // The Error trait’s source() method is implemented to return whichever
    // field has a #[source] attribute or is named source, if any. This is
    // for identifying the underlying lower level error that caused your
    // error. #[from] implies #[source]. Any error type that implements
    // `std::error::Error` or dereferences to `dyn std::error::Error` will work
    // as a source.
    #[source]
    source: std::io::Error,
    // Automatically detected to implement provide()
    // backtrace: std::backtrace::Backtrace,
}

impl std::fmt::Display for MyError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
    Err(Box::new(MyError {
        msg: "Error message".to_string(),
        source: io_error,
    }))
}

fn main() {
    match example() {
        Ok(_) => {
            println!("Got OK");
        }
        Err(err) => {
            println!("Got {}", err);
        }
    }
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
