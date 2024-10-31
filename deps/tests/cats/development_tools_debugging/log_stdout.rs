use env_logger::Builder;
use env_logger::Target;

fn main() {
    Builder::new().target(Target::Stdout).init();

    log::error!("This error has been printed to Stdout");
}

#[test]
fn test() {
    main();
}
