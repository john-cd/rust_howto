mod baseview;
mod tao;
mod winit;

fn main() -> anyhow::Result<()> {
    // [ P1](https://github.com/john-cd/rust_howto/issues/1057)
    baseview::main()?;
    tao::main();
    winit::main();
    Ok(())
}
