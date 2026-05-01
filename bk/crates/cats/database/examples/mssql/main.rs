#[cfg(feature = "mssql")]
mod tiberius;

fn main() {
    #[cfg(feature = "mssql")]
    {
        let _ = tiberius::run();
    }
}
