#![allow(dead_code)]

// ANCHOR: capnp
/// Compile a Captain Proto schema.
fn capnp() {
    println!("cargo::rerun-if-changed=examples/binary_encoders/foo.capnp");
    capnpc::CompilerCommand::new()
        //.src_prefix("schema") //  For all files specified for compilation that start with prefix, removes the prefix when computing output filenames.
        .file("examples/binary_encoders/foo.capnp") // Add a file to the list of files to be compiled.
        .run()
        .expect("schema compiler command"); // TODO
}
// ANCHOR_END: capnp

// ANCHOR: prost
/// Compile a ProtoBuf schema.
fn prost() {
    println!("cargo::rerun-if-changed=examples/binary_encoders/person.proto");

    let protoc = protoc_bin_vendored::protoc_bin_path()
        .expect("Failed to locate vendored protoc binary.");
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }

    prost_build::compile_protos(
        &["examples/binary_encoders/person.proto"],
        &["examples/binary_encoders/"],
    )
    .expect("Failed to compile Protocol Buffers. Ensure `protoc` is installed and on your PATH.");
}
// ANCHOR_END: prost

/// Entry point for the build script.
fn main() {
    #[cfg(target_os = "linux")]
    capnp();
    prost();
}
// [is prost fixed?](https://github.com/john-cd/rust_howto/issues/1417)
