// use thiserror::Error;
// use thiserror::source;

// #[derive(Error, Debug)]
// pub struct MyError {
//     msg: String,
//     // The Error trait’s source() method is implemented to return whichever field has a #[source] attribute or is named source, if any.
//     // This is for identifying the underlying lower level error that caused your error. #[from] implies #[source].
//     // Any error type that implements `std::error::Error` or dereferences to `dyn std::error::Error` will work as a source.
//     #[source]
//     source: std::io::Error,
//     // Automatically detected to implement provide()
//     //backtrace: std::backtrace::Backtrace,
// }

// impl std::fmt::Display for MyError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{}", self.msg)
//     }
// }

fn main() {}
