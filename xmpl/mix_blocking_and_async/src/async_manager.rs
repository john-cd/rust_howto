use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub struct AsyncManager {
    rt: tokio::runtime::Runtime,
}

impl AsyncManager {
    // `enable_all()` enables the IO and timer drivers on the Tokio runtime. If they are not enabled, the runtime is unable to perform IO or timers
    fn build_current_thread_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    }

    // equivalent to #[tokio::main] async fn main()
    fn build_multi_thread_runtime(n: usize) -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(n)
            .thread_name_fn(|| {
                static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
                let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
                format!("tokio-runtime-worker-{}", id)
            })
            .enable_all()
            .build()
            .unwrap()
    }

    pub fn new() -> Self {
        let rt = Self::build_multi_thread_runtime(2);
        Self { rt }
    }
}

// https://ryhl.io/blog/actors-with-tokio/

// Set up a channel for communicating.
//let (tx, mut rx) = mpsc::channel(16);

// std::thread::spawn(move || {
//     rt.block_on(async move {
//         while let Some(task) = rx.recv().await {
//             //tokio::spawn(handle_task(task));
//         }

//         // Once all senders have gone out of scope,
//         // the `.recv()` call returns None and it will
//         // exit from the while loop and shut down the
//         // thread.
//     });
// });
