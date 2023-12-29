use std::thread;
use std::time::Duration;

fn main() {
    let thread_one = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let thread_two = thread::spawn(|| { /* ... */ });
    // more stufff

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
