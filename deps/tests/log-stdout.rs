use env_logger::Builder;
use env_logger::Target;

#[test]
fn test() {
    Builder::new().target(Target::Stdout).init();

    log::error!("This error has been printed to Stdout");
}
