#[path = "../../toasty.rs"]
mod toasty;

fn main() -> anyhow::Result<()> {
    toasty::run()
}
