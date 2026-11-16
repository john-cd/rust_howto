mod vulkan_instance_version;

fn main() -> impl std::process::Termination {
    vulkan_instance_version::run()
}
