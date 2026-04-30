#[cfg(feature = "gtk")]
mod gtk4;
#[cfg(feature = "gtk")]
mod relm4;

#[allow(unused_imports)]
fn main() {
    #[cfg(feature = "gtk")]
    {
        gtk4::main();
        relm4::main();
    }
}
