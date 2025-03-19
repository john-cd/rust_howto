mod femtovg;
mod skia_safe;
mod vello;
mod vger;
mod webrender;

fn main() {
    // [WIP review](https://github.com/john-cd/rust_howto/issues/1047)
    femtovg::main();
    skia_safe::main();
    vello::main();
    vger::main();
    webrender::main();
}
