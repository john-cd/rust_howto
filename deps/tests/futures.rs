#![allow(dead_code)]

use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};

async fn task_one() {
    // ...
}
async fn task_two() {
    // ...
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("task one completed first"),
        () = t2 => println!("task two completed first"),
    }
}

#[tokio::main]
async fn main() {
    race_tasks().await;
}

#[test]
fn test() {
    main();
}
