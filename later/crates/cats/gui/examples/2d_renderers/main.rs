#[cfg(feature = "femtovg")]
mod femtovg;

#[cfg(not(feature = "femtovg"))]
mod femtovg {
    pub fn run() {}
}

mod minifb;

#[cfg(feature = "skia")]
mod skia_safe;

#[cfg(not(feature = "skia"))]
mod skia_safe {
    pub fn run() {}
}

#[cfg(feature = "vello")]
mod vello;

#[cfg(not(feature = "vello"))]
mod vello {
    pub fn run() {}
}

mod vger;

#[cfg(feature = "webrender")]
mod webrender;

#[cfg(not(feature = "webrender"))]
mod webrender {
    pub fn run() {}
}

fn main() {
    // [review](https://github.com/john-cd/rust_howto/issues/1047).
    minifb::run();
    femtovg::run();
    skia_safe::run();
    vello::run();
    vger::run();
    webrender::run();
}
