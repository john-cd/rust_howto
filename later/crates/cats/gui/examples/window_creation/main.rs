mod baseview;
mod tao;
mod winit;

fn main() -> anyhow::Result<()> {
    baseview::run()?;
    tao::run();
    winit::run()?;
    Ok(())
}
