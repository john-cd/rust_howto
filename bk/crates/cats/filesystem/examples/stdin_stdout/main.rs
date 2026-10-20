mod stdin_stdout;

fn main() -> anyhow::Result<()> {
    stdin_stdout::run()?;
    Ok(())
}
