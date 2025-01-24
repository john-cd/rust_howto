mod baseview;
mod tao;
mod winit;

fn main() -> anyhow::Result<()> {
    // TODO P1
    baseview::main()?;
    tao::main();
    winit::main();
    Ok(())
}
