#[cfg(feature = "floem")]
mod floem;
mod iced;
mod slint;
#[cfg(feature = "vizia")]
mod vizia;
mod xilem;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "floem")]
    floem::main();
    iced::main();
    // slint::main()?;
    #[cfg(feature = "vizia")]
    vizia::main();
    xilem::main();
    Ok(())
}
// [finish fix](https://github.com/john-cd/rust_howto/issues/1051)
