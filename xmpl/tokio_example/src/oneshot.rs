use tokio::sync::oneshot;

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}

pub async fn one_shot() {
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
