mod baseview;
mod tao;
mod winit;

fn main() -> anyhow::Result<()> {
    baseview::main()?;
    tao::main();
    winit::main()?;
    Ok(())
}
