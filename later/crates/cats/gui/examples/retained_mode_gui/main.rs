#[cfg(feature = "floem")]
mod floem;
#[cfg(feature = "iced")]
mod iced;
#[cfg(feature = "slint")]
mod slint;
#[cfg(feature = "vizia")]
mod vizia;
#[cfg(feature = "xilem")]
mod xilem;

#[allow(unused_imports)]
fn main() -> anyhow::Result<()> {
    #[cfg(feature = "floem")]
    floem::main();
    #[cfg(feature = "iced")]
    let _ = iced::main();
    #[cfg(feature = "slint")]
    slint::main()?;
    #[cfg(feature = "vizia")]
    vizia::main();
    #[cfg(feature = "xilem")]
    let _ = xilem::main();

    Ok(())
}
