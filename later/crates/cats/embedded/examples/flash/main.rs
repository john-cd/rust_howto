mod nor_flash_storage;

fn main() -> impl std::process::Termination {
    nor_flash_storage::run()
}
