mod openrr;
mod zenoh;

fn main() {
    openrr::run();
    zenoh::run();
}
