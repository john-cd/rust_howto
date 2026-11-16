mod wgsl_to_spirv;

fn main() -> impl std::process::Termination {
    wgsl_to_spirv::run()
}
