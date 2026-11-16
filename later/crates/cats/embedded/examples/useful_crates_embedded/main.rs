mod postcard_queue;

fn main() -> impl std::process::Termination {
    postcard_queue::run()
}
