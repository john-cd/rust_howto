mod floem;
mod iced;
mod slint;
mod vizia;
mod xilem;

fn main() -> anyhow::Result<()> {
    floem::main();
    iced::main();
    // slint::main()?; TODO P1 fix
    vizia::main();
    xilem::main();
    Ok(())
}
