#[cfg(all(target_os = "linux", feature = "opencv"))]
mod opencv;

fn main() {
    #[cfg(all(target_os = "linux", feature = "opencv"))]
    {
        opencv::run();
    }
}
