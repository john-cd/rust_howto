#[test]
#[ignore]
fn test() {
    cc::Build::new().file("src/hello.c").compile("hello");
    // outputs `libhello.a`
}
