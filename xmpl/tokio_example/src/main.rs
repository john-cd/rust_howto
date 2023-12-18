mod mpsc;
mod oneshot;

#[tokio::main]
async fn main() {
    oneshot::one_shot().await;
    mpsc::multi_producer_single_receiver().await;
}
