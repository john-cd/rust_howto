mod floem;
mod iced;
mod slint;
mod vizia;
mod xilem;

fn main() -> anyhow::Result<()> {
    floem::main();
    iced::main();
    // slint::main()?;
    // vizia::main();
    xilem::main();
    Ok(())
}
// [ P1 fix](https://github.com/john-cd/rust_howto/issues/1051)
