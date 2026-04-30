#[cfg(feature = "candle")]
mod candle;
mod linfa;
mod smartcore;

fn main() {
    candle::run();
    linfa::run();
    smartcore::run();
}
