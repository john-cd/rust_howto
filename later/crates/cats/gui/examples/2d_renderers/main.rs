mod femtovg;
mod skia_safe;
mod vello;
mod vger;
mod webrender;

fn main() {
    println!("Running 2D renderer examples...");
    femtovg::main();
    skia_safe::main();
    vello::main();
    vger::main();
    webrender::main();
}
