// TODO Reviewed ??-??-2026
mod tonic;
mod tonic_client;

fn main() -> anyhow::Result<()> {
    tonic::run()?;
    Ok(())
}
