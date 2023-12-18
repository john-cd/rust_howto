#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// or simply #[allow(unused)]
#![allow(dead_code)]
mod async_manager;
#[allow(missing_docs)]
mod synchronous_backend_manager;

use async_manager::AsyncManager;
use std::error::Error;
use synchronous_backend_manager::{OperationHandle, Responder, SynchronousBackendManager};
use tracing::{debug, error, event, info, trace, warn, Level};

async fn some_async_computation(i: u64) -> Result<(), Box<dyn Error>> {
    let millis = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms.", i, millis);
    tokio::time::sleep(tokio::time::Duration::from_millis(millis)).await;
    println!("Task {} stopping.", i);
    Ok(())
}

// async entrypoint
async fn run() {
    tokio::spawn(async move {
        for i in 1..=10 {
            let res = some_async_computation(i).await;
            //tx.send(res).await.unwrap();
        }
    });
}

// synchronous computations below ---------------------

fn some_sync_computation(cmd: Command, resp: &dyn Responder<String>) {
    match cmd {
        Command::Run(i) => {
            let millis = 1000 - 50 * i;
            resp.respond_with(format!("Task {} sleeping for {} ms.", i, millis));
            std::thread::sleep(std::time::Duration::from_millis(millis));
            resp.respond_with(format!("Task {} stopping.", i));
        }
        _ => resp.respond_with("Not implemented".to_string()),
    }
}

#[derive(Clone)]
enum Command {
    Run(u64),
    Other,
}

// structure the application as largely synchronous, with smaller or logically distinct asynchronous portions.
//For instance, a GUI application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.
fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let mut m = SynchronousBackendManager::create()?;

    let (id, mut handle) = m.create_operation(some_sync_computation);

    // testing on another thread
    let sync_code = std::thread::spawn(move || {
        handle.send_command(Command::Run(0));
        // while let Some(out) = handle.get_response_blocking() {
        //     info!("{out}");
        // }
        // println!("done!")
    });
    sync_code.join().unwrap();

    Ok(())
}
