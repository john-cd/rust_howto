#[cfg(feature = "floem")]
mod floem;
mod iced;
mod slint;
#[cfg(feature = "vizia")]
mod vizia;
#[cfg(feature = "xilem")]
mod xilem;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "floem")]
    floem::run();
    iced::run();
    // slint::run()?;
    #[cfg(feature = "vizia")]
    vizia::run();
    #[cfg(feature = "xilem")]
    xilem::run();
    Ok(())
}
// [finish fix](https://github.com/john-cd/rust_howto/issues/1051)
