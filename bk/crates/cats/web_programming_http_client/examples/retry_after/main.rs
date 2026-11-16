mod retry_after_header;

fn main() -> impl std::process::Termination {
    retry_after_header::run()
}
