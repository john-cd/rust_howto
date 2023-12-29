mod mpsc;

#[tokio::main]
async fn main() {
    mpsc::multi_producer_single_receiver().await;
}
