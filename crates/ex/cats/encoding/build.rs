// ANCHOR: example
fn main() {
    capnpc::CompilerCommand::new()
        //.src_prefix("schema") //  For all files specified for compilation that start with prefix, removes the prefix when computing output filenames.
        .file("tests/proto/foo.capnp") // Add a file to the list of files to be compiled.
        .run()
        .expect("schema compiler command");
}
// ANCHOR_END: example
