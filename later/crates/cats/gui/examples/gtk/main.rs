#[cfg(feature = "gtk")]
mod gtk4;

#[cfg(feature = "gtk")]
mod relm4;

fn main() {
    #[cfg(feature = "gtk")]
    gtk4::main();

    #[cfg(feature = "gtk")]
    relm4::main();
}
