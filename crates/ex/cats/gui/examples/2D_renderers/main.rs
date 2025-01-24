mod femtovg;
mod skia_safe;
mod vello;
mod vger;
mod webrender;

fn main() {
    // TODO P1
    femtovg::main();
    skia_safe::main();
    vello::main();
    vger::main();
    webrender::main();
}
