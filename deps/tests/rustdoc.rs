/// This is a doc comment
/// Note the three slashes
/// The first line is equivalent to the next line.
/// This is a doc comment
fn documented_function() {
    println!("Function with doc comment.");
}

// Alternatively, you may use an external file

#[doc = include_str!("../README.md")]
fn function_including_external_file_as_documentation() {}

#[test]
fn test() {
    documented_function();
    function_including_external_file_as_documentation();
}
