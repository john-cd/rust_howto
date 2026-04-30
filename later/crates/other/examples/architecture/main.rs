mod cqrs;
mod di;
mod layered_architecture;
mod repository;
mod state_machine;

fn main() {
    cqrs::run();
    di::run();
    layered_architecture::run();
    repository::run();
    state_machine::run();
}
