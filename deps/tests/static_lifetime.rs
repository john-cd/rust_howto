// 'static indicates that the data pointed to by the reference lives for the _remaining_ lifetime of the running program.
// It can still be coerced to a shorter lifetime.
fn my_string() -> &'static str {
  let s: &'static str = "I have a static lifetime.";
  s
}

#[test]
fn test() {
  println!("{}", my_string());
}
