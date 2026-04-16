//! Adapted from <https://dystroy.org/btracing::info!/cancelable-background-task/>.

use cancelable::*;

fn main() {
    tracing_subscriber::fmt()
        // Sets the maximum verbosity level.
        .with_max_level(tracing::Level::DEBUG)
        // Sets this subscriber to be the global trace collector for this application.
        .init();

    let worker = Worker::new();
    let mut tasks = Vec::new();
    for _ in 0..10 {
        tasks.push(Task::new());
    }
    for task in tasks {
        worker.start(task);
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
    worker.process_responses();
}
