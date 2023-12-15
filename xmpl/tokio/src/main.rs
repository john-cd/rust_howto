use tokio::sync::mpsc;
use tokio::sync::oneshot;

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}

async fn one_shot() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation(0).await;
        tx.send(res).unwrap();
        // alernatively, return the value via the joinhandle returned by `spawn`
    });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = rx.await.unwrap();
    println!("{}", res);
}

async fn multi_producer_single_receiver() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 1..=10 {
            let res = some_computation(i).await;
            tx.send(res).await.unwrap();
        }
    });

    while let Some(res) = rx.recv().await {
        println!("{}", res);
    }
}

#[tokio::main]
async fn main() {
    one_shot().await;
    multi_producer_single_receiver().await;
}
