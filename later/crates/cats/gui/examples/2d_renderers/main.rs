#[cfg(feature = "femtovg")]
mod femtovg;
#[cfg(feature = "skia")]
mod skia_safe;
#[cfg(feature = "vello")]
mod vello;
#[cfg(feature = "vger")]
mod vger;
#[cfg(feature = "webrender")]
mod webrender;

#[allow(unused_imports)]
fn main() {
    #[cfg(feature = "femtovg")]
    femtovg::main();
    #[cfg(feature = "skia")]
    skia_safe::main();
    #[cfg(feature = "vello")]
    vello::main();
    #[cfg(feature = "vger")]
    vger::main();
    #[cfg(feature = "webrender")]
    webrender::main();
}
