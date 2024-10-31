use tokio::sync::mpsc;

async fn some_computation(input: u32) -> String {
    format!("the result of computation is {}", input)
}

pub async fn multi_producer_single_receiver() {
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
    multi_producer_single_receiver().await;
}

#[test]
fn test() {
    main();
}
