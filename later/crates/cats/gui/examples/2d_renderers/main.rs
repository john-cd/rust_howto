#[cfg(feature = "femtovg")]
mod femtovg;

#[cfg(not(feature = "femtovg"))]
mod femtovg {
    pub fn main() {}
}

mod minifb;

#[cfg(feature = "skia")]
mod skia_safe;

#[cfg(not(feature = "skia"))]
mod skia_safe {
    pub fn main() {}
}

#[cfg(feature = "vello")]
mod vello;

#[cfg(not(feature = "vello"))]
mod vello {
    pub fn main() {}
}

mod vger;

#[cfg(feature = "webrender")]
mod webrender;

#[cfg(not(feature = "webrender"))]
mod webrender {
    pub fn main() {}
}

fn main() {
    // [review](https://github.com/john-cd/rust_howto/issues/1047).
    minifb::main();
    femtovg::main();
    skia_safe::main();
    vello::main();
    vger::main();
    webrender::main();
}
