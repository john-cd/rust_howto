#![allow(dead_code)]

// ANCHOR: capnp
/// Compile a Captain Proto schema.
fn capnp() {
    capnpc::CompilerCommand::new()
        //.src_prefix("schema") //  For all files specified for compilation that start with prefix, removes the prefix when computing output filenames.
        .file("examples/binary_encoders/foo.capnp") // Add a file to the list of files to be compiled.
        .run()
        .expect("schema compiler command");
}
// ANCHOR_END: capnp

// ANCHOR: prost
/// Compile a ProtoBuf schema.
fn prost() {
    prost_build::compile_protos(
        &["examples/binary_encoders/person.proto"],
        &["src/"],
    )
    .unwrap();
}
// ANCHOR_END: prost

fn main() {
    #[cfg(target_os = "linux")]
    capnp();
    // prost();
}
// TODO fix prost
