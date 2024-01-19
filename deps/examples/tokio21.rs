use std::rc::Rc;

use tokio::task;
use tokio::time;

#[tokio::main]
async fn main() {
    let nonsend_data = Rc::new("world");
    let local = task::LocalSet::new();

    let nonsend_data2 = nonsend_data.clone();
    local.spawn_local(async move {
        // ...
        println!("hello {}", nonsend_data2)
    });

    local.spawn_local(async move {
        time::sleep(time::Duration::from_millis(100)).await;
        println!("goodbye {}", nonsend_data)
    });

    // ...

    local.await;
}
